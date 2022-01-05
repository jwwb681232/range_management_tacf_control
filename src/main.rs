use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json::Value;

use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use redis::Commands;

use crate::commands::{load_scenario, nop_ack, remote_controller_status, request_scenario_infos, request_scenario_list};
use crate::response::response_to_jsons;

mod commands;
mod response;


/***************************************************************************************************/
/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut con = client.get_connection().unwrap();
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let _: () = con.publish("tacf", &text).unwrap();
            }
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWs {}, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::thread::spawn(|| {
        tacf();
    });

    HttpServer::new(|| App::new().route("/tacf_control", web::get().to(index)))
        .bind("127.0.0.1:8881")?
        .run()
        .await

}

fn tacf() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1001").expect("Couldn't connect to the server...");

    let mut buffer = [0; 1024];

    let input = remote_controller_status();
    stream.write(&*input)?;

    let mut redis_thread_stream = stream.try_clone().unwrap();
    std::thread::spawn(move|| {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut con = client.get_connection().unwrap();
        let mut pubsub = con.as_pubsub();
        pubsub.subscribe("tacf").unwrap();

        loop {
            let payload:String = pubsub.get_message().unwrap().get_payload().unwrap();
            println!("{}",payload);
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
