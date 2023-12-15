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
    let mut num_updates: usize = 0;
    let mut num_rows: usize = 0;

    for message in messages {
        let parsed = parser::parse_message(message);

        match parsed {
            parser::ParsedMessage::Empty => (),
            parser::ParsedMessage::Replay(data) => {
                num_updates = num_updates + 1;
                num_rows = num_rows + 1;

                history.add_data(data);
            }
            parser::ParsedMessage::Update(updates) => {
                num_updates = num_updates + updates.len();
                num_rows = num_rows + 1;

                history.add_updates(updates);
            }
        };
    }

    println!("latest frame: {:?}", history.get_latest());
    println!("history length: {:?}", history.frames.len());
    println!("rows: {:?}", num_rows);
    println!("updates: {:?}", num_updates);

    println!("done");
}
