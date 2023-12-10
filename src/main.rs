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
            parser::ParsedMessage::Replay(data) => history.add_data(data),
            parser::ParsedMessage::Update(updates) => {
                for update in updates {
                    history.add_update(update)
                }
            }
        };
    }

    println!("latest frame: {:?}", history.get_latest());
    println!("hisotry length: {:?}", history.frames.len());

    println!("done");
}
