use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;

// pub mod deserializer;
pub mod models;

#[derive(Debug)]
pub enum ParsedMessage {
    Update(Vec<models::Message>),
    Replay(HashMap<String, Value>),
    Empty,
}

pub fn parse_message(message: String) -> ParsedMessage {
    let socket_message: models::SocketMessage =
        serde_json::from_str::<models::SocketMessage>(&message).unwrap();

    if let Some(mut updates) = socket_message.m {
        if updates.len() < 1 {
            return ParsedMessage::Empty;
        };

        // TimingDataF1 is a dupe of TimingData
        updates.retain(|update| update.a.0 != "TimingDataF1");

        return ParsedMessage::Update(updates);
    };

    if let Some(mut replay) = socket_message.r {
        // TimingDataF1 is a dupe of TimingData
        replay.retain(|k, _| k != "TimingDataF1");

        return ParsedMessage::Replay(replay);
    }

    ParsedMessage::Empty
}

pub fn get_value_path<T: for<'de> serde::Deserialize<'de>>(value: &Value, path: &str) -> Option<T> {
    let mut current_value = value;

    for key in path.split('.') {
        current_value = match current_value.get(key) {
            Some(value) => value,
            None => return None,
        };
    }

    serde_json::from_value(current_value.clone()).ok()
}

const FORMAT1: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";
const FORMAT2: &'static str = "%Y-%m-%dT%H:%M:%S";

pub fn chrono_date(s: &str) -> Option<DateTime<Utc>> {
    let dt = NaiveDateTime::parse_from_str(&s, FORMAT1)
        .or_else(|_| NaiveDateTime::parse_from_str(&s, FORMAT2))
        .ok()?;

    Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
}
