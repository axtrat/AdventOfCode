use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

struct Guard {
    x: i32,
    y: i32,
    direction: usize,
}

impl Clone for Guard {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            direction: self.direction,
        }
    }
}

//turn right
impl Guard {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            direction: 0,
        }
    }

    fn next(&self) -> (i32, i32) {
        let (dx, dy) = DIRECTIONS[self.direction];
        (self.x + dx, self.y + dy)
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn turn_right(&mut self) {
        self.direction = (self.direction + 1) % 4;
    }

    fn move_forwards(&mut self) {
        (self.x, self.y) = self.next();
    }
}

fn find_guard(data: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for (i, line) in data.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == '^' {
                return Some((i as i32, j as i32));
            }
        }
    }
    None
}

fn find_loop(data: &Vec<Vec<char>>, guard: &Guard) -> bool {
    let mut visited = HashSet::new();
    let mut guard = guard.clone();
    let mut data = data.clone();

    let (x, y) = guard.next();
    match data.get((x) as usize).and_then(|row| row.get((y) as usize)) {
        Some(_) => data[x as usize][y as usize] = '#',
        _ => return false
    }

    loop {
        let pos = (guard.position(), DIRECTIONS[guard.direction]);
        if visited.contains(&pos) {
            return true;
        }
        visited.insert(pos);

        let (x, y) = guard.next();
        match data.get((x) as usize).and_then(|row| row.get((y) as usize)) {
            None => break,
            Some('#') => guard.turn_right(),
            _ => guard.move_forwards(),
        }
    }
    false
}

fn main() {
    let data: Vec<Vec<char>> = include_str!("../input.txt")
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (x, y) = find_guard(&data).expect("Guard not found");
    let mut guard = Guard::new(x, y);
    
    let mut visited = HashSet::new();
    let mut loops = HashSet::new();
    let mut notloops = HashSet::new();

    loop {
        visited.insert(guard.position());

        if !notloops.contains(&guard.next()) && find_loop(&data, &guard) {
            loops.insert(guard.next());
        } else {
            notloops.insert(guard.next());
        }

        let (x, y) = guard.next();
        match data.get((x) as usize).and_then(|row| row.get((y) as usize)) {
            None => break,
            Some('#') => guard.turn_right(),
            _ => guard.move_forwards(),
        }

    }

    println!("Part 1: {}", visited.len());
    
    println!("Part 2: {}", loops.len());
}
