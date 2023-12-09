

use crate::parser;

pub struct History {
    frames: Vec<Frame>,
}



impl History {
    pub fn get_latest(&self) -> Option<&Frame> {
        self.frames.last()
    }

    pub fn get_history(&self, delay_ms: Option<i64>) -> Vec<&Frame> {
        match delay_ms {
            Some(delay_ms) => {
                let adjusted_unix = 
                self.frames.iter().filter(|frame| frame.unix )

            },
            None => self.frames
        }

    }
}

pub struct Frame {
    // utc: String,
    unix: i128,
    state: parser::State,
}
