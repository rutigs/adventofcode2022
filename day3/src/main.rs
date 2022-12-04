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
    priority_values.insert('a', 1); 
    priority_values.insert('b', 2);
    priority_values.insert('c', 3);
    priority_values.insert('d', 4);
    priority_values.insert('e', 5);
    priority_values.insert('f', 6);
    priority_values.insert('g', 7);
    priority_values.insert('h', 8);
    priority_values.insert('i', 9);
    priority_values.insert('j', 10); 
    priority_values.insert('k', 11);
    priority_values.insert('l', 12);
    priority_values.insert('m', 13);
    priority_values.insert('n', 14);
    priority_values.insert('o', 15);
    priority_values.insert('p', 16);
    priority_values.insert('q', 17);
    priority_values.insert('r', 18);
    priority_values.insert('s', 19);
    priority_values.insert('t', 20);
    priority_values.insert('u', 21);
    priority_values.insert('v', 22);
    priority_values.insert('w', 23);
    priority_values.insert('x', 24);
    priority_values.insert('y', 25);
    priority_values.insert('z', 26);
    priority_values.insert('A', 27);
    priority_values.insert('B', 28);
    priority_values.insert('C', 29);
    priority_values.insert('D', 30);
    priority_values.insert('E', 31);
    priority_values.insert('F', 32);
    priority_values.insert('G', 33);
    priority_values.insert('H', 34);
    priority_values.insert('I', 35);
    priority_values.insert('J', 36);
    priority_values.insert('K', 37);
    priority_values.insert('L', 38);
    priority_values.insert('M', 39);
    priority_values.insert('N', 40);
    priority_values.insert('O', 41);
    priority_values.insert('P', 42);
    priority_values.insert('Q', 43);
    priority_values.insert('R', 44);
    priority_values.insert('S', 45);
    priority_values.insert('T', 46);
    priority_values.insert('U', 47);
    priority_values.insert('V', 48);
    priority_values.insert('W', 49);
    priority_values.insert('X', 50);
    priority_values.insert('Y', 51);
    priority_values.insert('Z', 52);

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