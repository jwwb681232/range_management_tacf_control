use std::io::prelude::*;

use std::thread::sleep;
use std::time::Duration;
//use serde_json::Value;


fn main() -> std::io::Result<()> {
    use std::net::TcpStream;

    let mut stream = TcpStream::connect("127.0.0.1:1001").expect("Couldn't connect to the server...");

    let mut buffer = [0; 1024];

    let mut input = vec![2 as u8,0 as u8,50 as u8,0 as u8];
    let messages = br#"{"CommandId":2000,"MessageId":"TACF-073118689-000001","StationId":1,"StationName":"TestStation"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);
    stream.write(&*input)?;

    loop {
        let offset = stream.read(&mut buffer[..])?;

        let response = String::from_utf8_lossy(&buffer[..offset]).to_string();

        if response.len()>0 {

            //let jsons:Vec<String>;

            let items = buffer[..offset].to_vec().clone();

            let mut clear_items = vec![];

            for (index,item) in items.iter().enumerate() {
                if *item == (2 as u8) && items[index+1] == (0 as u8) && items[index+2] == (50 as u8) && items[index+3] == (0 as u8){
                    continue;
                }

                if *item == (0 as u8) && items[index+1] == (50 as u8) && items[index+2] == (0 as u8) && items[index-1] == (2 as u8){
                    continue;
                }

                if *item == (50 as u8) && items[index+1] == (0 as u8) && items[index-2] == (2 as u8) && items[index-1] == (0 as u8){
                    continue;
                }

                if *item == (0 as u8) && items[index-3] == (2 as u8) && items[index-2] == (0 as u8) && items[index-1] == (50 as u8){
                    continue;
                }

                if *item == b"}"[0] && items[index+1] == (3 as u8){
                    clear_items.push(b"}"[0]);
                    continue;
                }

                if *item == (3 as u8) && items[index-1] == b"}"[0]{
                    continue;
                }

                clear_items.push(items[index])
            }

            println!("{}",String::from_utf8(clear_items).unwrap());
        }

        sleep(Duration::from_millis(500));
    }

    Ok(())
}
