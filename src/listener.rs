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
    pub fn new() -> Result<Server, Box<dyn std::error::Error>> {
        Ok(Server {
            listener: net::TcpListener::bind("0.0.0.0:9080")?,
            data_name: None,
            data: None,
        })
    }

    /// listen start's the TCP listener and wait for a new connection.
    /// It will read everything the client will given and save it in the `data` field.
    pub fn listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf_data: Vec<u8> = Vec::new();
        let mut client = self.listener.incoming().next().unwrap()?;
        let mut size_buf = [0u8; 4];
        client.read_exact(&mut size_buf).unwrap();
        let name_len = u32::from_be_bytes(size_buf) as usize;
        let mut name_buf = vec![0u8; name_len];
        client.read_exact(&mut name_buf).unwrap();
        client.read_to_end(&mut buf_data).unwrap();
        self.data = Some(buf_data);
        self.data_name = Some(String::from_utf8(name_buf).unwrap());
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
    use std::fs::{read_to_string, remove_file};

    #[test]
    fn test_lister() {
        let mut test_server = Server::new().unwrap();
        test_server.listen().unwrap();
        assert_eq!(
            "Hello World",
            String::from_utf8(test_server.data.clone().unwrap()).unwrap()
        );
        assert_eq!("testfile.txt", test_server.data_name.clone().unwrap());
        test_server.save().unwrap();
        assert_eq!("Hello World", read_to_string("testfile.txt").unwrap());
        remove_file("testfile.txt").unwrap();
    }
}
