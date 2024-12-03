use std::io::stdin;
use std::collections::HashMap;


fn part1(pairs: &Vec<(i32, i32)> ) {
    let (mut l1, mut l2): (Vec<i32>, Vec<i32>) = pairs
        .iter()
        .cloned()
        .unzip();

    l1.sort();
    l2.sort();


    let res = l1
        .iter()
        .zip(l2.iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<i32>();

    println!("{res}");
}

fn part2(pairs: &Vec<(i32, i32)> ) {
    let (l1, l2): (Vec<i32>, Vec<i32>) = pairs
        .iter()
        .cloned()
        .unzip();

    
    let mut dict: HashMap<i32, i32> = HashMap::new();
    for num in l2 {
        dict.insert(num, dict.get(&num).unwrap_or(&0) + 1);
    }

    let res: i32 = l1.iter()
        .filter(|num| dict.contains_key(num))
        .map(|num| num * dict.get(num).unwrap())
        .sum();
        
    println!("{res}");
}

fn main() {
    let mut pairs = Vec::new();
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let mut iter = line.trim().split("   ")
                .map(|x| {
                    x.parse::<i32>().unwrap()
                });
            
                pairs.push((iter.next().unwrap(), iter.next().unwrap()));
            },
            Err(_) => break,
        }
    }

    part1(&pairs);
    part2(&pairs);
}
