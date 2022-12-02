use std::collections::BinaryHeap;
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

    let mut acc = 0;
    let mut max_cals_carried = 0;
    let mut calories_list = BinaryHeap::new(); 

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);

        if line.trim().eq("") {
            if acc > max_cals_carried {
                max_cals_carried = acc;
            }
            calories_list.push(acc);
            acc = 0;
            continue;
        }

        match line.parse::<i32>() {
            Ok(cals) => acc += cals,
            Err(e) => println!("Invalid line input {}", e),
        }
    }

    if acc > max_cals_carried {
        max_cals_carried = acc;
    }

    // use unwrap to ignore the option type
    let elf_one = calories_list.pop().unwrap();
    let elf_two = calories_list.pop().unwrap();
    let elf_three = calories_list.pop().unwrap();

    println!("Max cals carried is {}", max_cals_carried);
    println!("3 Top elves are carrying: {}, {}, {}", elf_one, elf_two, elf_three);
    println!("Together they are carrying {}", elf_one + elf_two + elf_three);
}
