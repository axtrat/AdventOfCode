use std::cell;
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

fn read_lines() -> Vec<String> {
    let path = args().nth(1).expect("No args given");
    let file = read_to_string(path).expect("Failed to read file");
    file.lines().map(|line| line.to_string()).collect()
}

fn find_zeros(map: &Vec<Vec<u32>>) -> Vec<Point> {
    let mut zeros = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                zeros.push(Point { x: i as i32, y: j as i32 });
            }
        }
    }
    zeros
}

const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];

fn trailheads(map: &Vec<Vec<u32>>, curr: Point) -> HashSet<Point> {
    let mut set: HashSet<Point>= HashSet::new();
    if map[curr.x as usize][curr.y as usize] == 9 {
        set.insert(curr);        
        return set;
    }

    //println!("{}, {} => {}", curr.x, curr.y, map[curr.x as usize][curr.y as usize]);

    DIRECTIONS
        .iter()
        .filter_map(|dir| {
            let new = *dir+curr;
            let curr = map[curr.x as usize][curr.y as usize];
            map
                .get(new.x as usize)
                .and_then(|row| row
                    .get(new.y as usize)
                ).filter(|&&cell| cell == curr+1)
                .map(|_| trailheads(map, new))
        })
        .for_each(|res| {
            res.iter().for_each(|pos| {
                set.insert(*pos);
            });
        });

    set
    
}

fn part1(map: &Vec<Vec<u32>>) {
    let zeros = find_zeros(map);
    let sum: u32 = zeros
        .iter()
        .map(|&pos| trailheads(map, pos).len() as u32)
        .sum();
    println!("{sum}");
}

fn main() {
    let file = read_lines();
    let map: Vec<Vec<u32>> = file
        .iter()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
        )
        .collect();
    part1(&map)
}