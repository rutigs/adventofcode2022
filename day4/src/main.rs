use std::fs::File;
use std::env;
use std::process;
use std::io::{BufRead, BufReader};
use std::cmp;

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

    let mut count: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim().eq("") {
           continue;
        }

        let mut assignment_pairs = line.split(",");
        let mut assignment_one = assignment_pairs.next().unwrap().split("-");
        let mut assignment_two = assignment_pairs.next().unwrap().split("-");

        let a1_start: i32 = assignment_one.next().unwrap().parse().unwrap();
        let a1_end: i32 = assignment_one.next().unwrap().parse().unwrap();

        let a2_start: i32 = assignment_two.next().unwrap().parse().unwrap();
        let a2_end: i32 = assignment_two.next().unwrap().parse().unwrap();

        // part 1
        // if contains(a1_start, a1_end, a2_start, a2_end) || contains(a2_start, a2_end, a1_start, a1_end) {
        //     count += 1;
        // }
        
        if cmp::max(a1_start, a2_start) <= cmp::min(a1_end, a2_end) {
            count += 1;
        }
    }

    println!("Total overlapping assignments: {}", count);
}

// part 1
// check if one falls within two
// fn contains(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
//     return start1 >= start2 && end1 <= end2;
// }
