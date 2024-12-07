use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(equations) = read_input() {
        println!(
            "\na={:?}",
            equations
                .iter()
                .filter(|&equation| check_equation(equation))
                .map(|equation| equation[0])
                .sum::<i64>()
        );
        // println!("\nb={:?}", count_x_mas(&word_puzzle));
    }
}

fn check_equation(equation: &Vec<i64>) -> bool {
    return check_dfs(&equation[2..].to_vec(), equation[1], equation[0]);
}

fn mul(a: i64, b: i64) -> i64 {
    return a * b;
}

fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

fn check_dfs(remaining: &Vec<i64>, current_value: i64, expected_value: i64) -> bool {
    if remaining.len() == 0 {
        return current_value == expected_value;
    }
    if current_value > expected_value {
        return false;
    }
    for operator in vec![mul, add] {
        if check_dfs(
            &remaining[1..].to_vec(),
            operator(current_value, remaining[0]),
            expected_value,
        ) {
            return true;
        }
    }
    return false;
}

fn read_input() -> io::Result<Vec<Vec<i64>>> {
    let filename = Path::new("day_7_input");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut vec = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i64> = line
            .replace(":", "")
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        vec.push(numbers);
    }

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let equation = vec![292, 11, 6, 16, 20];
        let result = check_dfs(&equation[1..].to_vec(), 0, equation[0]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_count_valid_equations() {
        let equations = vec![
            vec![190, 10, 19],
            vec![3267, 81, 40, 27],
            vec![83, 17, 5],
            vec![156, 15, 6],
            vec![7290, 6, 8, 6, 15],
            vec![161011, 16, 10, 13],
            vec![192, 17, 8, 14],
            vec![21037, 9, 7, 18, 13],
            vec![292, 11, 6, 16, 20],
        ];
        let result: i64 = equations
            .iter()
            .filter(|&equation| check_equation(equation))
            .map(|equation| equation[0])
            .sum();
        let expected = 3749;
        assert_eq!(result, expected);
    }
}