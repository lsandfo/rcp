use std::fs::write;
use std::{io::Read, net};

static SAVEFILE: &str = "received";
pub struct Server {
    listener: net::TcpListener,
    //data_name: Option<String>,
    data: Option<Vec<u8>>,
}

impl Server {
    // Aktuell startet new gleich das binding auf den Port 9080 auf 0.0.0.0
    pub fn new() -> Result<Server, Box<dyn std::error::Error>> {
        Ok(Server {
            listener: net::TcpListener::bind("0.0.0.0:9080")?,
            //data_name: None,
            data: None,
        })
    }

    // listen wartet bis eine connection statt gefunden hat und nimmt den 
    // inhalt der Datei an. 
    pub fn listen(&mut self) {
        //let mut buf_name = String::new();
        let mut buf_data: Vec<u8> = Vec::new();
        let mut client = self.listener.incoming().next().unwrap().unwrap();
        //client.read_to_string(&mut buf_name).unwrap();
        //self.data_name = Some(buf_name);
        client.read_to_end(&mut buf_data).unwrap();
        self.data = Some(buf_data);
    }

    // Speichert self.data in SAVEFIEL
    pub fn save(&mut self) {
        write(SAVEFILE, self.data.clone().unwrap()).expect("Can't save received file!");
    }
}

#[cfg(test)]
mod testing {

use std::fs::read_to_string;
use super::*;

    #[test]
    fn test_lister() {
        let mut test_server = Server::new().unwrap();
        test_server.listen();
        assert_eq!(
            "Hello World",
            String::from_utf8(test_server.data.clone().unwrap()).unwrap()
        );
        test_server.save();
        assert_eq!("Hello World", read_to_string(SAVEFILE).unwrap());
        std::fs::remove_file(SAVEFILE).unwrap();
    }
}
