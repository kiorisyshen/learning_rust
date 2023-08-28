use crate::garden::vegetables::Asparagus;

pub mod garden;

use std::fs::File;
use std::io::{self, Read};

fn main() {
    let a = Asparagus::default();
    println!("a: {:?}", a);

    {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];
        // let first = &v[100]; // will failed in runtime
        // v.push(6); // will failed in compile

        println!("The first element is: {first}");
        v.push(6);

        for i in &mut v {
            *i += 50;
        }

        for i in &v {
            println!("{i}");
        }
    }

    {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        println!("row: {:?}", row);
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        println!("{s3}");

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
        println!("{s}");
    }

    {
        // let hello = "Здравствуйте";
        let hello = String::from("Здравствуйте");
        // let answer = &hello[0];
        // let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("{s}");

        for c in "Зд".chars() {
            println!("{c}");
        }
        for b in "Зд".bytes() {
            println!("{b}");
        }
    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        // let score = scores.get(&team_name).copied().unwrap_or(0);
        // if let score = scores.get(&team_name) {
        //     println!("{score}");
        // }

        if let std::option::Option::Some(s) = scores.get(&team_name) {
            println!("{s}");
        } else {
            println!("no such team {team_name}");
        }

        for (key, value) in &scores {
            println!("{key}: {value}");
        }

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        {
            let text = "hello world wonderful world";

            let mut map = HashMap::new();

            for word in text.split_whitespace() {
                let count = map.entry(word).or_insert(0);
                *count += 1;
            }

            println!("{:?}", map);
        }
    }

    {
        let greeting_file_result = File::open("hello.txt");

        // fully process using match
        // let greeting_file = match greeting_file_result {
        //     Ok(file) => file,
        //     Err(error) => match error.kind() {
        //         ErrorKind::NotFound => match File::create("hello.txt") {
        //             Ok(fc) => fc,
        //             Err(e) => panic!("Problem creating the file: {:?}", e),
        //         },
        //         other_error => {
        //             panic!("Problem opening the file: {:?}", other_error);
        //         }
        //     },
        // };

        // unwrap, panic at err
        let greeting_file = File::open("hello.txt").unwrap();

        // expect, panic with msg
        let greeting_file =
            File::open("hello.txt").expect("hello.txt should be included in this project");

        read_username_from_file();
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // fully process
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // using ?
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    use std::fs;
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
