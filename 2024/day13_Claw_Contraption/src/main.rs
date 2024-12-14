use std::env::args;
use std::fs::read_to_string;

fn read_lines() -> Vec<String> {
    let path = args().nth(1).expect("No args given");
    let file = read_to_string(path).expect("Failed to read file");
    file.lines().map(|line| line.trim().to_string()).collect()
}

fn part1(file: &Vec<Vec<i32>>) {
    let mut sum = 0;
    for i in (0..file.len()).step_by(3) {
        let (ax, ay) = (&file[i][0], &file[i][1]);
        let (bx, by) = (&file[i+1][0], &file[i+1][1]);
        let (cx, cy) = (&file[i+2][0], &file[i+2][1]);

        let mut a = 0;
        let b = loop {
            let [ax, ay] = [ax * a, ay * a];
            let [dx, dy] = [cx - ax, cy - ay];
            if dx%bx == 0 && dy == dx/bx * by{
                break dx/bx
            }

            if ax > *cx || ay > *cy {
                break -1;
            }
            a+=1;
        };

        if b == -1 || a > 100 || b > 100 {
            continue;
        }
        sum += 3*a+b;
    }
    println!("{}", sum);
}

fn main() {
    let file: Vec<Vec<i32>> = read_lines()
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line
            .to_string()
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|x| x[2..].parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>();
    
    
    part1(&file);
}