//#![windows_subsystem = "windows"]

extern crate ws;
extern crate chrono;
extern crate dotenv;
extern crate redis;

mod commands;
mod response;

use std::io::{Read, Write};
use std::net::TcpStream;
use chrono::Local;
use redis::Commands;
use serde_json::Value;
use crate::commands::{load_scenario, nop_ack, remote_controller_status, request_scenario_infos, request_scenario_list};
use crate::response::response_to_jsons;

struct Router {
    sender: ws::Sender,
    inner: Box<dyn ws::Handler>,
}

impl ws::Handler for Router {
    fn on_shutdown(&mut self) {
        self.inner.on_shutdown()
    }

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.inner.on_open(shake)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.inner.on_message(msg)
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        self.inner.on_close(code, reason)
    }

    fn on_error(&mut self, err: ws::Error) {
        self.inner.on_error(err)
    }

    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        let out = self.sender.clone();

        match req.resource() {
            "/" => self.inner = Box::new(NFCReceiver),
            "/tacf_control" => self.inner = Box::new(TacfControl { ws: out }),
            _ => self.inner = Box::new(NotFound),
        }

        self.inner.on_request(req)
    }
}

struct NotFound;

impl ws::Handler for NotFound {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        let mut res = ws::Response::from_request(req)?;
        res.set_status(404);
        res.set_reason("Not Found");
        Ok(res)
    }
}


struct TacfControl {
    ws: ws::Sender,
}

impl ws::Handler for TacfControl {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut con = client.get_connection().unwrap();
        let _: () = con.publish("websocket_to_tacf", msg.to_string()).unwrap();

        println!("Step 2: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        //self.ws.broadcast(msg)

        if msg.to_string() != "RequestScenarioList".to_string() {
            self.ws.broadcast(msg).unwrap()
        }
        Ok(())
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {

    }
}

struct NFCReceiver;

impl ws::Handler for NFCReceiver {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        println!("Step 1: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        Ok(())
    }

    fn on_message(&mut self, _: ws::Message) -> ws::Result<()> {
        Ok(())
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
    }
}

fn main() {

    //一个客户端专门从tacf_to_websocket 里取数据广播
    let inner_sender_thread = std::thread::spawn(||{
        let mut tacf_to_websocket_con = redis::Client::open("redis://127.0.0.1:6379").unwrap().get_connection().unwrap();
        let mut tacf_to_websocket_pubsub = tacf_to_websocket_con.as_pubsub();
        tacf_to_websocket_pubsub.subscribe("tacf_to_websocket").unwrap();

        loop {
            let msg = tacf_to_websocket_pubsub.get_message().unwrap();
            let payload : String = msg.get_payload().unwrap();

            ws::connect("ws://127.0.0.1:8084/tacf_control", move|out| {
                out.send(payload.to_owned()).unwrap();
                move |_| {
                    out.close(ws::CloseCode::Normal).unwrap();
                    Ok(())
                }
            }).unwrap();

            //std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });


    //tacf 数据交互
    let tacf_thread = std::thread::spawn(|| {
        tacf().unwrap();
    });


    //websocket 服务端
    ws::listen(format!("127.0.0.1:8084"), |out| {
        Router {
            sender: out,
            inner: Box::new(NFCReceiver),
        }
    }).unwrap();


    let _ = inner_sender_thread.join();
    let _ = tacf_thread.join();

}

fn tacf() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1001").expect("Couldn't connect to the server...");
    let mut buffer = [0; 1024];
    let input = remote_controller_status();
    stream.write(&*input)?;


    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut con = client.get_connection().unwrap();

    let mut redis_thread_stream = stream.try_clone().unwrap();
    std::thread::spawn(move|| {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut con = client.get_connection().unwrap();
        let mut pubsub = con.as_pubsub();
        pubsub.subscribe("websocket_to_tacf").unwrap();

        loop {
            let payload:String = pubsub.get_message().unwrap().get_payload().unwrap();

            if payload == "RequestScenarioList".to_string() {
                let input = request_scenario_list();
                redis_thread_stream.write(&*input).unwrap();
            }

            if payload == "LoadScenario".to_string() {
                let input = load_scenario();
                redis_thread_stream.write(&*input).unwrap();
            }

            if payload == "RequestScenarioInfos".to_string() {
                let input = request_scenario_infos();
                redis_thread_stream.write(&*input).unwrap();
            }

        }
    });

    loop {
        let offset = stream.read(&mut buffer[..])?;

        let response = String::from_utf8_lossy(&buffer[..offset]).to_string();

        if response.len() > 0 {
            let items = buffer[..offset].to_vec().clone();
            let json_response = response_to_jsons(items);

            for item in json_response {

                let _: () = con.publish("tacf_to_websocket", &item).unwrap();

                let decode_item: Value = serde_json::from_str(&item).unwrap();

                if decode_item["CommandId"] == 0 {
                    let input = nop_ack();
                    stream.write(&*input)?;
                    println!("command 0 :{}", decode_item)
                }

                if decode_item["CommandId"] == 1001 {
                    println!("command 1001 :{}", decode_item)
                }

                if decode_item["CommandId"] == 1002 {
                    println!("command 1002 :{}", decode_item)
                }

                if decode_item["CommandId"] == 1003 {
                    println!("command 1003 :{}", decode_item)
                }
            }
        }
    }
}
