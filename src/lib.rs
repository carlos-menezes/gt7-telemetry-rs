mod client;
mod crypt;
mod packet;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::client::Client;

    #[test]
    fn it_works() {
        let client = Client::new("192.168.1.154").unwrap();
        client.start().unwrap()
    }
}
