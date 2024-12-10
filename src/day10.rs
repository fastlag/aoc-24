use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(map) = read_input() {
        println!(
            "a={:?}",
            map.iter()
                .map(|report| report_safe(report) as i32)
                .sum::<i32>()
        );
    }
}

fn read_input() -> io::Result<Vec<Vec<i32>>> {
    let filename = Path::new("data/day_2_input");
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
