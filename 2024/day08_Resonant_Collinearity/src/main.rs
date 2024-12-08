use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::read_to_string;



fn part1(antennas: &HashMap<char, Vec<(usize, usize)>>, rows: usize, cols: usize) {
    let mut antinodes =  HashSet::new();
    for (_, antennas) in antennas {
        for i in 0..antennas.len() {
            for j in i+1..antennas.len() {
                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];

                let (dx, dy) = (x2 as isize - x1 as isize, y2 as isize - y1 as isize);
                let nodes = [(x2 as isize +dx, y2 as isize+dy), (x1 as isize-dx, y1 as isize-dy)];

                for node in nodes {
                    if 0 <= node.0 && node.0 < rows as isize && 0<= node.1 && node.1 < cols as isize {
                        antinodes.insert(node);
                    }
                }
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
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, line) in file.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert(Vec::new()).push((i, j));
            }
        }
    }

    part1(&antennas, file.len(), file[0].len());
}