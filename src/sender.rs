use std::fs;
use std::io::Write;
use std::net;

/// Client contains the listener, which contains a connection to the target, as TCPStream and the file to send in the data field as `Vec<u8>`
pub struct Client {
    pub listener: net::TcpStream,
    //data_name: Option<String>,
    data: Option<Vec<u8>>,
}
/// new takes the target IP and path to the file to be sended. It reads the file as `Vec<u8>` in the data field.
/// It als creates a new connectio to the target and failed to create a Instance if the connection failed.
impl Client {
    pub fn new(ip: String, file: &str) -> Result<Client, Box<dyn std::error::Error>> {
        Ok(Client {
            listener: net::TcpStream::connect(format!("{ip}:9080"))?,
            data: Some(fs::read(file)?),
        })
    }

    /// send try to send the `data` field to the target.
    pub fn send(&mut self) {
        self.listener
            .write_all(self.data.clone().unwrap().as_ref())
            .unwrap();
        self.listener.flush().expect("Can't send all data.");
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_send() {
        fs::write("testfile.txt".to_string(), "Hello World".as_bytes()).unwrap();
        let mut client = Client::new("localhost".to_string(), "testfile.txt").unwrap();
        client.send();
        fs::remove_file("testfile.txt").unwrap();
    }
}
