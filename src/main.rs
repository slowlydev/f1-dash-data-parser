mod file_loader;
mod parser;

fn main() {
    println!("loading file...");

    let Ok(messages) = file_loader::load_file() else {
        println!("failed reading file...");
        return;
    };

    println!("parsing");

    for message in messages {
        println!("");
        println!("raw message: {message}");
        println!("");

        let parsed = parser::parse_message(message);

        match parsed {
            parser::ParsedMessage::Empty => (),
            _ => println!("{parsed:?}"),
        };
    }

    println!("done");
}
