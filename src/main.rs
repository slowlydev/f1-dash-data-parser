mod file_loader;
mod parser;

fn main() {
    println!("loading file...");

    let Ok(messages) = file_loader::load_file() else {
        println!("failed reading file...");
        return;
    };

    println!("parsing");

    let mut state: parser::State = parser::State::default();

    for message in messages {
        let parsed = parser::parse_message(message);

        match parsed {
            parser::ParsedMessage::Empty => (),
            parser::ParsedMessage::Replay(data) => state = data.into(),
            parser::ParsedMessage::Update(update) => {
                
            }
        };

        println!("{:?}", state);
    }

    println!("done");
}
