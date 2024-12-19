use std::io::{prelude::*, BufReader, ErrorKind};
use std::io::{stdin, Write};
use std::net::TcpStream;
use std::process::exit;
use std::thread;
pub static ADDR: &str = "127.0.0.1:7070";
pub fn start() -> std::io::Result<()> {
    let stream = TcpStream::connect(ADDR).unwrap();
    let stream_copy = stream.try_clone().unwrap();

    thread::spawn(|| {
        read_data(stream);
    });
    return write_input(stream_copy);
}
fn write_input(mut stream: TcpStream) -> std::io::Result<()> {
    let mut input_text = String::new();
    let msg = "Hello, my name is client!\r\n";
    println!("{}", msg);
    stream.write_all(msg.as_bytes()).unwrap();
    loop {
        println!("Please enter some text: ");
        stdin()
            .read_line(&mut input_text)
            .expect("Did not enter a correct string");
        stream.write_all(input_text.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
fn read_data(stream: TcpStream) {
    let buf_reader = BufReader::new(stream);
    println!("REading");
    for line_result in buf_reader.lines() {
        let out_line = match line_result {
            Ok(line) => line,
            Err(error) => match error.kind() {
                ErrorKind::ConnectionReset => handle_server_closed(),
                _ => {
                    panic!("Server hat kacke geschickt:{:?}", error);
                }
            },
        };
        if out_line.is_empty() {
            return;
        }
        println!("Received: {:?}", out_line);
    }
}
fn handle_server_closed() -> String {
    println!("Server closed");
    exit(100);
}
