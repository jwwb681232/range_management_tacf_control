pub fn remote_controller_status() -> Vec<u8> {
    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2000,"MessageId":"TACF-073118689-000001","StationId":1,"StationName":"TestStation"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn nop_ack() -> Vec<u8> {
    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":999,"MessageId":""}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn request_scenario_list() -> Vec<u8> {
    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2002,"MessageId":"TACF-055109987-00002"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn load_scenario() -> Vec<u8> {
    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2100,"ScenarioIndex":2,"OnlineMode":false,"MessageId":"TACF-055209012-00003"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn request_scenario_infos() -> Vec<u8> {
    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2003,"MessageId":"TACF-055219312-00004"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}
