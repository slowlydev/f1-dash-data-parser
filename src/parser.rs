pub mod deserializer;
pub mod models;

#[derive(Debug)]
pub enum ParsedMessage {
    Update(Vec<models::Update>),
    Replay(models::Data),
    Empty,
}

pub fn parse_message(message: String) -> ParsedMessage {
    let socket_message: models::SocketMessage =
        serde_json::from_str::<models::SocketMessage>(&message).unwrap();

    if let Some(updates) = socket_message.m {
        let mut vec: Vec<models::Update> = Vec::new();

        if updates.len() < 1 {
            return ParsedMessage::Empty;
        };

        for update in updates {
            vec.push(update.a);
        }

        return ParsedMessage::Update(vec);
    };

    if let Some(replay) = socket_message.r {
        return ParsedMessage::Replay(replay);
    }

    ParsedMessage::Empty
}
