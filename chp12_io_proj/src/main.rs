use std::env;
// use std::error::Error;
// use std::fs;
use std::process;

use chp12_io_proj::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    // let query = &args[1];
    // let file_path = &args[2];

    // println!("Searching for {}", query);
    // println!("In file {}", file_path);

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    let args: Vec<String> = env::args().collect();

    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {err}");
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    // println!("With text:\n{contents}");

    if let Err(e) = chp12_io_proj::run(config) {
        // println!("Application error: {e}");
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// // trait objects
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;

//     println!("With text:\n{contents}");

//     Ok(())
// }

// struct Config {
//     query: String,
//     file_path: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Config { query, file_path }
//     }

//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }
// }

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }
