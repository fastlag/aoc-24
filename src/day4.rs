use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(word_puzzle) = read_input() {
        println!("matrix={:?}", word_puzzle);
        println!("\na={:?}", count_xmas(word_puzzle));
    }
}

fn count_xmas(word_puzzle: Vec<Vec<char>>) -> i32 {
    let size_x = word_puzzle.len();
    let size_y = word_puzzle[0].len();
    println!("size=({:?}, {:?})", size_x, size_y);
    let mut count = 0;
    let mas = vec!['M', 'A', 'S'];
    for i in 0..size_x {
        for j in 0..size_y {
            if word_puzzle[i][j] == 'X' {
                print!("\n({:?}, {:?})=X: ", i, j);
                // Check lower right triangle
                if check_word(&word_puzzle, i + 1, j + 1, i + 3, j + 3, mas.clone()) {
                    count += 1;
                }
                if check_word(&word_puzzle, i + 1, j, i + 3, j, mas.clone()) {
                    count += 1;
                }
                if check_word(&word_puzzle, i, j + 1, i, j + 3, mas.clone()) {
                    count += 1;
                }
                if i >= 3 {
                    // Check upper right triangle
                    if check_word(&word_puzzle, i - 1, j + 1, i - 3, j + 3, mas.clone()) {
                        count += 1;
                    }
                    if check_word(&word_puzzle, i - 1, j, i - 3, j, mas.clone()) {
                        count += 1;
                    }
                }
                if j >= 3 {
                    // Check lower left triangle
                    if check_word(&word_puzzle, i + 1, j - 1, i + 3, j - 3, mas.clone()) {
                        count += 1;
                    }
                    if check_word(&word_puzzle, i, j - 1, i, j - 3, mas.clone()) {
                        count += 1;
                    }
                }
                if j >= 3 && i >= 3 {
                    // Check upper left triangle
                    if check_word(&word_puzzle, i - 1, j - 1, i - 3, j - 3, mas.clone()) {
                        count += 1;
                    }
                }
            }
        }
    }
    return count;
}

fn check_word(
    word_puzzle: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    x: usize,
    y: usize,
    word: Vec<char>,
) -> bool {
    let mut mas = word.into_iter();
    print!("\n\t({:?}, {:?})->({:?}, {:?})", i, j, x, y);
    if x >= word_puzzle.len() || y >= word_puzzle[0].len() {
        print!(" => out of bounds!");
        return false;
    }
    print!(" = ");
    for (m, n) in create_range(i, x).zip(create_range(j, y)) {
        print!("{:?}", word_puzzle[m][n]);
    }
    for (m, n) in create_range(i, x).zip(create_range(j, y)) {
        if word_puzzle[m][n] != mas.next().unwrap() {
            return false;
        }
    }
    print!(" => match!");
    return true;
}

fn create_range(start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
    if start == end {
        Box::new(std::iter::repeat(start))
    } else if start > end {
        Box::new((end..start + 1).rev())
    } else {
        Box::new(start..end)
    }
}

fn read_input() -> io::Result<Vec<Vec<char>>> {
    let filename = Path::new("day_4_small");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut vec = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let characters: Vec<char> = line.chars().collect();
        vec.push(characters);
    }

    Ok(vec)
}
