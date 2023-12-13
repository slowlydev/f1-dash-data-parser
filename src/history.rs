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
        self.frames.push(Frame::new(data.into()))
        // self.db.create("frame").content(Frame::new(data.into()))
    }

    pub fn add_updates(&mut self, updates: Vec<Update>) {
        if let Some(last) = self.frames.last() {
            // me make a new frame, as we want the new timestamp here
            let mut new: Frame = Frame::new(last.state.clone());
            for update in updates {
                new.state.update_field(update);
            }
            self.frames.push(new);
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    timestamp: chrono::DateTime<Utc>,
    state: parser::State,
}

impl Frame {
    pub fn new(state: parser::State) -> Frame {
        Frame {
            timestamp: Utc::now(),
            state,
        }
    }
}
