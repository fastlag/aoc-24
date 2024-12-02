use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(reports) = read_input() {
        println!(
            "{:?}",
            reports
                .iter()
                .map(|report| report_change_safety(report).iter().all(|&x| x) as i32)
                .sum::<i32>()
        );
    }
}

fn report_change_safety(report: &Vec<i32>) -> Vec<bool> {
    return report
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|pair| safe(pair[0], pair[1]))
        .collect();
}

fn safe(left: i32, right: i32) -> bool {
    if left.abs() > 3 || right.abs() > 3 {
        return false;
    } else if left.abs() < 1 || right.abs() < 1 {
        return false;
    } else if left < 0 && right > 0 || left > 0 && right < 0 {
        return false;
    } else {
        return true;
    }
}

fn read_input() -> io::Result<Vec<Vec<i32>>> {
    let filename = Path::new("day_2_input");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut vec = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        vec.push(numbers);
    }

    Ok(vec)
}
