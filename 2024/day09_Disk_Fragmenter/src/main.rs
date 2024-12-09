use std::env::args;
use std::fs::read_to_string;

fn read_lines() -> Vec<String> {
    let path = args().nth(1).expect("No args given");
    let file = read_to_string(path).expect("Failed to read file");
    file.lines().map(|line| line.to_string()).collect()
}

fn part1(data: &Vec<u64>) {
    let mut mem: Vec<Option<u64>> = Vec::new();
    let mut is_file = true;
    for (id, size) in data.iter().enumerate() {
        for _ in 0..*size {
            if is_file {
                mem.push(Some(id as u64/2));
            } else {
                mem.push(None);
            }
        }
        is_file = !is_file;
    }
    
    let (mut i, mut j) = (0, mem.len()-1);
    while i < j {
        match (mem[i], mem[j]) {
            (Some(_), _) => i+=1,
            (_, None) => j-=1,
            (None, id) => (mem[i], mem[j]) = (id, None)
        }
    }
    let sum: u64 = mem
        .iter()
        .enumerate()
        .filter_map(|(i, id)| id.map(|x| i as u64 * x))
        .sum();
    println!("{:?}", sum);
}

fn main() {
    let data: Vec<u64> = read_lines()[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    part1(&data)
}