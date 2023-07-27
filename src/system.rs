use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    ops::Mul,
    sync::{mpsc::Receiver, Arc, Mutex, MutexGuard},
    time::Instant,
};

use glium::{
    backend::glutin::glutin,
    glutin::{
        event::{self, Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    Display, Surface,
};
use imgui::{Context, FontConfig, FontSource, ProgressBar, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::WinitPlatform;

use crate::packet::Packet;

pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
}

pub fn init() -> System {
    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = WindowBuilder::new()
        .with_title("Gran Turismo 7 Telemetry Viewer")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024i32, 768i32));
    let display = Display::new(builder, context, &event_loop).expect("failed to init display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(
            imgui.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Locked(2f64),
        );
    }

    let font_size = 16.0;

    imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../resources/FOT-NewCezanne Pro DB.otf"),
        size_pixels: font_size,
        config: Some(FontConfig {
            rasterizer_multiply: 1.5,
            oversample_h: 4,
            oversample_v: 4,
            ..FontConfig::default()
        }),
    }]);

    let renderer = Renderer::init(&mut imgui, &display).expect("Failed to init renderer");
    System {
        event_loop,
        display,
        imgui,
        platform,
        renderer,
        font_size,
    }
}

impl System {
    pub fn run(self, channel_rx: Receiver<Packet>) {
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;
        let mut last_frame = Instant::now();
        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();

                // .scale *= 2f32;

                if let Some(packet) = Self::get_received_data(&channel_rx) {
                    Self::update_ui(&ui, packet);
                }

                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                platform.prepare_render(ui, gl_window.window());
                let draw_data = imgui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        });
    }

    fn get_received_data(rx: &Receiver<Packet>) -> Option<Packet> {
        rx.iter().next()
    }

    fn update_ui(ui: &Ui, data: Packet) {
        ui.window("data").build(|| {
            ProgressBar::new(data.throttle as f32 / 255f32).build(ui);
            ProgressBar::new(data.brake as f32 / 255f32).build(ui);
            ui.text_colored(
                [255f32, 0f32, 0f32, 255f32],
                format!("{}", data.current_gear),
            );
        });
    }
}
