use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use serde_json::Value;


fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1001")?;

    let input = format!("{}{}{}", 0x02, "{\"CommandId\":2000,\"MessageId\":\"TACF-055330689-000001\",\"StationId\":1,\"StationName\":\"TestStation\"}", 0x03);
    println!("Controller Request: {}", String::from_utf8_lossy(input.as_bytes()));

    stream.write(input.as_bytes())?;
    stream.write(input.as_bytes())?;

    let mut buffer = [0; 1024];
    /*
        let offset = stream.read(&mut buffer[..])?;

        let response = String::from_utf8_lossy(&buffer[4..offset-1]).to_string();
        let response_json:Value = serde_json::from_str(&response).unwrap();
        println!("TACF Response: {:#?}", response_json);

        let input = format!("{}{{'CommandId':{},'MessageId':'{}'}}{}", 0x02, 999,response_json["MessageId"], 0x03);
        println!("Controller Request: {}", String::from_utf8_lossy(input.as_bytes()));*/

    let mut count = 0;
    loop {
        let offset = stream.read(&mut buffer[..])?;
        //if offset > 0 {

            let response = String::from_utf8_lossy(&buffer[..offset]).to_string();
            //let response_json:Value = serde_json::from_str(&response).unwrap();
            println!("TACF Response {}: {}", count,response);
            sleep(Duration::from_secs(1));
            count+=1;
        //}
    }

    Ok(())
}
