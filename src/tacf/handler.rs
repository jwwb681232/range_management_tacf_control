use std::io::{Read, Write};
use std::net::TcpStream;
use redis::Commands;
use serde_json::Value;
use crate::tacf::commands::{load_scenario, nop_ack, remote_controller_status, request_scenario_infos, request_scenario_list, request_training_results, start_scenario, start_training, stop_scenario, stop_training, unload_scenario};
use crate::tacf::response::response_to_jsons;

pub fn run() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1001").expect("Couldn't connect to the server...");
    //let mut buffer = [0; 65535];
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

            if payload == "StartTraining".to_string() {
                let input = start_training();
                redis_thread_stream.write(&*input).unwrap();
            }

            if payload == "StartScenario".to_string() {
                let input = start_scenario();
                redis_thread_stream.write(&*input).unwrap();
            }

            if payload == "StopScenario".to_string() {
                let input = stop_scenario();
                redis_thread_stream.write(&*input).unwrap();
            }

            if payload == "StopTraining".to_string() {
                let input = stop_training();
                redis_thread_stream.write(&*input).unwrap();
            }

            if payload == "RequestTrainingResults".to_string() {
                let input = request_training_results();
                redis_thread_stream.write(&*input).unwrap();
            }

            if payload == "UnloadScenario".to_string() {
                let input = unload_scenario();
                redis_thread_stream.write(&*input).unwrap();
            }

        }
    });

    //let mut buffer = [0; 65535];
    let mut incoming_bytes = Vec::<u8>::new();

    loop {
        let mut incoming_trunk = vec![0u8; 1024*8];
        let _bytes_count = stream.read(&mut incoming_trunk)?;
        println!("{}",_bytes_count);
        if _bytes_count == 0 {
            break;
        } else {
            incoming_bytes.append(&mut incoming_trunk);
        }
    }

    loop {
        // todo 如果最后不是 3 as u8 这个byte就继续写入buffer
        //let offset = stream.read(&mut buffer[..])?;

        /*let len = incoming_bytes.len();
        if len > 0 {
            println!("{}",len);
        }*/

        if incoming_bytes.len() > 0 {
            let json_response = response_to_jsons(incoming_bytes.clone());

            for item in json_response {

                let _: () = con.publish("tacf_to_websocket", &item).unwrap();

                let decode_item: Value = serde_json::from_str(&item).unwrap();

                if decode_item["CommandId"] == 0 {
                    let input = nop_ack();
                    stream.write(&*input)?;
                }

                if decode_item["CommandId"] == 1001 {
                }

                if decode_item["CommandId"] == 1002 {
                }

                if decode_item["CommandId"] == 1003 {
                }
            }
        }

        //let response = String::from_utf8_lossy(&incoming_bytes).to_string();

        /*if incoming_bytes.len() > 0 {
            let json_response = response_to_jsons(incoming_bytes);

            for item in json_response {

                let _: () = con.publish("tacf_to_websocket", &item).unwrap();

                let decode_item: Value = serde_json::from_str(&item).unwrap();

                if decode_item["CommandId"] == 0 {
                    let input = nop_ack();
                    stream.write(&*input)?;
                }

                if decode_item["CommandId"] == 1001 {
                }

                if decode_item["CommandId"] == 1002 {
                }

                if decode_item["CommandId"] == 1003 {
                }
            }
        }*/
    }
}
