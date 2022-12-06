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

    let mut crate_stacks: Vec<Vec<char>> = Vec::with_capacity(9);
    for _ in 0..9 {
        crate_stacks.insert(0, Vec::new());
    }

    let mut file_index = 0;
    let crate_delim: usize = 3;

    for line in reader.lines() {
        let line = line.unwrap();
        
        if file_index <= 7 {
            let chars: Vec<char> = line.chars().collect();

            for i in 0..9 {
                let crate_val = chars[1 + i + crate_delim*i];
                match crate_val == ' ' {
                    true => continue,
                    false => crate_stacks[i].insert(0, crate_val),
                }
            }
        }

        if file_index > 9 {
            let line_items: Vec<&str> = line.split(" ").collect();
            let stack_items = (*line_items[1]).parse::<usize>().unwrap();
            let from_index = (*line_items[3]).parse::<usize>().unwrap();
            let to_index = (*line_items[5]).parse::<usize>().unwrap();

            let mut temp_stack: Vec<char> = Vec::new();
            for _ in 0..stack_items {
                let temp_crate = crate_stacks[from_index-1].pop().unwrap();
                temp_stack.insert(0, temp_crate);

                // part 1
                // let temp = crate_stacks[from_index-1].pop().unwrap();
                // crate_stacks[to_index-1].push(temp);
            }
            crate_stacks[to_index-1].extend(temp_stack);
        }

        file_index += 1;
    }

    print_crates(&crate_stacks);
}

fn print_crates(crates: &Vec<Vec<char>>) {
    println!("crate stacks 1: {:?}", crates[0]);
    println!("crate stacks 2: {:?}", crates[1]);
    println!("crate stacks 3: {:?}", crates[2]);
    println!("crate stacks 4: {:?}", crates[3]);
    println!("crate stacks 5: {:?}", crates[4]);
    println!("crate stacks 6: {:?}", crates[5]);
    println!("crate stacks 7: {:?}", crates[6]);
    println!("crate stacks 8: {:?}", crates[7]);
    println!("crate stacks 9: {:?}", crates[8]);

}
