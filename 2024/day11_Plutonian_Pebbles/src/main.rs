use std::env::args;
use std::fs::read_to_string;

const TIMES: usize = 25;

fn read_lines() -> Vec<u64> {
    let path = args().nth(1).expect("No args given");
    read_to_string(path).expect("Failed to read file")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(line: &Vec<u64>) {
    let mut line = line.clone();
    for i in 0..TIMES {
        let mut new_line = Vec::new();
        for stone in &line {
            let len = format!("{stone}").len() as u32;
            if *stone == 0 {
                new_line.push(1);
            } else if len % 2 == 0 {
                let pow  =10_u64.pow(len/2);
                new_line.push(stone/pow);
                new_line.push(stone%pow);
            } else {
                new_line.push(stone*2024);
            }
        }
        //println!("{:?}", new_line);
        println!("{}: {}", i, new_line.len());
        line = new_line;
    }
    println!("Part 1: {}", line.len());
}

fn main() {
    let file = read_lines();

    part1(&file);

}