pub fn client(){
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
}
