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
    let pattern = Regex::new(r"(do\(\)|don't\(\))|mul\((\d+),(\d+)\)").unwrap();
    
    let mut enable = true;
    let mut res = 0;
    for cap in pattern.captures_iter(file) {
        match cap.get(1).map(|m| m.as_str()) {
            Some("do()") => enable = true,
            Some("don't()") => enable = false,
            _ => match (cap.get(2).map(|m| m.as_str()), cap.get(3).map(|m| m.as_str())) {
                (Some(a), Some(b)) => {
                    if enable {
                        res += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
                    }
                }
                _ => (),
            },      
        }
    }
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
