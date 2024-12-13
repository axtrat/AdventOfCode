use std::collections::{HashMap, HashSet, VecDeque};
use std::env::args;
use std::fs::read_to_string;

type Point = (isize, isize);

fn read_lines() -> Vec<String> {
    let path = args().nth(1).expect("No args given");
    let file = read_to_string(path).expect("Failed to read file");
    file.lines().map(|line| line.to_string()).collect()
}

const DIRECTIONS: [Point; 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

const ANGLES: [Point; 4] = [
    (0, 0),
    (1, 0),
    (0, 1),
    (1, 1),
];

fn bfs(file: &[Vec<char>], start: Point) -> HashSet<Point> {
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut region: HashSet<Point> = HashSet::new();
    queue.push_back(start);
    region.insert(start);

    let plant = file[start.0 as usize][start.1 as usize];

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for (dx, dy) in DIRECTIONS.iter() {
            let (nx, ny) = (x + dx, y + dy);
            if !((0..file.len() as isize).contains(&nx) && (0..file[0].len() as isize).contains(&ny)) {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if plant != file[nx][ny] { continue;}
            
            let point = (nx as isize, ny as isize);
            if region.contains(&point) {
                continue;
            }
            region.insert(point);
            queue.push_back(point);
        }
    }
    region
}

fn sides(region: &HashSet<Point>) -> usize {
    let mut outside: HashMap<Point, usize> = HashMap::new();
    let mut edges: usize = 0;
    for &(x, y) in region {
        for (dx, dy) in ANGLES.iter() {
            let (nx, ny) = (x + dx, y + dy);
            if outside.contains_key(&(nx, ny)) {
                if !((region.contains(&(nx, ny)) || region.contains(&(nx-1, ny-1))) && (region.contains(&(nx-1, ny)) || region.contains(&(nx, ny-1)))) {
                    edges += 2;
                }
            }
            *outside.entry((nx, ny)).or_insert(0) += 1;
        }
    }

    let count: usize = outside
        .iter()
        .map(|(_, &val)| val % 2)
        .sum();

    count + edges
}

fn perimeter(region: &HashSet<Point>) -> usize {
    region.iter().map(|&(x, y)| {
        DIRECTIONS.iter().filter(|(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            !region.contains(&(nx, ny))
        }).count()
    }).sum()
}

fn main() {
    let file: Vec<Vec<char>> = read_lines()
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut visited: HashSet<Point> = HashSet::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for (i, line) in file.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            let point = (i as isize, j as isize);
            if visited.contains(&point) {
                continue;
            }
            let region = bfs(&file, point);
            visited.extend(region.iter());
            part1 += region.len() * perimeter(&region);
            part2 += region.len() * sides(&region);
        }
    }
    println!("{}", part1);
    println!("{}", part2);
}