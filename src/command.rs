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
