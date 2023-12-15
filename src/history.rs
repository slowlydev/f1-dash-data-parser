use chrono::Utc;
// use serde::Serialize;
// use surrealdb::engine::local::{Db, Mem};
// use surrealdb::Surreal;

use crate::parser::{
    self,
    models::{Data, Update},
};

pub struct History {
    pub frames: Vec<Frame>,
    // db: Surreal<Db>,
}

impl History {
    pub fn new() -> History {
        // let db: Surreal<Db> = Surreal::new::<Mem>(()).await.unwrap();

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

    pub fn add_data(&mut self, data: Data) {
        // self.frames.push(Frame::new(data.into()))
        let frame = Frame {
            state: data.clone().into(),
            timestamp: data.heartbeat.utc,
        };

        self.frames.push(frame)
    }

    pub fn add_updates(&mut self, updates: Vec<Update>) {
        if let Some(first) = updates.first() {
            let create_new_frame = if let Some(last) = self.frames.last() {
                (first.get_timestamp() - last.timestamp).num_seconds() >= 2
            } else {
                false
            };

            if create_new_frame {
                if let Some(last) = self.frames.last() {
                    self.frames.push(Frame {
                        state: last.state.clone(),
                        timestamp: first.get_timestamp(),
                    });
                }
            }
        }

        if let Some(last) = self.frames.last_mut() {
            for update in updates {
                last.state.update_field(update);
            }
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    timestamp: chrono::DateTime<Utc>,
    state: parser::State,
}
