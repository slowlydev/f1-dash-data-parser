use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

use crate::{merge, parser};

#[derive(Debug, Serialize)]
pub struct History {
    pub initial: Option<Value>,
    pub updates: Vec<parser::Update>,
    pub delay_states: HashMap<String, Value>,
    pub realtime: Option<Realtime>,
}

impl History {
    pub fn new() -> History {
        History {
            initial: None,
            updates: Vec::new(),
            delay_states: HashMap::new(),
            realtime: None,
        }
    }

    pub fn set_intitial(&mut self, state: Value) {
        self.realtime = Some(Realtime {
            timestamp: None,
            state: state.clone(),
        });

        self.initial = Some(state);
    }

    pub fn add_updates(&mut self, updates: &mut Vec<parser::Update>) {
        // we add the update to the updates vec
        // and update the realtime state

        if updates.len() < 1 {
            return;
        };

        self.updates.append(updates);

        if let Some(ref mut realtime) = self.realtime {
            for update in updates {
                realtime.timestamp = Some(update.timestamp);
                merge::merge(&mut realtime.state, &update.state);
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Realtime {
    timestamp: Option<DateTime<Utc>>,
    state: Value,
}
