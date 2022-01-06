pub fn response_to_jsons(items:Vec<u8>) -> Vec<String> {
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
        let mut will_push_str  = item.to_string();
        let first_str = item.chars().nth(0).unwrap().to_string();
        let last_str = item.chars().last().unwrap().to_string();

        if first_str != "{".to_string() && last_str != "}".to_string() {
            will_push_str = "{".to_string() + &item + "}";
        }

        if first_str == "{".to_string() && last_str != "}".to_string() {
            will_push_str = item.to_owned() + "}";
        }

        if first_str != "{".to_string() && last_str == "}".to_string() {
            will_push_str = "{".to_string() + &item;
        }

        jsons_strings.push(will_push_str);

    }

    jsons_strings
}
