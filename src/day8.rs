use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(frequency_map) = read_input() {
        println!(
            "\na={:?}",
            antinodes(&frequency_map)
                .iter()
                .flatten()
                .filter(|&&x| x == '#')
                .count()
        );
        println!(
            "\nb={:?}",
            harmonic_antinodes(&frequency_map)
                .iter()
                .flatten()
                .filter(|&&x| x == '#')
                .count()
        );
    }
}

fn antinodes(frequency_map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let y_size = frequency_map.len();
    let x_size = frequency_map[0].len();
    let mut antinodes = vec![vec!['.'; x_size]; y_size];
    for i in 0..y_size {
        for j in 0..x_size {
            let symbol = frequency_map[i][j];
            if symbol == '.' {
                continue;
            }
            add_antinodes(&frequency_map, symbol, i, j, &mut antinodes);
        }
    }
    return antinodes;
}

fn harmonic_antinodes(frequency_map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let y_size = frequency_map.len();
    let x_size = frequency_map[0].len();
    let mut antinodes = vec![vec!['.'; x_size]; y_size];
    for i in 0..y_size {
        for j in 0..x_size {
            let symbol = frequency_map[i][j];
            if symbol == '.' {
                continue;
            }
            add_harmonic_antinodes(&frequency_map, symbol, i, j, &mut antinodes);
        }
    }
    return antinodes;
}

fn add_antinodes(
    frequency_map: &Vec<Vec<char>>,
    symbol: char,
    i: usize,
    j: usize,
    antinodes: &mut Vec<Vec<char>>,
) {
    let y_size = frequency_map.len();
    let x_size = frequency_map[0].len();
    for m in 0..y_size {
        for n in 0..x_size {
            if symbol == frequency_map[m][n] {
                if m == i && n == j {
                    continue;
                }
                if 2 * m >= i && 2 * n >= j {
                    let antinode_y: usize = 2 * m - i;
                    let antinode_x: usize = 2 * n - j;
                    if antinode_y < y_size && antinode_x < x_size {
                        antinodes[antinode_y][antinode_x] = '#';
                    }
                }
            }
        }
    }
}

fn add_harmonic_antinodes(
    frequency_map: &Vec<Vec<char>>,
    symbol: char,
    i: usize,
    j: usize,
    antinodes: &mut Vec<Vec<char>>,
) {
    let y_size = frequency_map.len();
    let x_size = frequency_map[0].len();
    for m in 0..y_size {
        for n in 0..x_size {
            if symbol == frequency_map[m][n] {
                let positions = positions_along_line(frequency_map, i, j, m, n);
                for position in positions {
                    antinodes[position.0][position.1] = '#';
                }
            }
        }
    }
}

fn positions_along_line(
    frequency_map: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    m: usize,
    n: usize,
) -> Vec<(usize, usize)> {
    let y_size = frequency_map.len();
    let x_size = frequency_map[0].len();
    let mut positions = Vec::new();

    let i = i as isize;
    let j = j as isize;
    let m = m as isize;
    let n = n as isize;

    let dy = m - i;
    let dx = n - j;

    if dx == 0 {
        let y_min = cmp::min(i, m);
        let y_max = cmp::max(i, m);
        for y in y_min..=y_max {
            if y >= 0 && y < y_size as isize {
                positions.push((j as usize, y as usize));
            }
        }
    } else if dy == 0 {
        let x_min = cmp::min(j, n);
        let x_max = cmp::max(j, n);
        for x in x_min..=x_max {
            if x >= 0 && x < x_size as isize {
                positions.push((x as usize, i as usize));
            }
        }
    } else {
        let gcd = gcd(dx.abs(), dy.abs());
        let step_x = dx / gcd;
        let step_y = dy / gcd;

        let mut x = j;
        let mut y = i;
        while x >= 0 && x < x_size as isize && y >= 0 && y < y_size as isize {
            positions.push((x as usize, y as usize));
            x += step_x;
            y += step_y;
        }
    }
    positions
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn read_input() -> io::Result<Vec<Vec<char>>> {
    let filename = Path::new("day_8_input");
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
    fn test_count_antinodes() {
        let freqs = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'A', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        for freqs_slice in &freqs {
            println!("{:?}", freqs_slice);
        }
        println!("------------------------------------------------------------");
        let antinodes = antinodes(&freqs);
        for antinodes_slice in &antinodes {
            println!("{:?}", antinodes_slice);
        }
        let result = antinodes.iter().flatten().filter(|&&x| x == '#').count();
        assert_eq!(result, 14);
    }

    #[test]
    fn test_count_harmonic_antinodes_tiny() {
        let freqs = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', 'x', '.', '.'],
            vec!['.', '.', 'x', '.'],
            vec!['.', '.', '.', '.'],
        ];
        for freqs_slice in &freqs {
            println!("{:?}", freqs_slice);
        }
        println!("------------------------------------------------------------");
        let antinodes = harmonic_antinodes(&freqs);
        for antinodes_slice in &antinodes {
            println!("{:?}", antinodes_slice);
        }
        let result = antinodes.iter().flatten().filter(|&&x| x == '#').count();
        assert_eq!(result, 4);
    }
    #[test]
    fn test_count_harmonic_antinodes() {
        let freqs = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'A', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        for freqs_slice in &freqs {
            println!("{:?}", freqs_slice);
        }
        println!("------------------------------------------------------------");
        let antinodes = harmonic_antinodes(&freqs);
        for antinodes_slice in &antinodes {
            println!("{:?}", antinodes_slice);
        }
        let result = antinodes.iter().flatten().filter(|&&x| x == '#').count();
        assert_eq!(result, 34);
    }
}
