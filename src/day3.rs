use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(reports) = read_input() {
        println!(
            "a={:?}",
            reports.iter().map(|&pair| pair.0 * pair.1).sum::<i32>()
        );
    }
}

fn read_input() -> io::Result<Vec<(i32, i32)>> {
    let filename = Path::new("day_3_input");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let content: String = reader.lines().collect::<Result<_, _>>()?;

    let mut vec = Vec::new();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for regex_match in re.captures_iter(content.as_str()) {
        if let Ok(x) = regex_match.get(1).unwrap().as_str().parse::<i32>() {
            if let Ok(y) = regex_match.get(2).unwrap().as_str().parse::<i32>() {
                vec.push((x, y));
            }
        };
    }

    Ok(vec)
}