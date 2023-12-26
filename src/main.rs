mod file_loader;
mod history;
mod merge;
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
        let mut parsed = parser::parse_message(message);

        match parsed {
            parser::ParsedMessage::Empty => (),
            parser::ParsedMessage::Replay(state) => history.set_intitial(state),
            parser::ParsedMessage::Updates(ref mut updates) => {
                history.add_updates(updates);
            }
        };
    }

    println!("realtime: {:?}", history.realtime);
    println!("updates: {}", history.updates.len());

    println!("done");
}
