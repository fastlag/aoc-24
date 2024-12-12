use std::collections::{hash_map::Entry::*, BTreeSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(stones) = read_input() {
        println!("a={:?}", transform(&stones, 25).iter().count());
        println!(
            "b={:?}",
            memoized_transform(&stones, 75).iter().sum::<usize>()
        );
    }
}

fn memoized_transform(stones: &Vec<usize>, times: usize) -> Vec<usize> {
    let mut lookup: HashMap<usize, Vec<usize>> = HashMap::default();
    let mut stack: BTreeSet<usize> = stones.clone().into_iter().collect();

    while let Some(todo) = stack.pop_first() {
        let mut current = todo;
        loop {
            match lookup.entry(current) {
                Vacant(vacant_entry) => {
                    let new_entry = next(current);
                    current = new_entry[0];
                    vacant_entry.insert(new_entry.clone());
                    for stone in new_entry.clone() {
                        stack.insert(stone);
                    }
                }
                Occupied(_) => {
                    break;
                }
            }
        }
    }

    let mut counts: HashMap<usize, usize> = HashMap::default();
    for stone in stones {
        *counts.entry(*stone).or_insert(0) += 1;
    }

    for _ in 0..times {
        let mut next_counts: HashMap<usize, usize> = HashMap::default();
        for (stone, count) in counts.iter() {
            for stone in lookup.get(stone).unwrap() {
                *next_counts.entry(*stone).or_insert(0) += count;
            }
        }
        counts = next_counts;
    }

    return counts.values().cloned().collect();
}

fn transform(stones: &Vec<usize>, times: usize) -> Vec<usize> {
    let mut new_stones = stones.clone();
    for _ in 0..times {
        let mut transformed_stones = Vec::with_capacity(new_stones.len() * 2);
        for stone in new_stones {
            transformed_stones.extend(next(stone));
        }
        new_stones = transformed_stones;
    }
    new_stones
}

fn next(number: usize) -> Vec<usize> {
    if number == 0 {
        vec![1]
    } else if number.to_string().len() % 2 == 0 {
        let (left_number, right_number) = split_number(number);
        vec![left_number, right_number]
    } else {
        vec![number * 2024]
    }
}

fn split_number(number: usize) -> (usize, usize) {
    let len = (number as f64).log10() as usize + 1;

    if len % 2 != 0 {
        panic!("The number does not have an even number of digits.");
    }

    let mid = len / 2;
    let divisor = 10_usize.pow(mid as u32);

    let left_number = number / divisor;
    let right_number = number % divisor;

    (left_number, right_number)
}

fn read_input() -> io::Result<Vec<usize>> {
    let filename = Path::new("data/day_11_input");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let content: String = reader.lines().collect::<Result<_, _>>()?;
    let numbers: Vec<usize> = content
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let tests = vec![
            (10, vec![1, 0]),
            (0, vec![1]),
            (3, vec![3 * 2024]),
            (100010, vec![100, 10]),
        ];
        for test in tests {
            assert_eq!(next(test.0), test.1);
        }
    }

    #[test]
    fn test_transform() {
        let stones = vec![125, 17];
        let number_stones = transform(&stones, 25).iter().count();
        assert_eq!(number_stones, 55312);
    }

    #[test]
    fn test_memoized_transform() {
        let stones = vec![125, 17];
        let number_stones: usize = memoized_transform(&stones, 25).iter().sum();
        assert_eq!(number_stones, 55312);
    }
}
