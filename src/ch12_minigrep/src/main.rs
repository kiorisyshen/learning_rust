use ch12_minigrep::Config;
use std::env;
use std::process;
// std::env::args_os // Use this if arguments have invalid unicode

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = ch12_minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
