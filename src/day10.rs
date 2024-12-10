use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(map) = read_input() {
        println!("a={:?}", possible_trails(&map));
        println!("b={:?}", trails_rating(&map));
    }
}

fn possible_trails(map: &Vec<Vec<usize>>) -> usize {
    let mut score = 0;
    for trail_head in trail_heads(&map) {
        let trails = trails(trail_head.0, trail_head.1, &map);
        score += trails.len();
    }
    return score;
}

fn trails_rating(map: &Vec<Vec<usize>>) -> usize {
    let mut score = 0;
    for trail_head in trail_heads(&map) {
        score += rate_trails(trail_head.0, trail_head.1, &map);
    }
    return score;
}

fn trail_heads(map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut trail_heads = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[1].len() {
            if map[i][j] == 0 {
                trail_heads.push((i, j));
            }
        }
    }
    return trail_heads;
}

fn trails(y: usize, x: usize, map: &Vec<Vec<usize>>) -> HashSet<(usize, usize)> {
    let mut reachable_ends = HashSet::new();
    let mut stack = vec![(y, x)];
    while let Some((cy, cx)) = stack.pop() {
        let current_value = map[cy][cx];
        if current_value == 9 {
            reachable_ends.insert((cy, cx));
            continue;
        }
        let neighbors = [
            (cy.saturating_sub(1), cx),
            (cy + 1, cx),
            (cy, cx.saturating_sub(1)),
            (cy, cx + 1),
        ];

        for &(ny, nx) in &neighbors {
            if ny < map.len() && nx < map[0].len() && map[ny][nx] == current_value + 1 {
                stack.push((ny, nx));
            }
        }
    }

    return reachable_ends;
}

fn rate_trails(y: usize, x: usize, map: &Vec<Vec<usize>>) -> usize {
    let mut total_rating = 0;
    let mut stack = vec![(y, x)];
    while let Some((cy, cx)) = stack.pop() {
        let current_value = map[cy][cx];
        if current_value == 9 {
            total_rating += 1;
            continue;
        }
        let neighbors = [
            (cy.saturating_sub(1), cx),
            (cy + 1, cx),
            (cy, cx.saturating_sub(1)),
            (cy, cx + 1),
        ];

        for &(ny, nx) in &neighbors {
            if ny < map.len() && nx < map[0].len() && map[ny][nx] == current_value + 1 {
                stack.push((ny, nx));
            }
        }
    }

    return total_rating;
}

fn read_input() -> io::Result<Vec<Vec<usize>>> {
    let filename = Path::new("data/day_10_input");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut vec = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<usize> = line
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|n| n as usize))
            .collect();
        vec.push(numbers);
    }

    Ok(vec)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trail_heads() {
        let map = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        let trail_heads = trail_heads(&map);
        assert_eq!(trail_heads.len(), 9);
    }

    #[test]
    fn test_trails() {
        let map = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        let heads_and_scores = vec![
            (0, 2, 5),
            (0, 4, 6),
            (2, 4, 5),
            (4, 6, 3),
            (5, 2, 1),
            (5, 5, 3),
            (6, 0, 5),
            (6, 6, 3),
            (7, 1, 5),
        ];
        for (y, x, score) in heads_and_scores {
            let trails = trails(y, x, &map);
            assert_eq!(trails.len(), score);
        }
    }

    #[test]
    fn test_possible_trails() {
        let map = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        let trails = possible_trails(&map);
        assert_eq!(trails, 36);
    }

    #[test]
    fn test_trails_rating() {
        let map = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        let trails = trails_rating(&map);
        assert_eq!(trails, 81);
    }
}
