use std::io::prelude::*;

use std::thread::sleep;
use std::time::Duration;
use crate::command::remote_controller_status;
use crate::response::response_to_jsons;
//use serde_json::Value;

mod response;
mod command;

fn main() -> std::io::Result<()> {
    use std::net::TcpStream;

    let mut stream = TcpStream::connect("127.0.0.1:1001").expect("Couldn't connect to the server...");

    let mut buffer = [0; 1024];

    let input = remote_controller_status();
    stream.write(&*input)?;

    loop {
        let offset = stream.read(&mut buffer[..])?;

        let response = String::from_utf8_lossy(&buffer[..offset]).to_string();

        if response.len()>0 {
            let items = buffer[..offset].to_vec().clone();
            let json_response = response_to_jsons(items);

            //println!("{}",json_response);
        }

        sleep(Duration::from_millis(500));
    }

    Ok(())
}
