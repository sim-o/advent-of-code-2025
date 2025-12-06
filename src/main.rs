use std::{fs, process::exit};

use crate::{
    challenge1a::challenge1a, challenge1b::challenge1b, challenge2a::challenge2a,
    challenge2b::challenge2b, challenge3a::challenge3a, challenge3b::challenge3b,
    challenge4a::challenge4a,
};
mod challenge1a;
mod challenge1b;
mod challenge2a;
mod challenge2b;
mod challenge3a;
mod challenge3b;
mod challenge4a;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: aoc25 <challenge> <a|b>");
        exit(1);
    }

    let challenge = &args[1];
    let input = match fs::read_to_string(challenge) {
        Ok(input) => input,
        Err(_) => {
            eprintln!("Error reading challenge input.");
            exit(1);
        }
    };

    let part = &args[2];

    let f = String::new() + challenge + part;
    let output = match &*f {
        "1a" => challenge1a(input),
        "1b" => challenge1b(input),
        "2a" => challenge2a(input),
        "2b" => challenge2b(input),
        "3a" => challenge3a(input),
        "3b" => challenge3b(input),
        "4a" => challenge4a(input),
        _ => {
            println!("No challenge {challenge}");
            exit(1);
        }
    };

    println!("solution: {output}");
}
