use std::collections::HashMap;

use serde_json::Value;

pub mod deserializer;
pub mod models;

#[derive(Debug)]
pub enum ParsedMessage {
    Update(HashMap<String, Value>),
    Replay(models::Data),
    Empty,
}

pub fn parse_message(message: String) -> ParsedMessage {
    // let Ok(socket_message) = serde_json::from_str::<models::SocketMessage>(&message) else {
    //     return ParsedMessage::Empty;
    // };

    let socket_message: models::SocketMessage =
        serde_json::from_str::<models::SocketMessage>(&message).unwrap();

    if let Some(updates) = socket_message.m {
        let mut map: HashMap<String, Value> = HashMap::new();

        for update in updates {
            map.insert(update.a.0, update.a.1);
        }

        return ParsedMessage::Update(map);
    };

    if let Some(replay) = socket_message.r {
        return ParsedMessage::Replay(replay);
    }

    ParsedMessage::Empty
}
