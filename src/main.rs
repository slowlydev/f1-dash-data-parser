use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex},
    thread::{self},
    time::Duration,
};

use serde_json::Value;

use crate::parser::ParsedMessage;

mod file_loader;
mod history;
mod merge;
mod parser;

fn main() {
    let (history_tx, history_rx) = mpsc::channel::<ParsedMessage>();
    let (realtime_tx, realtime_rx) = mpsc::channel::<Option<Value>>();
    let (delay_request_tx, delay_request_rx) = mpsc::channel::<i64>();
    let (delay_response_tx, delay_response_rx) = mpsc::channel::<HashMap<i64, Value>>();

    let history = Arc::new(Mutex::new(history::History::new()));

    let history_recive = Arc::clone(&history);
    let history_delay = Arc::clone(&history);
    let history_delay_request = Arc::clone(&history);

    let history_recive_handle = thread::spawn(move || {
        for mut recived_message in history_rx {
            let mut hist = history_recive.lock().unwrap();

            println!("history: got message");

            match recived_message {
                parser::ParsedMessage::Empty => (),
                parser::ParsedMessage::Replay(state) => hist.set_intitial(state),
                parser::ParsedMessage::Updates(ref mut updates) => {
                    hist.add_updates(updates);
                }
            };

            let _ = realtime_tx.send(hist.get_realtime());
        }
    });

    let history_delay_handle = thread::spawn(move || loop {
        println!("history: delayed: getting new data");
        let mut hist = history_delay.lock().unwrap();
        let updated_states = hist.get_all_delayed();
        let _ = delay_response_tx.send(updated_states);

        thread::sleep(Duration::from_millis(100));
    });

    let history_delay_request_handle = thread::spawn(move || {
        for request in delay_request_rx {
            let mut hist = history_delay_request.lock().unwrap();
            println!("history: delayed: got request for {}", request);
            hist.get_delayed(&request);
        }
    });

    let file_handle = thread::spawn(move || {
        println!("file: reading");

        let Ok(messages) = file_loader::load_file() else {
            println!("failed reading file...");
            return;
        };

        println!("file: parsing");

        for message in messages {
            let parsed = parser::parse_message(message);
            let _ = history_tx.send(parsed);
            thread::sleep(Duration::from_millis(10));
        }

        println!("file: done");
    });

    let realtime_handle =
        thread::spawn(|| {
            println!("realtime: listening");

            for _ in realtime_rx {
                println!("realtime: got state");
            }

            println!("realtime: done");
        });

    let delayed_handle = thread::spawn(move || {
        println!("delayed: listening");

        let delay: i64 = 10;

        println!("delayed: requesting");
        let _ = delay_request_tx.send(delay);

        for response in delay_response_rx {
            println!("delayed: got response");
            if let Some(state) = response.get(&delay) {
                println!("delayed: got state for delay: {}: {}", delay, state);
            }
        }
    });

    file_handle.join().unwrap();
    history_recive_handle.join().unwrap();
    history_delay_handle.join().unwrap();
    realtime_handle.join().unwrap();
    delayed_handle.join().unwrap();
    history_delay_request_handle.join().unwrap();
}
