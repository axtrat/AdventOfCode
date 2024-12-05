use std::io::stdin;

fn is_corret(rules: &Vec<(i32, i32)>, manual: &Vec<i32>) -> bool {
    for (x, y) in rules {
        if manual.contains(x) && manual[..manual.iter().position(|page| page == x).unwrap()].contains(y) {
            return false;
        }
    }
    return true;
}


fn part1(rules: &Vec<(i32, i32)>, manuals: &Vec<Vec<i32>>) {
    let res = manuals
        .iter()
        .filter(|manual| is_corret(&rules, manual))
        .map(|manual| manual[manual.len()/2])
        .sum::<i32>();

    println!("{}", res);
}

fn correct(rules: &Vec<(i32, i32)>, manual: &mut Vec<i32>) {
    let mut i = 0;
    while i < manual.len() {
        for (x, y) in rules {
            if manual[i] == *x && manual[..i].contains(y) {
                let j = manual.iter().position(|page| page == y).unwrap();
                let page = manual.remove(j);
                manual.insert(i, page);
                i -= 1;
            }
        }
        i += 1;            
    }
}

fn part2(rules: &Vec<(i32, i32)>, manuals: &Vec<Vec<i32>>) {
    let to_correct: Vec<&Vec<i32>> = manuals
        .iter()
        .filter(|manual| !is_corret(&rules, manual))
        .collect();
    let mut corrected = Vec::new();
    for manual in &to_correct {
        let mut tmp : Vec<i32> = manual.to_vec();
        correct(&rules, &mut tmp);
        while !is_corret(rules, &tmp) {
            println!("{:?}", tmp);
            correct(rules, &mut tmp); 
        }
        corrected.push(tmp);
    }

    let res = corrected
        .iter()
        .map(|manual| manual[manual.len()/2])
        .sum::<i32>();

    println!("{}", res);
}


fn main() {
    let mut file = String::new();
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => file.push_str(&line),
            Err(_) => break,
        }
    }
    let mut parts = file.split("\n\n");
    let rules = parts.next().unwrap();
    let manuals = parts.next().unwrap();

    

    let rules: Vec<(i32, i32)> = rules
        .split("\n")
        .map(|rule| {
            let mut parts = rule
                .split("|")
                .map(|x| x.parse::<i32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        }
        )
        .collect();
    
    let manuals = manuals
        .split("\n")
        .map(|x| x
            .split(",")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>();
    part1(&rules, &manuals);
    part2(&rules, &manuals);
}
