use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(map) = read_input() {
        println!(
            "\na={:?}",
            &areas_and_perimiters(map)
                .iter()
                .map(|&(area, perimiter)| area * perimiter)
                .sum::<usize>()
        );
    }
}

fn areas_and_perimiters(map: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut regions = Vec::new();
    let mut visited = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if visited.contains(&(i, j)) {
                continue;
            }
            let mut region = Vec::new();
            let plant = map[i][j];
            let mut stack = vec![(i, j)];
            stack.extend(neighbours(i, j, map.len() - 1, map[0].len() - 1));
            loop {
                if let Some(new) = stack.pop() {
                    if map[new.0][new.1] == plant && !visited.contains(&(new.0, new.1)) {
                        visited.insert((new.0, new.1));

                        let neighbours = neighbours(new.0, new.1, map.len() - 1, map[0].len() - 1);
                        let mut same_count = 0;
                        for neighbour in neighbours {
                            if map[neighbour.0][neighbour.1] == plant {
                                same_count += 1;
                                if !visited.contains(&(neighbour.0, neighbour.1)) {
                                    stack.push((neighbour.0, neighbour.1));
                                }
                            }
                        }
                        region.push((new.0, new.1, 4 - same_count));
                    }
                } else {
                    break;
                }
            }
            regions.push(region);
        }
    }

    let mut areas_and_perimiters = Vec::new();
    for region in regions {
        let area = region.len();
        let perimiter: usize = region.iter().map(|&(_, _, c)| c).sum();
        areas_and_perimiters.push((area, perimiter));
    }

    return areas_and_perimiters;
}

fn neighbours(y: usize, x: usize, max_y: usize, max_x: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    if y >= 1 {
        neighbours.push((y - 1, x));
    }
    if y < max_y {
        neighbours.push((y + 1, x));
    }
    if x >= 1 {
        neighbours.push((y, x - 1));
    }
    if x < max_x {
        neighbours.push((y, x + 1));
    }
    return neighbours;
}

// position, contribution to perimiter

fn read_input() -> io::Result<Vec<Vec<char>>> {
    let filename = Path::new("data/day_12_input");
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
    fn test_calculate_fence_cost() {
        let map = vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ];
        println!("hello");
        let result: usize = areas_and_perimiters(map)
            .iter()
            .map(|&(area, perimiter)| area * perimiter)
            .sum();
        assert_eq!(result, 1930);
    }
}
