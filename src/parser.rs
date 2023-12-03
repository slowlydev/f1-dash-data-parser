pub mod deserializer;
pub mod models;

#[derive(Debug)]
pub enum ParsedMessage {
    Update(Vec<models::DataType>),
    Replay(models::Data),
    Empty,
}

pub fn parse_message(message: String) -> ParsedMessage {
    println!("message: {message:?}");

    let socket_message: models::SocketMessage =
        serde_json::from_str::<models::SocketMessage>(&message).unwrap();

    if let Some(updates) = socket_message.m {
        let mut vec: Vec<models::DataType> = Vec::new();

        if updates.len() < 1 {
            return ParsedMessage::Empty;
        };

        for update in updates {
            vec.push(update.a.1);
        }

        return ParsedMessage::Update(vec);
    };

    if let Some(replay) = socket_message.r {
        return ParsedMessage::Replay(replay);
    }

    ParsedMessage::Empty
}
