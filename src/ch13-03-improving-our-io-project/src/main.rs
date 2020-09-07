use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let testVec = vec![3, 4, 5];
    for i in 2..testVec.len() {
        println!("i: {}", i);
    }

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn performanceIter(buffer: &mut [i32], coefficients: [i64; 12], qlp_shift: i16) {
    for i in 12..buffer.len() {
        let prediction = coefficients
            .iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}
