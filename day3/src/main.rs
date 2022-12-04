use std::fs::File;
use std::env;
use std::process;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    println!("Received {} arguments", args.len());

    if args.len() < 2 {
        println!("missing file path argument!");
        process::exit(1);
    }

    let mut priority_values: HashMap<char, usize> = HashMap::new();
    for i in 0..26 {
        let lower_letter = char::from_u32('a' as u32 + i).unwrap();
        let upper_letter = lower_letter.to_ascii_uppercase();
        priority_values.insert(lower_letter, (i + 1) as usize);
        priority_values.insert(upper_letter, (i + 27) as usize);
    }

    let file_name = &args[1];
    let file = File::open(file_name).unwrap(); // ignore errors from Result type
    let reader = BufReader::new(file);

    let mut frequency_map: HashMap<char, i32> = HashMap::new();
    let mut group: Vec<HashSet<char>> = Vec::with_capacity(3);

    for line in reader.lines() {
        let line = line.unwrap();

        let new_set: HashSet<char> = HashSet::from_iter(line.chars());
        group.push(new_set);

        if group.len() < 3 {
            continue;
        }

        let elf_one = group.get(0).unwrap();
        let elf_two = group.get(1).unwrap();
        let elf_three = group.get(2).unwrap();

        let seen_set: HashSet<char> = elf_one.intersection(&elf_two).cloned().collect();
        let seen_set: HashSet<char> = seen_set.intersection(&elf_three).cloned().collect();

        assert!(seen_set.len() == 1);
        for c in seen_set.into_iter() {
            let priority_sum = priority_values[&c];
            if frequency_map.contains_key(&c) {
                let curr = frequency_map.get(&c).unwrap();
                frequency_map.insert(c, curr + priority_sum as i32);
            } else {
                frequency_map.insert(c, priority_sum as i32); 
            }
        }

        group.clear();
    }

    let mut total_sum: i32 = 0;
    for (_, freq) in frequency_map.into_iter() {
        total_sum += freq as i32;
    }

    println!("Total sum: {}", total_sum);
}
