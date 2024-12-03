use std::io::stdin;


fn part1(file: &Vec<String>) {
    let pairs: Vec<(i32, i32)> = file
        .iter()
        .map(|line| {
            let mut iter = line.split("   ")
                .map(|x| {
                    x.parse::<i32>().unwrap()
                });
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect();

    let (mut l1, mut l2): (Vec<i32>, Vec<i32>) = pairs.iter().cloned().unzip();
    l1.sort();
    l2.sort();


    let res = l1.iter().zip(l2.iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<i32>();

    println!("{res}");
}

fn part2(file: &Vec<String>) {
    todo!();
}

fn main() {
    let mut file = Vec::new();
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => file.push(line.trim().to_string()),
            Err(_) => break,
        }
    }

    part1(&file);
    part2(&file);
}
