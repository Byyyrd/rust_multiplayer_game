use std::{
    io::{ErrorKind, Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, str::FromStr, sync::{mpsc::{Receiver, TryRecvError}, Arc, Mutex}, thread
};
//pub static ADDR: &str = "127.0.0.1:7070";
pub fn start(address: &str,reciever: Receiver<()>) {
    let ip_address = SocketAddr::from_str(address).expect("Invalid Ip address");
    let listener = TcpListener::bind(ip_address).expect("Server konnte nicht starten.");
    println!("Server gestartet auf {}", address);
    let a_receiver: Arc<Mutex<Receiver<()>>> = Arc::new(Mutex::new(reciever));
    for stream in listener.incoming() {
        let receiver_clone = a_receiver.clone();
        match receiver_clone.lock().unwrap().try_recv(){
            Ok(_) | Err(TryRecvError::Disconnected)=>{break;}
            Err(TryRecvError::Empty) => {}
        }
        match stream {
            Ok(stream) => {
                println!("Neuer Client verbunden: {:?}", stream.peer_addr());
                thread::spawn(|| {
                    handle_client(stream,receiver_clone);
                });
            }
            Err(e) => {
                eprintln!("Fehler beim Akzeptieren eines Clients: {}", e);
            }
        }
    }
}
fn handle_client(mut stream: TcpStream,receiver: Arc<Mutex<Receiver<()>>>) {
    let client_address = stream.peer_addr().unwrap();
    println!("Verbunden mit {}", client_address);
    let mut buffer = [0u8; 512]; // 512-Byte-Puffer für eingehende Nachrichten

    loop {
        match receiver.lock().unwrap().try_recv(){
            Ok(_) | Err(TryRecvError::Disconnected)=>{break;}
            Err(TryRecvError::Empty) => {}
        }
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Verbindung geschlossen
                println!("Client {} hat die Verbindung geschlossen.", client_address);
                break;
            }
            Ok(n) => {
                let received = String::from_utf8_lossy(&buffer[..n]);
                println!("Empfangen von {}: {}", client_address, received);

                // Antwort senden
                let response: &str;
                if received.trim() == "Hello" {
                    response = "Hello, Client!\n";
                } else {
                    response = "Unbekannte Nachricht\n";
                }
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // Keine Daten verfügbar
                continue;
            }
            Err(ref e) if e.kind() == ErrorKind::ConnectionReset => {
                println!("Verbindung von {} wurde zurückgesetzt.", client_address);
                break;
            }
            Err(e) => {
                eprintln!("Fehler beim Lesen von {}: {}", client_address, e);
                break;
            }
        }
    }

    println!("Beende Verbindung mit {}", client_address);
}
