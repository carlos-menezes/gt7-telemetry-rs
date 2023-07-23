use byteorder::{LittleEndian, ReadBytesExt};
use std::{io::Cursor, mem};

use crate::{crypt::MAGIC_VALUE, errors::PacketError};

pub const PACKET_SIZE: usize = 0x198;
pub const HEARTBEAT_PACKET_DATA: &[u8; 1] = b"A";

pub struct Packet {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub rotation: [f32; 3],
    pub relative_orientation_to_north: f32,
    pub angular_velocity: [f32; 3],
    pub body_height: f32,
    pub engine_rpm: f32,
    pub gas_level: f32,
    pub gas_capacity: f32,
    pub meters_per_second: f32,
    pub turbo_boost: f32,
    pub oil_pressure: f32,
    pub water_temperature: f32,
    pub oil_temperature: f32,
    pub tire_fl_surface_temperature: f32,
    pub tire_fr_surface_temperature: f32,
    pub tire_rl_surface_temperature: f32,
    pub tire_rr_surface_temperature: f32,
    pub packet_id: i32,
    pub lap_count: i16,
    pub laps_in_race: i16,
    pub best_lap_time: i32,
    pub last_lap_time: i32,
    pub time_of_day_progression: i32,
    pub qualifying_position: i16,
    pub num_cars_pre_race: i16,
    pub alert_rpm_min: i16,
    pub alert_rpm_max: i16,
    pub calculated_max_speed: i16,
    pub flags: Flags,
    pub current_gear: u8,
    pub suggested_gear: u8,
    pub throttle: u8,
    pub brake: u8,
    pub road_plane: [f32; 3],
    pub road_plane_distance: f32,
    pub wheel_fl_rps: f32,
    pub wheel_fr_rps: f32,
    pub wheel_rl_rps: f32,
    pub wheel_rr_rps: f32,
    pub tire_fl_radius: f32,
    pub tire_fr_radius: f32,
    pub tire_rl_radius: f32,
    pub tire_rr_radius: f32,
    pub tire_fl_suspension_height: f32,
    pub tire_fr_suspension_height: f32,
    pub tire_rl_suspension_height: f32,
    pub tire_rr_suspension_height: f32,
    pub clutch_pedal: f32,
    pub clutch_engagement: f32,
    pub rpm_from_clutch_to_gearbox: f32,
    pub transmission_top_speed: f32,
    pub gear_ratios: [f32; 7],
    pub car_code: i32,
}

