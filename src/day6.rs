use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(map) = read_input() {
        println!(
            "\na={:?}",
            &guard_path(&map)
                .iter()
                .map(|slice| slice.iter().filter(|&&x| x == 'X').count())
                .sum::<usize>()
        );
        //println!("\nb={:?}", count_x_mas(&word_puzzle));
    }
}

fn guard_path(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map_with_path = map.clone();
    let mut pos = guard_pos(map);
    map_with_path[pos.0][pos.1] = 'X';
    loop {
        if pos.2 == 0 {
            if pos.0 >= 1 {
                if map[pos.0 - 1][pos.1] != '#' {
                    pos.0 -= 1;
                    map_with_path[pos.0][pos.1] = 'X';
                    continue;
                } else {
                    pos.2 = 1
                }
            } else {
                break;
            }
        }
        if pos.2 == 1 {
            if pos.1 < map[0].len() - 1 {
                if map[pos.0][pos.1 + 1] != '#' {
                    pos.1 += 1;
                    map_with_path[pos.0][pos.1] = 'X';
                    continue;
                } else {
                    pos.2 = 2
                }
            } else {
                break;
            }
        }
        if pos.2 == 2 {
            if pos.0 < map.len() - 1 {
                if map[pos.0 + 1][pos.1] != '#' {
                    pos.0 += 1;
                    map_with_path[pos.0][pos.1] = 'X';
                    continue;
                } else {
                    pos.2 = 3
                }
            } else {
                break;
            }
        }
        if pos.2 == 3 {
            if pos.1 >= 1 {
                if map[pos.0][pos.1 - 1] != '#' {
                    pos.1 -= 1;
                    map_with_path[pos.0][pos.1] = 'X';
                    continue;
                } else {
                    pos.2 = 0
                }
            } else {
                break;
            }
        }
    }
    return map_with_path;
}

fn guard_pos(map: &Vec<Vec<char>>) -> (usize, usize, usize) {
    for i in 0..map.len() - 1 {
        for j in 0..map[0].len() - 1 {
            if map[i][j] == '<' {
                return (i, j, 3);
            } else if map[i][j] == '>' {
                return (i, j, 1);
            } else if map[i][j] == '^' {
                return (i, j, 0);
            } else if map[i][j] == 'v' {
                return (i, j, 2);
            }
        }
    }
    return (0, 0, 0);
}

fn read_input() -> io::Result<Vec<Vec<char>>> {
    let filename = Path::new("day_6_input");
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
    fn test_count_guard_positions() {
        let map = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ];
        let guard_path = guard_path(&map);
        for slice in &guard_path {
            println!("{:?}", slice);
        }
        let result = &guard_path
            .iter()
            .map(|slice| slice.iter().filter(|&&x| x == 'X').count())
            .sum::<usize>();
        assert_eq!(*result as i32, 41);
    }
}
