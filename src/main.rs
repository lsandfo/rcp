use rcp::listener;
use rcp::sender;
use std::fs;

use std::env;
use std::process::exit;

// main reads the given arguments. If the first arg doesn't start with an '-', rcp starts in Listener Mode and waits for an connection.
// It writes the received data in a file, named by what ever in args[1] stands.
// If you give rcp as args an IP or FQDN as first arg and a file name as second, it will try to send the file to that host.
// You can call the help with the "-h" flag.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let listener = listener::Server::new();
        match listener {
            Ok(mut server) => {
                println!("Wait for connection...");
                server.listen().expect("Error while receive data!");
                println!("Data received!");
                server.save().expect("Data can't saved to File!");
                println!("Data saved to file successfully");
            }
            Err(srv_err) => {
                println!("Can't start listener!\n{srv_err}");
                exit(1);
            }
        }
    } else if args.len() == 2 && args[1] == "-h" {
        println!("Send Mode: IP/FQDN Filepath\nFilename: Listener Mode");
        exit(1);
    } else if args.len() == 2 && args[1] == "-v" {
        let version: &str = env!("CARGO_PKG_VERSION");
        println!("rcp Version: {version}");
    } else if args.len() >= 3 {
        let target_ip: String = args[1].clone(); //expect("Ziel Ip als erstes Argument benötigt");
        let data_file: String = args[2].clone(); //expect("Zeildatei als 2tes Argument benötigt");
        if fs::exists(&data_file).unwrap() {
            println!("Connect to Listener...");
            let mut sender = sender::Client::new(target_ip, data_file.as_str())
                .expect("Target System not reachable!");
            println!("Send Data...");
            sender.send().unwrap();
            println!("Data sended successfully!");
        } else {
            println!("Target File '{data_file}' not found!");
            exit(1);
        }
    }
}
