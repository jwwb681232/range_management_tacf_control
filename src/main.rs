use std::io::prelude::*;
use std::net::TcpStream;
use serde_json::Value;

mod controller;
mod tacf;

use controller::NopAck;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1001")?;

    //let input = format!("{}{}{}", 0x02, "{'CommandId':2000,'MessageId':'030312689-000001'}", 0x03);
    //println!("Controller Request: {}", String::from_utf8_lossy(input.as_bytes()));

    //stream.write(input.as_bytes())?;

    let mut buffer = [0; 1024];

    loop {
        let offset = stream.read(&mut buffer[..])?;
        if offset > 0 {





            let response = String::from_utf8_lossy(&buffer[4..offset-1]).to_string();
            let response_json:Value = serde_json::from_str(&response).unwrap();
            //println!("TACF Response: {:#?}", response_json);

            let input_string = NopAck::new().input(response_json["MessageId"].to_string()).to_string();
            stream.write(input_string.as_bytes())?;
        }
    }
}
