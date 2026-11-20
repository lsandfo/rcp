use std::fs;
use std::io::Write;
use std::net;

pub struct Client {
    pub listener: net::TcpStream,
    //data_name: Option<String>,
    data: Option<Vec<u8>>,
}

impl Client {
    // new nimmt die Ziel IP, den path zu dem zu versendenen File, liest es als u8 und speichert es im data Feld 
    pub fn new(
        ip: String, /*, data_name: String*/
        file: &str,
    ) -> Result<Client, Box<dyn std::error::Error>> {
        Ok(Client {
            listener: net::TcpStream::connect(format!("{ip}:9080"))?,
            data: Some(fs::read(file)?),
            //data_name: Some(data_name),
        })
    }

    // send versucht das data feld in einem rutsch in den Socket zu schreiben
    pub fn send(&mut self) {
        //self.listener.write_all(self.data_name.clone().unwrap().as_bytes()).unwrap();
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
