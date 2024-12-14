use std::collections::HashSet;
use std::env::args;
use std::fs::read_to_string;

fn read_lines() -> Vec<String> {
    let path = args().nth(1).expect("No args given");
    let file = read_to_string(path).expect("Failed to read file");
    file.lines().map(|line| line.to_string()).collect()
}

fn part1(file: &Vec<Vec<(i32, i32)>>) {
    let times = 100;
    let (wide, tall) = (101, 103);
    let mut q = [0;4];
    for robot in file.iter() {
        let (px, py) = robot[0];
        let (vx, vy) = robot[1];
        let (nx, ny) = (px+times*vx, py+times*vy);
        let (nx, ny) = (nx.rem_euclid(wide), ny.rem_euclid(tall));
        
        
        if nx < wide/2 {
            if ny < tall/2 {
                q[0]+=1;
            }else if ny != tall/2 {
                q[1]+=1;
            }
        } else if nx != wide/2 {
            if ny < tall/2 {
                q[2]+=1;
            }else if ny != tall/2 {
                q[3]+=1;
            }
        }

    }
    println!("{}", q.iter().product::<i32>())
}

fn _print_mat(robots: HashSet<(i32, i32)>, wide: &i32, tall: &i32) {
    for i in 0..*wide {
        for j in 0..*tall {
            if robots.contains(&(i, j)) {
                print!("R")
            } else {
                print!(".")
            }
        }
        println!()
    };
}

fn part2(file: &Vec<Vec<(i32, i32)>>) {
    let (wide, tall) = (101, 103);
    for times in 1..10_000 {
        let robots = file
            .iter()
            .map(|robot: &Vec<(i32, i32)>| {
                let (px, py) = robot[0];
                let (vx, vy) = robot[1];
                let (nx, ny) = (px+times*vx, py+times*vy);
                (nx.rem_euclid(wide), ny.rem_euclid(tall))
            })
            .collect::<HashSet<(i32, i32)>>();
        let avgx = robots
            .iter()
            .map(|(x, _)| x)
            .sum::<i32>()/robots.len() as i32;
        let avgy = robots
            .iter()
            .map(|(_, y)| y)
            .sum::<i32>()/robots.len() as i32;
        let varx = robots
            .iter()
            .map(|(x, _)| (x-avgx).pow(2))
            .sum::<i32>()/robots.len() as i32;
        let vary = robots
            .iter()
            .map(|(_, y)| (y-avgy).pow(2))
            .sum::<i32>()/robots.len() as i32;
        if varx < 746 || varx > 975 {
            if vary < 746 || vary > 975 {
                _print_mat(robots, &wide, &tall);
                println!("{times}");
            }
        }
    }
}



fn main() {
    let file: Vec<Vec<(i32, i32)>> = read_lines()
        .iter()
        .map(|line| line
            .split_whitespace()
            .map(|part| {
                let mut split = part[2..]
                .split(",")
                .map(|num|num.parse::<i32>().unwrap());
                (split.next().unwrap(), split.next().unwrap())
            })
            .collect::<Vec<(i32, i32)>>()
        )
        .collect::<Vec<Vec<(i32, i32)>>>();

    part1(&file);
    part2(&file);
}