use std::{fs, io::Write};

mod file_loader;
mod history;
mod parser;

fn main() {
    println!("loading file...");

    let Ok(messages) = file_loader::load_file() else {
        println!("failed reading file...");
        return;
    };

    println!("parsing");

    let mut history: history::History = history::History::new();

    for message in messages {
        let parsed = parser::parse_message(message);

        match parsed {
            parser::ParsedMessage::Empty => (),
            parser::ParsedMessage::Replay(data) => {
                history.add_data(data);
            }
            parser::ParsedMessage::Update(updates) => {
                history.add_updates(updates);
            }
        };
    }

    for (k, v) in &history.frames {
        println!("history: {:?}: {:?}", k, v.len());
    }

    let binding = serde_json::to_string(&history).unwrap();
    let buf = binding.as_bytes();
    let _ = fs::File::create("history.json").and_then(|mut file| file.write_all(buf));

    println!("done");
}
