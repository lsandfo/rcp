use std::io::Write;
use std::net;

pub struct Client {
    pub listener: net::TcpStream,
    //data_name: Option<String>,
    data: Option<Vec<u8>>,
}

impl Client {
    pub fn new(
        ip: String, /*, data_name: String*/
        data: Vec<u8>,
    ) -> Result<Client, Box<dyn std::error::Error>> {
        Ok(Client {
            listener: net::TcpStream::connect(format!("{ip}:9080"))?,
            //data_name: Some(data_name),
            data: Some(data),
        })
    }

    pub fn send(&mut self) {
        //self.listener.write_all(self.data_name.clone().unwrap().as_bytes()).unwrap();
        self.listener
            .write_all(self.data.clone().unwrap().as_ref())
            .unwrap();
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_send() {
        let mut client = Client::new(
            "localhost".to_string(),
            "Hello World".to_string().as_bytes().to_vec(),
        )
        .unwrap();
        client.send();
    }
}