impl Packet {
    fn parse(packet: [u8; PACKET_SIZE]) -> Result<Self, PacketError> {
        let mut cursor = Cursor::new(packet);
        let magic = cursor.read_u32::<LittleEndian>()?;
        if magic != MAGIC_VALUE {
            return Err(PacketError::UnexpectedMagicValue());
        }

        let position: [f32; 3] = [
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
        ];
        let velocity: [f32; 3] = [
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
        ];
        let rotation: [f32; 3] = [
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
        ];
        let relative_orientation_to_north = cursor.read_f32::<LittleEndian>()?;
        let angular_velocity = [
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
        ];
        let body_height = cursor.read_f32::<LittleEndian>()?;
        let engine_rpm = cursor.read_f32::<LittleEndian>()?;

        // Skip IV
        cursor.set_position(cursor.position() + (mem::size_of::<usize>() as u64));

        let gas_level = cursor.read_f32::<LittleEndian>()?;
        let gas_capacity = cursor.read_f32::<LittleEndian>()?;
        let meters_per_second = cursor.read_f32::<LittleEndian>()?;
        let turbo_boost = cursor.read_f32::<LittleEndian>()?;
        let oil_pressure = cursor.read_f32::<LittleEndian>()?;
        let water_temperature = cursor.read_f32::<LittleEndian>()?;
        let oil_temperature = cursor.read_f32::<LittleEndian>()?;
        let tire_fl_surface_temperature = cursor.read_f32::<LittleEndian>()?;
        let tire_fr_surface_temperature = cursor.read_f32::<LittleEndian>()?;
        let tire_rl_surface_temperature = cursor.read_f32::<LittleEndian>()?;
        let tire_rr_surface_temperature = cursor.read_f32::<LittleEndian>()?;
        let packet_id = cursor.read_i32::<LittleEndian>()?;
        let lap_count = cursor.read_i16::<LittleEndian>()?;
        let laps_in_race = cursor.read_i16::<LittleEndian>()?;
        let best_lap_time = cursor.read_i32::<LittleEndian>()?;
        let last_lap_time = cursor.read_i32::<LittleEndian>()?;
        let time_of_day_progression = cursor.read_i32::<LittleEndian>()?;
        let qualifying_position = cursor.read_i16::<LittleEndian>()?;
        let num_cars_pre_race = cursor.read_i16::<LittleEndian>()?;
        let alert_rpm_min = cursor.read_i16::<LittleEndian>()?;
        let alert_rpm_max = cursor.read_i16::<LittleEndian>()?;
        let calculated_max_speed = cursor.read_i16::<LittleEndian>()?;
        let flags = Flags::try_from(cursor.read_i16::<LittleEndian>()?)?;

        let bits = cursor.read_u8()?;
        let current_gear = bits & 0b1111;
        let suggested_gear = bits >> 4;

        let throttle = cursor.read_u8()?;
        let brake = cursor.read_u8()?;

        // Skip an unused byte
        cursor.read_u8()?;

        let road_plane = [
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
        ];

        let road_plane_distance = cursor.read_f32::<LittleEndian>()?;

        let wheel_fl_rps = cursor.read_f32::<LittleEndian>()?;
        let wheel_fr_rps = cursor.read_f32::<LittleEndian>()?;
        let wheel_rl_rps = cursor.read_f32::<LittleEndian>()?;
        let wheel_rr_rps = cursor.read_f32::<LittleEndian>()?;
        let tire_fl_radius = cursor.read_f32::<LittleEndian>()?;
        let tire_fr_radius = cursor.read_f32::<LittleEndian>()?;
        let tire_rl_radius = cursor.read_f32::<LittleEndian>()?;
        let tire_rr_radius = cursor.read_f32::<LittleEndian>()?;
        let tire_fl_suspension_height = cursor.read_f32::<LittleEndian>()?;
        let tire_fr_suspension_height = cursor.read_f32::<LittleEndian>()?;
        let tire_rl_suspension_height = cursor.read_f32::<LittleEndian>()?;
        let tire_rr_suspension_height = cursor.read_f32::<LittleEndian>()?;

        cursor.set_position(cursor.position() + (mem::size_of::<usize>() as u64 * 8));
        let clutch_pedal = cursor.read_f32::<LittleEndian>()?;
        let clutch_engagement = cursor.read_f32::<LittleEndian>()?;
        let rpm_from_clutch_to_gearbox = cursor.read_f32::<LittleEndian>()?;
        let transmission_top_speed = cursor.read_f32::<LittleEndian>()?;
        // There is an eight gear which the game overrides without bound checking.
        // For cars with more than 7 gears (e.g. LC500), the `car_code` is overriden.

        let mut gear_ratios: [f32; 7] = [0f32; 7];
        for i in 0..7 {
            gear_ratios[i] = cursor.read_f32::<LittleEndian>()?;
        }

        // Skip 8th gear
        cursor.read_f32::<LittleEndian>()?;
        let car_code = cursor.read_i32::<LittleEndian>()?;

        Ok(Self {
            position,
            velocity,
            rotation,
            relative_orientation_to_north,
            angular_velocity,
            body_height,
            engine_rpm,
            gas_level,
            gas_capacity,
            meters_per_second,
            turbo_boost,
            oil_pressure,
            water_temperature,
            oil_temperature,
            tire_fl_surface_temperature,
            tire_fr_surface_temperature,
            tire_rl_surface_temperature,
            tire_rr_surface_temperature,
            packet_id,
            lap_count,
            laps_in_race,
            best_lap_time,
            last_lap_time,
            time_of_day_progression,
            qualifying_position,
            num_cars_pre_race,
            alert_rpm_min,
            alert_rpm_max,
            calculated_max_speed,
            flags,
            current_gear,
            suggested_gear,
            brake,
            throttle,
            road_plane,
            road_plane_distance,
            wheel_fl_rps,
            wheel_fr_rps,
            wheel_rl_rps,
            wheel_rr_rps,
            tire_fl_radius,
            tire_fr_radius,
            tire_rl_radius,
            tire_rr_radius,
            tire_fl_suspension_height,
            tire_fr_suspension_height,
            tire_rl_suspension_height,
            tire_rr_suspension_height,
            clutch_pedal,
            clutch_engagement,
            rpm_from_clutch_to_gearbox,
            transmission_top_speed,
            gear_ratios,
            car_code,
        })
    }
}

#[repr(i16)]
pub enum Flags {
    None = 0,
    CarOnTrack = 1 << 0,
    Paused = 1 << 1,
    LoadingOrProcessing = 1 << 2,
    InGear = 1 << 3,
    HasTurbo = 1 << 4,
    RevLimiterBlinkAlertActive = 1 << 5,
    HandBrakeActive = 1 << 6,
    LightsActive = 1 << 7,
    HighBeamActive = 1 << 8,
    LowBeamActive = 1 << 9,
    ASMActive = 1 << 10,
    TCSActive = 1 << 11,
}

impl TryFrom<i16> for Flags {
    type Error = PacketError;

    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            x if x == Flags::None as i16 => Ok(Flags::None),
            x if x == Flags::CarOnTrack as i16 => Ok(Flags::CarOnTrack),
            x if x == Flags::Paused as i16 => Ok(Flags::Paused),
            x if x == Flags::LoadingOrProcessing as i16 => Ok(Flags::LoadingOrProcessing),
            x if x == Flags::InGear as i16 => Ok(Flags::InGear),
            x if x == Flags::HasTurbo as i16 => Ok(Flags::HasTurbo),
            x if x == Flags::RevLimiterBlinkAlertActive as i16 => {
                Ok(Flags::RevLimiterBlinkAlertActive)
            }
            x if x == Flags::HandBrakeActive as i16 => Ok(Flags::HandBrakeActive),
            x if x == Flags::LightsActive as i16 => Ok(Flags::LightsActive),
            x if x == Flags::HighBeamActive as i16 => Ok(Flags::HighBeamActive),
            x if x == Flags::LowBeamActive as i16 => Ok(Flags::LowBeamActive),
            x if x == Flags::ASMActive as i16 => Ok(Flags::ASMActive),
            x if x == Flags::TCSActive as i16 => Ok(Flags::TCSActive),
            _ => Err(PacketError::UnknownFlag()),
        }
    }
}
