use std::io::stdin;
//use regex
use regex::Regex;


fn part1(file: &String) {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let res = pattern
        .captures_iter(file)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        })
        .sum::<i32>();
    println!("{}", res);
}

fn part2(file: &String) {
    let pattern = Regex::new(r"(do(?:n't)?\(\))|mul\((\d+),(\d+)\)").unwrap();
    
    let mut enable = true;
    let res = pattern
        .captures_iter(file)
        .map(|cap| {
            match cap.get(1).map(|m| m.as_str()) {
                Some("do()") => {
                    enable = true;
                    0
                }
                Some("don't()") => {
                    enable = false;
                    0
                }
                _ => if enable {
                    let a = cap[2].parse::<i32>().unwrap();
                    let b = cap[3].parse::<i32>().unwrap();
                    a * b
                } else { 0 },      
            }
        })
        .sum::<i32>();
            
    println!("{}", res);
}
    

fn main() {
    let mut file = String::new();
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => file += &line,
            Err(_) => break,
        }
    }
    part1(&file);
    part2(&file);
}
