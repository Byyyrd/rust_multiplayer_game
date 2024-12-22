use std::{
    io::{stdin, ErrorKind, Read, Write},
    net::{SocketAddr, TcpStream},
    str::FromStr,
    thread,
};
//pub static ADDR: &str = "127.0.0.1:7070";
pub fn start(address: &str) -> std::io::Result<()> {
    let ip_address = SocketAddr::from_str(address).expect("Invalid Ip address");
    let stream = TcpStream::connect(ip_address).expect("Verbindung zum Server fehlgeschlagen.");
    let stream_clone = stream
        .try_clone()
        .expect("Klonen des Streams fehlgeschlagen.");

    thread::spawn(move || {
        read_data(stream_clone);
    });

    write_input(stream)
}
fn write_input(mut stream: TcpStream) -> std::io::Result<()> {
    let mut input_text = String::new();
    let msg = "Hello, my name is client!\r\n";
    println!("{}", msg);
    stream.write_all(msg.as_bytes()).unwrap();

    loop {
        println!("Bitte Text eingeben: ");
        input_text.clear();
        stdin()
            .read_line(&mut input_text)
            .expect("Fehler beim Lesen der Eingabe.");

        if input_text.trim().is_empty() {
            println!("Leere Eingabe erkannt. Bitte erneut versuchen.");
            continue;
        }

        stream.write_all(input_text.as_bytes()).unwrap();
        stream.flush()?;
    }
}
fn read_data(mut stream: TcpStream) {
    loop {
        let mut buffer = [0 as u8; 512]; // using 512 byte buffer
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Server hat die Verbindung geschlossen
                println!("Server hat die Verbindung geschlossen.");
                break;
            }
            Ok(n) => {
                let received = String::from_utf8_lossy(&buffer[..n]);
                println!("Empfangen: {}", received);
            }
            Err(e) => {
                if e.kind() == ErrorKind::ConnectionReset {
                    println!("Verbindung zum Server verloren.");
                    break;
                } else {
                    eprintln!("Fehler beim Lesen der Daten: {}", e);
                }
            }
        }
    }
}
