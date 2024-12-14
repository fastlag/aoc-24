use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Game {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

const CORRECTION_VALUE: i64 = 10000000000000;

pub fn run() {
    if let Ok(games) = read_input() {
        println!(
            "a={:?}",
            solutions(&games)
                .iter()
                .map(|&(a, b)| a * 3 + b as i64)
                .sum::<i64>()
        );
        println!(
            "a={:?}",
            solutions(
                &games
                    .iter()
                    .map(|game| (Game {
                        a: game.a,
                        b: game.b,
                        prize: (
                            game.prize.0 + CORRECTION_VALUE,
                            game.prize.1 + CORRECTION_VALUE
                        ),
                    }))
                    .collect()
            )
            .iter()
            .map(|&(a, b)| a * 3 + b as i64)
            .sum::<i64>()
        );
    }
}

fn solutions(games: &Vec<Game>) -> Vec<(i64, i64)> {
    let mut solutions = Vec::new();
    for game in games {
        if let Some(solution) = solution(game) {
            solutions.push(solution);
        }
    }
    return solutions;
}

fn solution(game: &Game) -> Option<(i64, i64)> {
    let a_gcd = gcd(game.a.0, game.a.1);
    let b_gcd = gcd(game.b.0, game.b.1);
    if game.a.0 / a_gcd != game.b.0 / b_gcd || game.a.1 / a_gcd != game.b.1 / b_gcd {
        let a_lcm = lcm(game.a.0, game.a.1);
        let b_lcm = lcm(game.b.0, game.b.1);
        let alpha_numerator = game.prize.0 * (b_lcm / game.b.0) - game.prize.1 * (b_lcm / game.b.1);
        let alpha_denominator = game.a.0 * (b_lcm / game.b.0) - game.a.1 * (b_lcm / game.b.1);
        let beta_numerator = game.prize.0 * (a_lcm / game.a.0) - game.prize.1 * (a_lcm / game.a.1);
        let beta_denominator = game.b.0 * (a_lcm / game.a.0) - game.b.1 * (a_lcm / game.a.1);

        if alpha_numerator % alpha_denominator == 0 && beta_numerator % beta_denominator == 0 {
            let nbr_a = alpha_numerator / alpha_denominator;
            let nbr_b = beta_numerator / beta_denominator;

            if nbr_a >= 0 && nbr_b >= 0 {
                return Some((nbr_a, nbr_b));
            }
        }
    }
    return None;
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut x = a.abs();
    let mut y = b.abs();
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    return x;
}

fn lcm(a: i64, b: i64) -> i64 {
    return (a.abs() * b.abs()) / gcd(a, b);
}

fn read_input() -> io::Result<Vec<Game>> {
    let filename = Path::new("data/day_13_input");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let content: String = reader.lines().collect::<Result<_, _>>()?;

    let mut vec = Vec::new();

    let re = Regex::new(
        r"Button A:\s*X\+(\d+),\s*Y\+(\d+)\s*Button B:\s*X\+(\d+),\s*Y\+(\d+)\s*Prize:\s*X=(\d+),\s*Y=(\d+)",

    )
    .unwrap();

    for caps in re.captures_iter(content.replace("\r\n", "\n").trim()) {
        let a = (
            caps[1].parse::<i64>().unwrap(),
            caps[2].parse::<i64>().unwrap(),
        );
        let b = (
            caps[3].parse::<i64>().unwrap(),
            caps[4].parse::<i64>().unwrap(),
        );
        let prize = (
            caps[5].parse::<i64>().unwrap(),
            caps[6].parse::<i64>().unwrap(),
        );
        vec.push(Game { a, b, prize });
    }

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions_a() {
        let games = vec![
            Game {
                a: (94, 34),
                b: (22, 67),
                prize: (8400, 5400),
            },
            Game {
                a: (26, 66),
                b: (67, 21),
                prize: (12748, 12176),
            },
            Game {
                a: (17, 86),
                b: (84, 37),
                prize: (7870, 6450),
            },
            Game {
                a: (69, 23),
                b: (27, 71),
                prize: (18641, 10279),
            },
        ];
        let solutions = solutions(&games);
        let total_cost: i64 = solutions.iter().map(|&(a, b)| a * 3 + b as i64).sum();
        let prizes: usize = solutions.iter().count();
        assert_eq!(prizes, 2);
        assert_eq!(total_cost, 480);
    }

    #[test]
    fn test_solutions_b() {
        let games = vec![
            Game {
                a: (94, 34),
                b: (22, 67),
                prize: (8400, 5400),
            },
            Game {
                a: (26, 66),
                b: (67, 21),
                prize: (12748, 12176),
            },
            Game {
                a: (17, 86),
                b: (84, 37),
                prize: (7870, 6450),
            },
            Game {
                a: (69, 23),
                b: (27, 71),
                prize: (18641, 10279),
            },
        ];
        let solutions = solutions(
            &games
                .iter()
                .map(|game| Game {
                    a: game.a,
                    b: game.b,
                    prize: (
                        game.prize.0 + CORRECTION_VALUE,
                        game.prize.1 + CORRECTION_VALUE,
                    ),
                })
                .collect(),
        );
        let prizes: usize = solutions.iter().count();
        assert_eq!(prizes, 2);
    }

    #[test]
    fn test_solution_a() {
        let game = Game {
            a: (94, 34),
            b: (22, 67),
            prize: (8400, 5400),
        };
        if let Some(solution) = solution(&game) {
            assert_eq!(solution.0, 80);
            assert_eq!(solution.1, 40);
        } else {
            assert_eq!(true, false);
        }
    }
}
