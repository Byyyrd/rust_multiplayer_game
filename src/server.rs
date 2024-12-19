use std::io::Write;
use std::{
    io::{BufRead, BufReader, ErrorKind},
    net::{TcpListener, TcpStream},
    thread,
};
pub static ADDR: &str = "127.0.0.1:7070";
pub fn start() -> TcpListener {
    let listener = TcpListener::bind(ADDR).unwrap();
    println!("Server startet");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Neuer Client");
        thread::spawn(|| {
            handle_client(stream);
        });
    }
    return listener;
}
fn handle_client(stream: TcpStream) {
    let adress = stream.peer_addr().unwrap().to_string();
    let mut stream_out = stream.try_clone().unwrap();
    //loop {
    //let stream_in = &mut &stream;

    let buf_reader = BufReader::new(stream);
    /*let mut buffer = [0 as u8; 512]; // using 512 byte buffer
    match stream_ref.read(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                println!("Server closed connection");
                break;
            }

            println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
            stream_ref.flush().expect("Flushing Error");
        }
        Err(e) => {
            if e.kind() == ErrorKind::ConnectionReset {
                handle_connection_lost(&adress);
                return;
            }else{
                panic!("Client hat kacke geschickt:{e}");
            }
        }
    }
    let response = "HEllo";

    stream_ref.write_all(response.as_bytes()).unwrap();*/

    for line_result in buf_reader.lines() {
        let out_line = match line_result {
            Ok(line) => line,
            Err(error) => match error.kind() {
                ErrorKind::ConnectionReset => handle_connection_lost(&adress),
                _ => {
                    panic!("Client hat kacke geschickt:{error}");
                }
            },
        };
        if out_line.is_empty() {
            return;
        }
        println!("{:?}", out_line);
        let response = "HEllo";

        stream_out.write_all(response.as_bytes()).unwrap();
        //}
    }
}
fn handle_connection_lost(stream: &String) -> String {
    println!("Lost Connection to Client{stream:?}");
    return String::new();
}
