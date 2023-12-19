use chrono::Utc;
use serde_json::Value;

use crate::parser;

pub struct History {
    pub frames: Vec<Frame>,
}

impl History {
    pub fn new() -> History {
        History { frames: vec![] }
    }

    pub fn get_latest(&self) -> Option<&Frame> {
        self.frames.last()
    }

    // pub fn get_history(&self, delay_ms: Option<i64>) -> Vec<&Frame> {
    //     match delay_ms {
    //         Some(delay_ms) => {
    //             let adjusted_unix: i64 = 0 - delay_ms;
    //             self.frames
    //                 .iter()
    //                 .filter(|frame: &&Frame| frame.timestamp)
    //                 .collect()
    //         }
    //         None => self.frames.iter().collect::<Vec<&Frame>>(),
    //     }
    // }

    pub fn add_data(&mut self, data: Value) {
        let frame = Frame {
            state: todo!(),
            timestamp: todo!(),
        };

        self.frames.push(frame)
    }

    pub fn add_updates(&mut self, updates: Vec<parser::models::Message>) {
        // add it
    }
}

#[derive(Debug)]
pub struct Frame {
    timestamp: chrono::DateTime<Utc>,
    state: Value,
}
