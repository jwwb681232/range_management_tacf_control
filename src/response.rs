use std::fmt::format;
use serde_json::json;

pub fn response_to_jsons(items:Vec<u8>) -> String {
    let mut jsons_strings:Vec<String> = vec![];

    let mut clear_items = vec![];

    for (index,item) in items.iter().enumerate() {
        if *item == (2 as u8) && items[index + 1] == (0 as u8) && items[index + 2] == (50 as u8) && items[index + 3] == (0 as u8) {
            continue;
        }

        if *item == (0 as u8) && items[index + 1] == (50 as u8) && items[index + 2] == (0 as u8) && items[index - 1] == (2 as u8) {
            continue;
        }

        if *item == (50 as u8) && items[index + 1] == (0 as u8) && items[index - 2] == (2 as u8) && items[index - 1] == (0 as u8) {
            continue;
        }

        if *item == (0 as u8) && items[index - 3] == (2 as u8) && items[index - 2] == (0 as u8) && items[index - 1] == (50 as u8) {
            continue;
        }

        if *item == b"}"[0] && items[index + 1] == (3 as u8) {
            clear_items.push(b"}"[0]);
            continue;
        }

        if *item == (3 as u8) && items[index - 1] == b"}"[0] {
            continue;
        }

        clear_items.push(items[index])
    }

    let one_line = String::from_utf8(clear_items).unwrap();

    let splited = one_line.split("}{").collect::<Vec<&str>>();

    for &item in splited.iter() {
        let first_str = item.chars().nth(0).unwrap();
        let last_str = item.chars().last().unwrap();

        if first_str.to_string() != "{".to_string() {
            let json_string = format!("{}{}","{".to_string(),item.to_string());
            println!("{:#?}",json!(json_string.as_str()));
            jsons_strings.push(json_string);



        }else if last_str.to_string() != "}".to_string() {
            let json_string = format!("{}{}",item.to_string(),"}".to_string());
            println!("{:#?}",json!(json_string.as_str()));
            jsons_strings.push(json_string);


        }else{
            jsons_strings.push(item.to_string());
        }
    }

    println!("{:#?}",jsons_strings);

    one_line
}
