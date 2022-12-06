use std::collections::HashSet;
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

    for line in reader.lines() {
        let line = line.unwrap();
        // println!("{}", line);

        if line.trim().eq("") {
           continue;
        }

        let mut curr_window = line.chars().take(14).collect::<Vec<char>>();
        let mut seen_count = 14;

        for c in line.chars().skip(14) {
            let temp_set: HashSet<char> = HashSet::from_iter(curr_window.clone());
            if temp_set.len() == 14 {
                println!("seen count {}", seen_count);
                break;
            } else {
                seen_count += 1;
                curr_window.remove(0);
                curr_window.push(c);
            }
        }
    }
}
