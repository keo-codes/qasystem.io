use std::env;

pub enum Command {
    Train,
    Ask(String),
}

pub fn parse() -> Command {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: train | ask \"question\"");
    }

    match args[1].as_str() {
        "train" => Command::Train,
        "ask" => Command::Ask(args.get(2).unwrap_or(&"".to_string()).clone()),
        _ => panic!("Unknown command"),
    }
}