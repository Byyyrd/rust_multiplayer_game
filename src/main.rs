use std::env;

mod client;
mod server;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let usage = &args[1];
    let address = &args[2];
    if usage == "server" {
        server::start(&address);
        return Ok(());
    } else if usage == "client" {
        return client::start(&address);
    }
    Ok(())
}
