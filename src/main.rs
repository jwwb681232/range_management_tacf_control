use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use serde_json::Value;


fn main() -> std::io::Result<()> {

    let mut stream = TcpStream::connect("127.0.0.1:1001")?;
    let mut buffer = [0; 1024];
    sleep(Duration::from_secs(1));


    let mut input = vec![2 as u8,0 as u8,50 as u8,0 as u8];
    let messages = br#"{"CommandId":2000,"MessageId":"TACF-073118689-000001","StationId":1,"StationName":"TestStation"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    stream.write(&*input)?;

    sleep(Duration::from_secs(1));

    let offset = stream.read(&mut buffer[..])?;
    let response = String::from_utf8_lossy(&buffer[..offset]).to_string();
    println!("TACF Response : {}", response);
    println!("TACF Response : {:?}", response.as_bytes());

    Ok(())

}
