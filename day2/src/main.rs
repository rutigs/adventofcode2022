use std::fs::File;
use std::env;
use std::process;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    println!("Received {} arguments", args.len());

    if args.len() < 2 {
        println!("missing file path argument!");
        process::exit(1);
    }

    let file_name = &args[1];
    let file = File::open(file_name).unwrap(); // ignore errors from Result type
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let strategy: Vec<&str> = line.trim().split(" ").collect();

        let opponent_play = strategy[0].trim();
        let your_play = strategy[1].trim();

        total_score += rps_match(opponent_play, your_play);
    }

    println!("Total score is: {}", total_score);
}

fn rps_match(opponent_play: &str, your_play: &str) -> i32 {
    // A 1 rock
    // B 2 paper
    // C 3 scissors

    // X lose
    // Y draw
    // Z win
    
    match opponent_play {
    "A"=> match your_play {
        "X" => return 0 + 3,
        "Y" => return 3 + 1,
        "Z" => return 6 + 2,
        _ => return 0,
    },
    "B"=> match your_play {
        "X" => return 0 + 1,
        "Y" => return 3 + 2,
        "Z" => return 6 + 3,
        _ => return 0,   
    },
    "C"=> match your_play {
        "X" => return 0 + 2,
        "Y" => return 3 + 3,
        "Z" => return 6 + 1,
        _ => return 0,  
    },
    _ => {
        println!("Invalid input!");
        return 0;
    },
    }
}
