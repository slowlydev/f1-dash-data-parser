use std::collections::HashMap;

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

use crate::parser;

#[derive(Serialize)]
pub struct History {
    pub frames: HashMap<String, Vec<Frame>>,
}

impl History {
    pub fn new() -> History {
        History {
            frames: HashMap::new(),
        }
    }

    pub fn get_latest(&self) -> HashMap<String, &Frame> {
        let mut map: HashMap<String, &Frame> = HashMap::new();

        for (k, v) in &self.frames {
            if let Some(last_frame) = v.last() {
                map.insert(k.to_owned(), last_frame);
            }
        }

        map
    }

    fn get_category(&self, category: &str) -> Option<&Vec<Frame>> {
        self.frames.get(category)
    }

    pub fn get_latest_category(&self, category: &str) -> Option<&Frame> {
        self.frames.get(category).and_then(|vec| vec.last())
    }

    pub fn add_data(&mut self, data: HashMap<String, Value>) {
        for (k, v) in data {
            let frame = Frame {
                state: v,
                timestamp: None,
            };

            // note we are overwritng everything here, which should be fine
            // as this is the first message form the f1 socket
            self.frames.insert(k, vec![frame]);
        }
    }

    pub fn add_updates(&mut self, updates: Vec<parser::models::Message>) {
        if updates.len() < 1 {
            return;
        };

        for update in updates {
            let cat = &update.a.0;

            if let Some(last_frame) = self.get_latest_category(&cat) {
                let base = &mut last_frame.state.clone();

                merge(base, &update.a.1);

                let timestamp = parser::chrono_date(&update.a.2);

                let frame = Frame {
                    state: base.take(),
                    timestamp,
                };

                if let Some(cat_history) = self.frames.get_mut(cat) {
                    cat_history.push(frame);
                }
            }
        }
    }

    // pub fn get(&self, delay_ms: Option<i64>) -> Vec<&Frame> {
    //     let current_time = chrono::Utc::now().timestamp();
    //     match delay_ms {
    //         Some(delay_ms) => {
    //             let adjusted_unix: i64 = current_time - delay_ms;
    //             self.frames
    //                 .iter()
    //                 .filter(|frame: &&Frame| frame.timestamp.timestamp() >= adjusted_unix)
    //                 .collect()
    //         }
    //         None => self.frames.iter().collect::<Vec<&Frame>>(),
    //     }
    // }
}

#[derive(Serialize, Debug)]
pub struct Frame {
    timestamp: Option<chrono::DateTime<Utc>>,
    state: Value,
}

pub fn merge(base: &mut Value, update: &Value) {
    match (base, update) {
        (Value::Object(ref mut prev), &Value::Object(ref update)) => {
            for (k, v) in update {
                merge(prev.entry(k).or_insert(Value::Null), v);
            }
        }
        (Value::Array(ref mut a), &Value::Array(ref b)) => {
            a.extend(b.clone());
        }
        (Value::Array(ref mut prev), Value::Object(ref update)) => {
            for (k, v) in update {
                // key is "_deleted"
                if k == "_deleted" {
                    if let Ok(index_to_delete) = serde_json::from_value::<Vec<usize>>(v.to_owned())
                    {
                        for index in index_to_delete {
                            // or we chenge to prev.delete(index) if we don't want to show/mark deleted times in the UI
                            if let Some(item) = prev.get_mut(index) {
                                if let Value::Object(item_map) = item {
                                    item_map.insert(String::from("deleted"), Value::Bool(true));
                                }
                            }
                        }
                    }
                }

                // find item with racing number eq to k
                let rnr_item = prev.iter_mut().find(|val| {
                    if let Some(rnr) = val.get("RacingNumber") {
                        if rnr == k {
                            return true;
                        }
                    }
                    false
                });

                if let Some(rnr_item_found) = rnr_item {
                    merge(rnr_item_found, v)
                } else {
                    k.parse::<usize>()
                        .ok()
                        .and_then(|index| prev.get_mut(index).map(|item| merge(item, v)));
                }
            }
        }
        (a, b) => *a = b.clone(),
    }
}
