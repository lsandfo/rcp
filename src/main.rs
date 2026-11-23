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
    if args.len() == 2 && !args[1].starts_with('-') {
        let listener = listener::Server::new(args[1].to_string());
        match listener {
            Ok(mut server) => {
                println!("Warte auf Verbindung...");
                server.listen().expect("Fehler beim empfangen der Daten!");
                println!("Datei empfangen!");
                server.save().expect("Date konnte nicht gespeichert werden");
                println!("Datei gespeichert");
            }
            Err(srv_err) => {
                println!("Listener konnte nicht gestartet werden!\n{srv_err}");
                exit(1);
            }
        }
    } else if args.len() == 2 && args[1] == "-h" {
        println!("Sende Modus: IP/FQDN Dateipfad\nDateiname: Empfänger Modus");
        exit(1);
    } else if args.len() == 2 && args[1] == "-v" {
        let version: &str = env!("CARGO_PKG_VERSION");
        println!("rcp Version: {version}");
    } else if args.len() >= 3 {
        let target_ip: String = args[1].clone(); //expect("Ziel Ip als erstes Argument benötigt");
        let data_file: String = args[2].clone(); //expect("Zeildatei als 2tes Argument benötigt");
        if fs::exists(&data_file).unwrap() {
            println!("Verbinde zum Ziel...");
            let mut sender = sender::Client::new(target_ip, data_file.as_str())
                .expect("Ziel System nicht erreichbar!");
            println!("Sende Datei...");
            sender.send();
            println!("Datei erfolgreich gesendet!");
        } else {
            println!("Zieldatei '{data_file}' nicht gefunden!");
            exit(1);
        }
    }
}
