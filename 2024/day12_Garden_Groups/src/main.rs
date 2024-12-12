use std::collections::{HashMap, HashSet, VecDeque};
use std::env::args;
use std::fs::read_to_string;

fn read_lines() -> Vec<String> {
    let path = args().nth(1).expect("No args given");
    let file = read_to_string(path).expect("Failed to read file");
    file.lines().map(|line| line.to_string()).collect()
}

const DIRECTIONS: [(isize, isize); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

fn bfs(file: &[Vec<char>], start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut region: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back(start);
    region.insert(start);

    let plant = file[start.0][start.1];
    let mut adiacent = 0;

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for (dx, dy) in DIRECTIONS.iter() {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if !((0..file.len() as isize).contains(&nx) && (0..file[0].len() as isize).contains(&ny)) {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if plant != file[nx][ny] { continue;}
            
            adiacent += 1;
            if region.contains(&(nx, ny)) {
                continue;
            }
            region.insert((nx, ny));
            queue.push_back((nx, ny));
        }
    }
    region
}

fn bfs2(outside: &HashMap<(usize, usize), usize>, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut region: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back(start);
    region.insert(start);

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for (dx, dy) in DIRECTIONS.iter() {
            let (nx, ny) = (x as isize + dx, y as isize + dy);  
            if !outside.contains_key(&(nx as usize, ny as usize)) {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);

           
            if region.contains(&(nx, ny)) {
                continue;
            }
            region.insert((nx, ny));
            queue.push_back((nx, ny));
        }
    }
    region
}

fn sides(region: &HashSet<(usize, usize)>) -> usize {
    let mut outside: HashMap<(usize, usize), usize> = HashMap::new();
    for (x, y) in region {
        for (dx, dy) in DIRECTIONS.iter() {
            let (nx, ny) = (*x as isize + dx, *y as isize + dy);
            if !region.contains(&(nx as usize, ny as usize)) {
                *outside.entry((nx as usize, ny as usize)).or_insert(0) += 1;
            }
        }
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut count = 0;
    for pos in outside.keys() {
        if visited.contains(&pos) {
            continue;
        }
        let region = bfs2(&outside, *pos);
        visited.extend(region.iter());

        let side = outside
        .iter()
        .filter(|(pos, _)| region.contains(pos))
        .map(|(_, &val)| val)
        .max()
        .unwrap();
        count += 1;
    }
    println!("____{:?}", count,);
    count
}

fn perimeter(region: &HashSet<(usize, usize)>) -> usize {
    region.iter().map(|&(x, y)| {
        DIRECTIONS.iter().filter(|(dx, dy)| {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            !region.contains(&(nx as usize, ny as usize))
        }).count()
    }).sum()
}


fn main() {
    let file: Vec<Vec<char>> = read_lines()
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for (i, line) in file.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            if visited.contains(&(i, j)) {
                continue;
            }
            let region = bfs(&file, (i, j));
            visited.extend(region.iter());
            part1 += region.len() * perimeter(&region);
            part2 += region.len() * sides(&region);
        }
    }
    println!("{}", part1);
    println!("{}", part2);
}