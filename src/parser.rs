use serde_json::Value;

pub mod deserializer;
pub mod models;

#[derive(Debug)]
pub enum ParsedMessage {
    Update(Vec<models::Message>),
    Replay(Value),
    Empty,
}

pub fn parse_message(message: String) -> ParsedMessage {
    let socket_message: models::SocketMessage =
        serde_json::from_str::<models::SocketMessage>(&message).unwrap();

    if let Some(updates) = socket_message.m {
        if updates.len() < 1 {
            return ParsedMessage::Empty;
        };

        return ParsedMessage::Update(updates);
    };

    if let Some(replay) = socket_message.r {
        return ParsedMessage::Replay(replay);
    }

    ParsedMessage::Empty
}
