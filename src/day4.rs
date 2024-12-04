use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(word_puzzle) = read_input() {
        println!("\na={:?}", count_xmas(&word_puzzle));
        println!("\nb={:?}", count_x_mas(&word_puzzle));
    }
}

fn count_xmas(word_puzzle: &Vec<Vec<char>>) -> i32 {
    let size_x = word_puzzle.len();
    let size_y = word_puzzle[0].len();
    let mut count = 0;
    let mas = vec!['M', 'A', 'S'];
    for i in 0..size_x {
        for j in 0..size_y {
            if word_puzzle[i][j] == 'X' {
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

fn count_x_mas(word_puzzle: &Vec<Vec<char>>) -> i32 {
    let size_x = word_puzzle.len() - 1;
    let size_y = word_puzzle[0].len() - 1;
    let mut count = 0;
    let mas = vec!['M', 'A', 'S'];
    for i in 1..size_x {
        for j in 1..size_y {
            if word_puzzle[i][j] == 'A' {
                if check_word(&word_puzzle, i - 1, j - 1, i + 1, j + 1, mas.clone())
                    || check_word(&word_puzzle, i + 1, j + 1, i - 1, j - 1, mas.clone())
                {
                    if check_word(&word_puzzle, i - 1, j + 1, i + 1, j - 1, mas.clone())
                        || check_word(&word_puzzle, i + 1, j - 1, i - 1, j + 1, mas.clone())
                    {
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
    if x >= word_puzzle.len() || y >= word_puzzle[0].len() {
        return false;
    }
    for (m, n) in create_range(i, x).zip(create_range(j, y)) {
        if word_puzzle[m][n] != mas.next().unwrap() {
            return false;
        }
    }
    return true;
}

fn create_range(start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
    if start == end {
        Box::new(std::iter::repeat(start))
    } else if start > end {
        Box::new((end..start + 1).rev())
    } else {
        Box::new(start..end + 1)
    }
}

fn read_input() -> io::Result<Vec<Vec<char>>> {
    let filename = Path::new("day_4_input");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_xmas() {
        let word_puzzle = vec![
            vec!['S', 'W', 'W', 'S', 'W', 'W', 'S'],
            vec!['W', 'A', 'W', 'A', 'W', 'A', 'W'],
            vec!['W', 'W', 'M', 'M', 'M', 'W', 'W'],
            vec!['S', 'A', 'M', 'X', 'M', 'A', 'S'],
            vec!['W', 'W', 'M', 'M', 'M', 'W', 'W'],
            vec!['W', 'A', 'W', 'A', 'W', 'A', 'W'],
            vec!['S', 'W', 'W', 'S', 'W', 'W', 'S'],
        ];
        let result = count_xmas(&word_puzzle);
        let expected = 8;
        assert_eq!(result, expected);
    }
}
