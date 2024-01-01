use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

use crate::{merge, parser};

#[derive(Debug)]
pub struct History {
    pub initial: Option<Value>,
    pub updates: Vec<parser::Update>,
    pub delay_states: HashMap<i64, AsyncState>,
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
                // realtime.timestamp = Some(update.timestamp);
                realtime.timestamp = Some(Utc::now());
                merge::merge(&mut realtime.state, &update.state);
            }
        }
    }

    pub fn get_all_delayed(&mut self) -> HashMap<i64, Value> {
        let mut map = HashMap::new();

        for (k, _) in self.delay_states.clone() {
            if let Some(val) = self.get_delayed(&k) {
                map.insert(k.to_owned(), val);
            }
        }

        map
    }

    pub fn get_delayed(&mut self, delay: &i64) -> Option<Value> {
        if let Some(existing) = self.delay_states.get_mut(&delay) {
            let delayed_timestamp = chrono::Utc::now().timestamp() - delay;

            if existing.next_timestamp.timestamp() >= delayed_timestamp {
                let mut latest_update_index: usize = 0;

                for (pos, update) in self.updates.iter().enumerate() {
                    if update.timestamp.timestamp() > existing.current_timestamp.timestamp() {
                        continue;
                    };

                    if update.timestamp.timestamp() > delayed_timestamp {
                        continue;
                    };

                    latest_update_index = pos;

                    merge::merge(&mut existing.state, &update.state);
                }

                let current = self.updates.get(latest_update_index).unwrap();
                let next = self.updates.get(latest_update_index + 1).unwrap();

                existing.current_timestamp = current.timestamp;
                existing.next_timestamp = next.timestamp;
            }

            return Some(existing.state.clone());
        };

        if let Some(initial_state) = &self.initial {
            let mut base = initial_state.clone();

            let delayed_timestamp = chrono::Utc::now().timestamp() - delay;

            let mut latest_update_index: usize = 0;

            for (pos, update) in self.updates.iter().enumerate() {
                if update.timestamp.timestamp() <= delayed_timestamp {
                    continue;
                };

                if update.timestamp.timestamp() > delayed_timestamp {
                    continue;
                };

                latest_update_index = pos;

                merge::merge(&mut base, &update.state);
            }

            let current = self.updates.get(latest_update_index).unwrap();
            let next = self.updates.get(latest_update_index + 1).unwrap();

            let async_state = AsyncState {
                state: base,
                current_timestamp: current.timestamp,
                next_timestamp: next.timestamp,
            };

            self.delay_states
                .insert(delay.to_owned(), async_state.clone());
            return Some(async_state.state);
        }

        None
    }

    fn filter_updates(&self, catagory: &str) -> Vec<&Value> {
        self.updates
            .iter()
            .filter(|el| el.catagory == catagory)
            .map(|el| &el.state)
            .collect()
    }

    pub fn get_realtime(&self) -> Option<Value> {
        if let Some(realtime) = &self.realtime {
            let mut history_less = realtime.state.clone();

            let weather_updates: Vec<&Value> = self.filter_updates("WeatherData");
            let mut weather: HashMap<String, Vec<Value>> = HashMap::new();

            for update in &weather_updates {
                if let Value::Object(obj) = update {
                    for (k, v) in obj {
                        if let Some(existing_key) = weather.get_mut(k) {
                            existing_key.push(v.clone());
                        } else {
                            weather.insert(k.to_owned(), vec![v.clone()]);
                        }
                    }
                }
            }

            if let Value::Object(ref mut history) = history_less {
                history.insert(
                    "WeatherData.history".to_owned(),
                    serde_json::to_value(weather).unwrap(),
                );
            }

            // weather
            // - [column]
            // gap
            // - Lines.[nr].IntervalToPositionAhead.Value
            // laptimes
            // - Lines.[nr].LastLapTime.Value
            // - Lines.[nr].LastLapTime.OverallFastest
            // - Lines.[nr].LastLapTime.PersonalFastest
            // sector times
            // - Lines.[nr].Sectors.[sector].Value
            // - Lines.[nr].Sectors.[sector].OverallFastest
            // - Lines.[nr].Sectors.[sector].PersonalFastest

            return Some(history_less);
        }

        None
    }
}

#[derive(Debug)]
pub struct Realtime {
    timestamp: Option<DateTime<Utc>>,
    state: Value,
}

#[derive(Debug, Clone)]
pub struct AsyncState {
    pub state: Value,
    pub current_timestamp: DateTime<Utc>,
    pub next_timestamp: DateTime<Utc>,
}
