use std::io::stdin;
use std::cmp::min;

fn update_buffer(buffer: &mut String, char: char, n: usize) {
    buffer.push(char);
    if buffer.len() > n {
        buffer.remove(0);
    }
}

fn find(file: &Vec<Vec<char>>) -> i32{
    let mut buffer: String = String::new();
    let (n, m) = (file.len(), file[0].len());

    let mut sum = file
        .iter()
        .map(|line| {
            line.iter()
                .collect::<String>()
                .matches("XMAS")
                .count() as i32
        })
        .sum::<i32>();

    for i in 0..m-3 {
        for j in 0..min(n, m-i) {
            update_buffer(&mut buffer, file[j][j+i], 4);
            if buffer == "XMAS" {
                sum += 1
            }
        }
        buffer.clear()
    } 

    for i in 1..n-3 {
        for j in 0..min(n-i, m) {
            update_buffer(&mut buffer, file[j+i][j], 4);
            if buffer == "XMAS" {
                sum += 1
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

fn find2(file: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut buffer: String = String::new();
    let (n, m) = (file.len(), file[0].len());

    for i in 0..m-3 {
        for j in 0..min(n, m-i) {
            update_buffer(&mut buffer, file[j][j+i], 3);
            if (buffer == "MAS" || buffer == "SAM") && ((file[j-2][j+i] == 'M' && file[j][j+i-2] == 'S') || (file[j-2][j+i] == 'S' && file[j][j+i-2] == 'M')) {
                sum += 1
            }
        }
        buffer.clear()
    } 

    for i in 1..n-3 {
        for j in 0..min(n-i, m) {
            update_buffer(&mut buffer, file[j+i][j], 3);
            if (buffer == "MAS" || buffer == "SAM") && ((file[j+i-2][j] == 'M' && file[j+i][j-2] == 'S') || (file[j+i-2][j] == 'S' && file[j+i][j-2] == 'M')) {
                sum += 1
            }
        }
        buffer.clear()
    }       

    sum
}

fn part1(file: &Vec<Vec<char>>) {
    let mut sum = 0;
    let mut matrix = file.clone();
    for _ in 0..4 {
        sum += find(&matrix);
        matrix = rotate_matrix(&matrix);
    }

    println!("{}", sum);
}

fn part2(file: &Vec<Vec<char>>) {
    let mut sum = 0;
    
    sum += find2(&file);
       

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

    part1(&file);
    part2(&file);
}
