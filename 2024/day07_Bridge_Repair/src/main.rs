use std::env::args;
use std::fs::read_to_string;

fn read_lines() -> Vec<String> {
    let path = args().nth(1).expect("No args given");
    let file = read_to_string(path + ".txt").expect("Failed to read file");
    file.lines().map(|line| line.to_string()).collect()
}

fn test(res: &u64, numbers: &[u64], part2: bool) -> bool {
    if numbers.len() == 1 {
        return *res == numbers[0];
    }

    let num = numbers[numbers.len() - 1];
    let numbers = &numbers[..numbers.len() - 1];

    if *res < num {
        return false;
    }
    
    let pow: u64  =10_u64.pow(format!("{num}").len() as u32);

    part2 && res % pow == num && test(&(res / pow), numbers, true)
        ||   res % num == 0   && test(&(res / num), &numbers, part2)
        ||   test(&(res - num), &numbers, part2)
}

fn part1(res: &Vec<u64>, numbers: &Vec<Vec<u64>>) {
    let mut sum = 0;
    for (i, res) in res.iter().enumerate() {
        if test(&res, &numbers[i], false) {
            sum += res;
        }
    }
    println!("Part 1: {}", sum);
}

fn part2(res: &Vec<u64>, numbers: &Vec<Vec<u64>>) {
    let mut sum = 0;
    for (i, res) in res.iter().enumerate() {
        if test(&res, &numbers[i], true) {
            sum += res;
        }
    }
    println!("Part 1: {}", sum);
}

fn main() {
    let file = read_lines();

    let mut res: Vec<u64> = Vec::new();
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    for line in &file {
        let tmp = line.split(": ").collect::<Vec<&str>>();
        res.push(tmp[0].parse().unwrap());
        numbers.push(tmp[1].split(" ").map(|x| x.parse().unwrap()).collect());
    }

    part1(&res, &numbers);
    part2(&res, &numbers);
}
