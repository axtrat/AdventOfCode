use std::io::stdin;

fn is_safe(report: &Vec<i32>) -> bool {
    let sign = if report[0] < report[1] { -1 } else { 1 };
    for i in 0..report.len()-1 {
        let diff = (report[i] - report[i+1]) * sign;
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn part1(reports: &Vec<Vec<i32>>) {
    let res = reports
        .iter()
        .filter(|report| is_safe(report))
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
    //part2(&file);
}
