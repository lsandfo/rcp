use std::fs::write;
use std::{io::Read, net};

pub struct Server {
    listener: net::TcpListener,
    data_name: Option<String>,
    data: Option<Vec<u8>>,
}

impl Server {
    /// new retruned a new Server Instance, which acts as listener to receive data.
    /// It tages the name of the to recevied file as argument and bind itself to Port `0.0.0.0:9080` via TCP.
    pub fn new(dn: String) -> Result<Server, Box<dyn std::error::Error>> {
        Ok(Server {
            listener: net::TcpListener::bind("0.0.0.0:9080")?,
            data_name: Some(dn),
            data: None,
        })
    }

    /// listen start's the TCP listener and wait for a new connection.
    /// It will read everything the client will given and save it in the `data` field.
    pub fn listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf_data: Vec<u8> = Vec::new();
        let mut client = self.listener.incoming().next().unwrap()?;
        client.read_to_end(&mut buf_data).unwrap();
        self.data = Some(buf_data);
        Ok(())
    }

    /// save will save the connent of the `data` field in a file named by `data_name` field.
    pub fn save(&mut self) -> Result<(), std::io::Error> {
        write(self.data_name.clone().unwrap(), self.data.clone().unwrap())?;
        Ok(())
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    use std::fs::read_to_string;

    static SAVEFILE: &str = "received";
    #[test]
    fn test_lister() {
        let mut test_server = Server::new(SAVEFILE.to_string()).unwrap();
        test_server.listen().unwrap();
        assert_eq!(
            "Hello World",
            String::from_utf8(test_server.data.clone().unwrap()).unwrap()
        );
        test_server.save().unwrap();
        assert_eq!("Hello World", read_to_string(SAVEFILE).unwrap());
        std::fs::remove_file(SAVEFILE).unwrap();
    }
}
