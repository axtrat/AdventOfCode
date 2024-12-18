use std::io::stdin;

fn find(file: &Vec<Vec<char>>) -> i32{
    let (n, m) = (file.len(), file[0].len());
    let mut sum = 0;
    for dr in vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        for i in 0..m {
            for j in 0..n {
                sum += (0..4)
                    .filter_map(|k| {
                        let ni = i as isize + k * dr.0;
                        let nj = j as isize + k * dr.1;
                        file.get(ni as usize)
                            .and_then(|line| {
                            line.get(nj as usize)
                        })
                    })
                    .collect::<String>()
                    .matches("XMAS")
                    .count() as i32;
            }
        }
    }
    sum
}

fn find2(file: &Vec<Vec<char>>) -> i32 {
    let (n, m) = (file.len(), file[0].len());
    let mut sum = 0;
    for i in 0..m-2 {
        for j in 0..n-2 {
            sum += (0..3)
                .filter_map(|k| {
                    if file[i+k][j+k] == file[i+k][j+2-k] {
                        Some(file[i+k][j+k])
                    } else {
                        None
                    }
                })
                .collect::<String>()
                .matches("MAS")
                .count() as i32;
        }
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


fn part1(file: &Vec<Vec<char>>) {
    println!("{}", find(file));
}

fn part2(file: &Vec<Vec<char>>) {
    let mut sum = 0;
    let mut matrix = file.clone();
    for _ in 0..4 {
        sum += find2(&matrix);
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

    part1(&file);
    part2(&file);
}
