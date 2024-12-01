use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok((vec1, vec2)) = read_input() {
        let distance = total_distance(vec1.clone(), vec2.clone());
        println!("{:?}", distance);
        let score = similarity_score(vec1, vec2);
        println!("{:?}", score);
    }
}

fn total_distance(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
    a.sort();
    b.sort();
    return a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum();
}

fn similarity_score(a: Vec<i32>, b: Vec<i32>) -> i32 {
    a.iter()
        .map(|&target| target * b.iter().filter(|&&value| value == target).count() as i32)
        .sum()
}

fn read_input() -> io::Result<(Vec<i32>, Vec<i32>)> {
    let filename = Path::new("day_1_input");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if numbers.len() >= 2 {
            vec1.push(numbers[0]);
            vec2.push(numbers[1]);
        }
    }

    Ok((vec1, vec2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_distance() {
        let vec1 = vec![10, 20, 30];
        let vec2 = vec![15, 5, 25];
        let result = total_distance(vec1.clone(), vec2.clone());
        let expected = 15;
        assert_eq!(result, expected);
    }
}
