use std::env::args;
use std::fs::read_to_string;

struct File {
    id: u64,
    size: u64,
    free_space: u64
}

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

fn print_mem(mem: &Vec<File>) {
    for file in mem.iter() {
        print!("{}{}", format!("{:x}", file.id).repeat(file.size as usize), ".".repeat(file.free_space as usize));
    }
    print!("\n");
}

fn part2(data: &Vec<u64>) {
    let mut mem: Vec<File> = Vec::new();
    for i in (0..data.len()).step_by(2) {
        mem.push(File {
            id: i as u64/2,
            size: data[i],
            free_space: if i+1 < data.len() {data[i+1]} else {0}
        });
    }

    let mut j = mem.len()-1;
    loop {
        //print_mem(&mem);
        for k in 0..j {
            //println!("{}: {} -> {}: {}",j, mem[j].size, k, mem[k].free_space);
            if mem[k].free_space >= mem[j].size {
                mem[j-1].free_space += mem[j].size+mem[j].free_space;
                mem[j].free_space = mem[k].free_space - mem[j].size;
                mem[k].free_space = 0;
                let tmp = mem.remove(j);
                mem.insert(k+1, tmp);
                j+=1;
                break;
            }
        }

        if j == 0 {
            break;
        }
        j-=1;
    }
    
    let mut i = 0;
    let mut sum = 0;
    for file in mem.iter() {
        sum += (i..i+file.size)
        .map(|j| j as u64 * file.id)
        .sum::<u64>();
        i += file.size + file.free_space;
    }
    println!("{:?}", sum);
}

fn main() {
    let data: Vec<u64> = read_lines()[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    part1(&data);
    part2(&data);
}