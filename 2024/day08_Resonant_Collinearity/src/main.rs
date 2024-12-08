use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::read_to_string;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn combinations<T>(v: &Vec<T>) -> Vec<(T, T)> where T: Copy {
    let mut result = Vec::new();
    for i in 0..v.len() {
        for j in i+1..v.len() {
            result.push((v[i], v[j]));
        }
    }
    result
}

fn part1(antennas: &HashMap<char, Vec<Point>>, rows: i32, cols: i32) {
    let mut antinodes = HashSet::new();
    for (_, antennas) in antennas {
        for (p1, p2) in combinations(antennas) {
            let delta = p2 - p1;
            let nodes = [p2 + delta, p1 - delta];

            for node in nodes {
                if (0..rows).contains(&node.x) && (0..cols).contains(&node.y) {
                    antinodes.insert(node);
                }
            }
        }
    }
    println!("{}", antinodes.len());
}

fn part2(antennas: &HashMap<char, Vec<Point>>, rows: i32, cols: i32) {
    let mut antinodes = HashSet::new();
    for (_, antennas) in antennas {
        for (p1, p2) in combinations(antennas) {
            let delta = p2 - p1;
            let mut node = p2;

            while (0..rows).contains(&node.x) && (0..cols).contains(&node.y) {
                antinodes.insert(node.clone());
                node = node + delta;
            }

            node = p1;
            while (0..rows).contains(&node.x) && (0..cols).contains(&node.y) {
                antinodes.insert(node.clone());
                node = node - delta;
            }
        }
    }
    println!("{}", antinodes.len());
}

fn read_lines() -> Vec<String> {
    let path = args().nth(1).expect("No args given");
    let file = read_to_string(path).expect("Failed to read file");
    file.lines().map(|line| line.to_string()).collect()
}

fn main() {
    let file = read_lines();
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for (i, line) in file.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert(Vec::new()).push(Point { x: i as i32, y: j as i32 });
            }
        }
    }

    part1(&antennas, file.len() as i32, file[0].len() as i32);
    part2(&antennas, file.len() as i32, file[0].len() as i32);
}