use chrono::Local;
use regex::Match;

pub fn remote_controller_status() -> Vec<u8> {
    println!("[{} Received client command]: RemoteControllerStatus",Local::now().format("%Y-%m-%d %H:%M:%S"));

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

pub fn request_status() -> Vec<u8> {
    println!("[{} Received client command]: RequestStatus",Local::now().format("%Y-%m-%d %H:%M:%S"));

    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2001,"MessageId":"TACF-055109987-00002"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn request_scenario_list() -> Vec<u8> {
    println!("[{} Received client command]: RequestScenarioList",Local::now().format("%Y-%m-%d %H:%M:%S"));

    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2002,"MessageId":"TACF-055109987-00002"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn load_scenario(matched:Match) -> Vec<u8> {
    println!("[{} Received client command]: {}",Local::now().format("%Y-%m-%d %H:%M:%S"),matched.as_str());

    let command_str = matched.as_str().split(":").collect::<Vec<&str>>();
    let scenario_index = command_str.last().unwrap().parse::<String>().unwrap();

    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = format!("{}{}{}", r#"{"CommandId":2100,"ScenarioIndex":"#, scenario_index,r#","OnlineMode":false,"MessageId":"TACF-055209012-00003"}"#);
    let messages = messages.as_bytes().to_vec();

    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn request_scenario_infos() -> Vec<u8> {
    println!("[{} Received client command]: RequestScenarioInfos",Local::now().format("%Y-%m-%d %H:%M:%S"));

    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2003,"MessageId":"TACF-055219312-00004"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn start_training() -> Vec<u8> {
    println!("[{} Received client command]: StartTraining",Local::now().format("%Y-%m-%d %H:%M:%S"));

    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2300,"TrainingsName":"TempTraining","InstructorID":"InstructorID","InstructorName":"InstructorName","InstructorPrename":"InstructorPrename","InstructorRank":"InstructorRank","InstructorUnit":"InstructorUnit","OperatorID":"OperatorID","OperatorName":"OperatorName","OperatorPrename":"OperatorPrename","OperatorRank":"OperatorRank","OperatorUnit":"OperatorUnit","Participants":[],"MessageId":"TACF-073732657-000012"} "#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn start_scenario() -> Vec<u8> {
    println!("[{} Received client command]: StartScenario",Local::now().format("%Y-%m-%d %H:%M:%S"));

    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2200,"StepIndex":-1,"MessageId":"TACF-055219312-00004"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn stop_scenario() -> Vec<u8> {
    println!("[{} Received client command]: StopScenario",Local::now().format("%Y-%m-%d %H:%M:%S"));

    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2201,"StepIndex":-1,"MessageId":"TACF-055219312-00004"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn stop_training() -> Vec<u8> {
    println!("[{} Received client command]: StopTraining",Local::now().format("%Y-%m-%d %H:%M:%S"));

    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2301,"MessageId":"TACF-073732657-000012"} "#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn request_training_results() -> Vec<u8> {
    println!("[{} Received client command]: RequestTrainingResults",Local::now().format("%Y-%m-%d %H:%M:%S"));

    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2004,"MessageId":"TACF-073732657-000012"} "#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}

pub fn unload_scenario() -> Vec<u8> {
    println!("[{} Received client command]: UnloadScenario",Local::now().format("%Y-%m-%d %H:%M:%S"));

    let mut input = vec![2 as u8, 0 as u8, 50 as u8, 0 as u8];
    let messages = br#"{"CommandId":2101,"MessageId":"TACF-055209012-00003"}"#.to_vec();
    for message in messages {
        input.push(message)
    }
    input.push(3 as u8);

    input
}
