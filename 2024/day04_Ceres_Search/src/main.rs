use std::io::stdin;
use std::cmp::min;

fn find(file: &Vec<Vec<char>>) -> i32{
    let mut sum = 0;
    let mut buffer: String = String::new();
    for line in file {
        for char in line {
            buffer.push(*char);
            if buffer.len() == 4 {
                if buffer == "XMAS" {
                    sum += 1
                }
                buffer.remove(0);
            }
        }
        buffer.clear()
    }
    sum
}

fn find_on_diagonals(file: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut buffer: String = String::new();
    let (n, m) = (file.len(), file[0].len());
    for i in 0..m {
        for j in 0..min(n, m-i) {
            buffer.push(file[j][j+i]);
            if buffer.len() == 4 {
                if buffer == "XMAS" {
                    sum += 1
                }
                buffer.remove(0);
            }
        }
        buffer.clear()
    } 

    for i in 1..n {
        for j in 0..min(n-i, m) {
            buffer.push(file[j+i][j]);
            if buffer.len() == 4 {
                if buffer == "XMAS" {
                    sum += 1
                }
                buffer.remove(0);
            }
        }
        buffer.clear()
    }            

    sum
}

fn rotate_matrix(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_matrix = vec![vec![' '; matrix.len()]; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            new_matrix[j][matrix.len()-1 - i] = matrix[i][j];
        }
    }
    new_matrix
}

fn _print_matrix(matrix: &Vec<Vec<char>>) {
    for line in matrix {
        for char in line {
            print!("{char} ");
        }
        println!();
    }
    println!();
}

fn part1(file: &Vec<Vec<char>>) {
    let mut sum = 0;
    let mut matrix = file.clone();
    for _ in 0..4 {
        sum += find(&matrix);
        sum += find_on_diagonals(&matrix);
        matrix = rotate_matrix(&matrix);
    }

    println!("{}", sum);
}

fn main() {
    let mut file: Vec<Vec<char>> = Vec::new();
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => file.push(line
                .trim()
                .chars()
                .collect()
            ),
            Err(_) => break,
        }
    }

    part1(&file)
}
