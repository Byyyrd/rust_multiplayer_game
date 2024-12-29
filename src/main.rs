use std::{env, sync::mpsc, thread};

pub mod graphics;
use graphics::*;
pub mod networking;
use networking::*;
fn main() -> Result<(), std::io::Error> {
    let render_threat = thread::spawn(|| {
        renderer::start();
    });
    
    let args: Vec<String> = env::args().collect();
    let usage = &args[1];
    let address = args[2].clone();

    let (sender, receiver) = mpsc::channel();

    if usage == "server" {
        thread::spawn(move||{
            server::start(&address,receiver);
        });
    } else if usage == "client" {
        thread::spawn(move||{
            client::start(&address,receiver);
        });
    }
    loop{
        if render_threat.is_finished() {
            println!("Shutdown...");
            _ = sender.send(());
            break;
        }
    }
    
    Ok(())
}

