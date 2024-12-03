use std::io::stdin;

fn is_safe(report: &Vec<i32>, tollerate: bool) -> bool {
    let sign = if report[0] < report[1] { -1 } else { 1 };
    for i in 0..report.len()-1 {
        let diff = (report[i] - report[i+1]) * sign;
        if diff < 1 || diff > 3 {
            if !tollerate {
                return false;
            }
            return is_safe(&report[1..].to_vec(), false) || is_safe(&[&report[..i], &report[i+1..]].concat(), false) || is_safe(&[&report[..i+1], &report[i+2..]].concat(), false);
        }
    }
    return true;
}

fn part1(reports: &Vec<Vec<i32>>) {
    let res = reports
        .iter()
        .filter(|report| is_safe(report, false))
        .count();
    println!("{}", res);
}

fn part2(reports: &Vec<Vec<i32>>) {
    let res = reports
        .iter()
        .filter(|report| is_safe(report, true))
        .count();
    println!("{}", res);
}

fn main() {
    let mut reports = Vec::new();
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => reports.push(line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
            ),
            Err(_) => break,
        }
    }
        

    part1(&reports);
    part2(&reports);
}
