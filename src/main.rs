extern crate rand;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(arg) => {
            if arg == "gen" {
                generate_test_data();
            }
        },
        None => {
            calculate_sum()
        },
    }
}

fn generate_test_data() {
    let mut rng = rand::thread_rng();
    let mut f = File::create("data").unwrap();
    for i in 1..1000 {
        println!("{} of {}", i, 1000);
        let mut line_str = String::new();
        for _ in 0..i {
            let num: u16 = rng.gen();
            line_str.push_str(&num.to_string());
            line_str.push_str(" ");
        }
        line_str.pop(); //remove the last space
        line_str.push('\n');
        f.write(line_str.as_bytes()).unwrap();
    }
    f.flush().unwrap();
}

fn calculate_sum() {
    let f = File::open("data").unwrap();
    let buffer = BufReader::new(f);
    let mut tree: Vec<Vec<u16>> = Vec::new();
    for line in buffer.lines() {
        let mut tree_row: Vec<u16> = Vec::new();
        for num_str in line.unwrap().split(" ").collect::<Vec<&str>>() {
            tree_row.push(num_str.parse::<u16>().unwrap());
        }
        tree.push(tree_row);
    }
}
