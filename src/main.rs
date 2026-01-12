use std::net::TcpStream;
use std::time::Duration;

fn main() {
    let timeout = Duration::from_secs(1);

    let port = [1, 2, 3, 4, 53];
    let mut i = 0;

    while i < port.len() {
        let address = format!("192.168.1.1:{}", port[i]);
        match TcpStream::connect_timeout(&address.parse().unwrap(), timeout) {
            Ok(_) => println!("Le port {} est OUVERT", port[i]),
            Err(_) => println!("Le port {} est FERMÉ ou filtré", port[i]),
        }
        i += 1;
    }
    println!("Fin du scan");
}
