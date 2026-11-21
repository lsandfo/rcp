use rcp::server;
use rcp::client;
use std::fs;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        let listener = server::Server::new();
        match listener {
            Ok(mut server) => {
                println!("Warte auf Verbindung...");
                server.listen().expect("Fehler beim empfangen der Daten!");
                println!("Datei empfangen!");
                server.save().expect("Date konnte nicht gespeichert werden");
                println!("Datei gespeichert");
            },
            Err(srv_err) => {
                println!("Listener konnte nicht gestartet werden!\n{srv_err}");
                exit(1);
            }
        }
    } else if args.len() == 2 && args[1] == "-h" {
        println!("Sende Modus: IP/FQDN Dateipfad\nOhne Argumente: Empfänger Modus");
        exit(1);
    } else if args.len() >= 3 {
        let target_ip: String = args[1].clone(); //expect("Ziel Ip als erstes Argument benötigt");
        let data_file: String = args[2].clone(); //expect("Zeildatei als 2tes Argument benötigt");
        if fs::exists(&data_file).unwrap(){
            println!("Verbinde zum Ziel...");
            let mut sender = client::Client::new(target_ip, &data_file.as_str()).expect("Ziel System nicht erreichbar!");
            println!("Sende Datei...");
            sender.send();
            println!("Datei erfolgreich gesendet!");
        } else {
            println!("Zieldatei '{data_file}' nicht gefunden!");
            exit(1);
        }
    }
}
