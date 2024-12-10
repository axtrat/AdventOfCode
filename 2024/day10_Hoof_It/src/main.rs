use std::collections::HashSet;
use std::env::args;
use std::fs::read_to_string;

type Point = (usize, usize);

fn read_lines() -> Vec<String> {
    let path = args().nth(1).expect("No args given");
    let file = read_to_string(path).expect("Failed to read file");
    file.lines().map(|line| line.to_string()).collect()
}

fn find_zeros(map: &Vec<Vec<usize>>) -> Vec<Point> {
    let mut zeros = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                zeros.push((i, j));
            }
        }
    }
    zeros
}

const DIRECTIONS: [(isize, isize); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

fn trailheads_score(map: &Vec<Vec<usize>>, curr: Point) -> HashSet<Point> {
    let mut set: HashSet<Point> = HashSet::new();
    if map[curr.0][curr.1] == 9 {
        set.insert(curr);
        return set;
    }

    DIRECTIONS
        .iter()
        .filter_map(|dir| {
            let new = (
                (curr.0 as isize + dir.0) as usize,
                (curr.1 as isize + dir.1) as usize,
            );
            let curr_val = map[curr.0][curr.1];
            map
                .get(new.0)
                .and_then(|row| row.get(new.1))
                .filter(|&&cell| cell == curr_val + 1)
                .map(|_| trailheads_score(map, new))
        })
        .for_each(|res| {
            res.iter().for_each(|&pos| {
                set.insert(pos);
            });
        });

    set
}

fn trailheads_rating(map: &Vec<Vec<usize>>, curr: Point) -> usize {
    if map[curr.0][curr.1] == 9 {
        return 1;
    }

    DIRECTIONS
        .iter()
        .filter_map(|dir| {
            let new = (
                (curr.0 as isize + dir.0) as usize,
                (curr.1 as isize + dir.1) as usize,
            );
            let curr_val = map[curr.0][curr.1];
            map
                .get(new.0)
                .and_then(|row| row.get(new.1))
                .filter(|&&cell| cell == curr_val + 1)
                .map(|_| trailheads_rating(map, new))
        })
        .sum()
}

fn main() {
    let file = read_lines();
    let map: Vec<Vec<usize>> = file
        .iter()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
        )
        .collect();

    let zeros = find_zeros(&map);
    let part1: usize = zeros
        .iter()
        .map(|&pos| trailheads_score(&map, pos).len())
        .sum();
    println!("{part1}");
    
    let part2: usize = zeros
        .iter()
        .map(|&pos| trailheads_rating(&map, pos))
        .sum();
    println!("{part2}");
}
