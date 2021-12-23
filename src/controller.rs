use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NopAck {
    command_id: u32,
    message_id: String,
}

impl NopAck {
    pub fn new() -> Self {
        NopAck {
            command_id: 999,
            message_id: "".to_string(),
        }
    }

    pub fn input(mut self, received_message_id: String) -> Self {
        self.message_id = received_message_id;
        self
    }
}

impl Display for NopAck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", 0x02, serde_json::to_string(&self).unwrap(), 0x03)
    }
}
