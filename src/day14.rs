use image::{ImageBuffer, Rgb};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Robot {
    p: (usize, usize),
    v: (i64, i64),
}

pub fn run() {
    let product = |(a, b, c, d): (usize, usize, usize, usize)| a * b * c * d;
    if let Ok(robots) = read_input() {
        println!(
            "a={:?}",
            product(quadrants_robots(&state(&robots, 101, 103, 100)))
        );
        tree_search(&robots, 101, 103);
    }
}

fn tree_search(robots: &Vec<Robot>, grid_x: usize, grid_y: usize) {
    let mut new_robots = robots.clone();
    for second in 0..grid_x * grid_y {
        for i in 0..robots.len() {
            let r = next_state(&new_robots[i], grid_y, grid_x);
            new_robots[i] = r;
        }
        let mut grid = vec![vec![0; grid_x]; grid_y];
        for robot in &new_robots {
            grid[robot.p.1][robot.p.0] += 1;
        }
        // render_grid(&grid);
        let filename = format!("14/{}.png", second);
        if let Ok(_) = save_grid_as_png(&grid, filename.as_str()) {
            continue;
        } else {
            continue;
        }
    }
}

fn quadrants_robots(grid: &Vec<Vec<usize>>) -> (usize, usize, usize, usize) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let row_mid = grid.len() / 2;
    let col_mid = grid[0].len() / 2;
    for row in 0..row_mid {
        for col in 0..col_mid {
            a += grid[row][col];
        }
    }

    for row in 0..row_mid {
        for col in col_mid + 1..grid[0].len() {
            b += grid[row][col];
        }
    }

    for row in row_mid + 1..grid.len() {
        for col in 0..col_mid {
            c += grid[row][col];
        }
    }

    for row in row_mid + 1..grid.len() {
        for col in col_mid + 1..grid[0].len() {
            d += grid[row][col];
        }
    }
    return (a, b, c, d);
}

fn state(robots: &Vec<Robot>, grid_x: usize, grid_y: usize, seconds: usize) -> Vec<Vec<usize>> {
    let mut new_robots = robots.clone();
    for _ in 0..seconds {
        for i in 0..robots.len() {
            let r = next_state(&new_robots[i], grid_y, grid_x);
            new_robots[i] = r;
        }
    }
    let mut grid = vec![vec![0; grid_x]; grid_y];
    for robot in new_robots {
        grid[robot.p.1][robot.p.0] += 1;
    }
    return grid;
}

fn next_state(robot: &Robot, grid_y: usize, grid_x: usize) -> Robot {
    let new_x: usize;
    if robot.v.0 < 0 && robot.v.0.abs() as usize > robot.p.0 {
        new_x = (grid_x - robot.v.0.abs() as usize) + robot.p.0;
    } else if robot.v.0 < 0 {
        new_x = robot.p.0 - robot.v.0.abs() as usize;
    } else {
        let potential_new_x = robot.v.0.abs() as usize + robot.p.0;
        if potential_new_x >= grid_x {
            new_x = potential_new_x - grid_x;
        } else {
            new_x = potential_new_x
        }
    }
    let new_y: usize;
    if robot.v.1 < 0 && robot.v.1.abs() as usize > robot.p.1 {
        new_y = (grid_y - robot.v.1.abs() as usize) + robot.p.1;
    } else if robot.v.1 < 0 {
        new_y = robot.p.1 - robot.v.1.abs() as usize;
    } else {
        let potential_new_y = robot.v.1.abs() as usize + robot.p.1;
        if potential_new_y >= grid_y {
            new_y = potential_new_y - grid_y;
        } else {
            new_y = potential_new_y
        }
    }
    return Robot {
        p: (new_x, new_y),
        v: robot.v,
    };
}

fn read_input() -> io::Result<Vec<Robot>> {
    let filename = Path::new("data/day_14_input");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let content: String = reader.lines().collect::<Result<_, _>>()?;

    let mut vec = Vec::new();

    let re = Regex::new(r"p=(\d+),(\d+)\s+v=(-?\d+),(-?\d+)").unwrap();

    for caps in re.captures_iter(content.replace("\r\n", "\n").trim()) {
        let p = (
            caps[1].parse::<usize>().unwrap(),
            caps[2].parse::<usize>().unwrap(),
        );
        let v = (
            caps[3].parse::<i64>().unwrap(),
            caps[4].parse::<i64>().unwrap(),
        );
        vec.push(Robot { p, v });
    }

    Ok(vec)
}

fn save_grid_as_png(
    grid: &Vec<Vec<usize>>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = grid[0].len() as u32;
    let height = grid.len() as u32;

    let mut img = ImageBuffer::new(width, height);

    let color_map = |value: usize| -> Rgb<u8> {
        match value {
            0 => Rgb([0, 0, 0]),
            1 => Rgb([255, 255, 255]),
            2 => Rgb([0, 255, 0]),
            3 => Rgb([0, 0, 255]),
            _ => Rgb([255, 0, 0]),
        }
    };

    // Populate the image buffer
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            img.put_pixel(x as u32, y as u32, color_map(cell));
        }
    }

    // Save the image as a PNG file
    img.save(filename)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_state() {
        let robot = Robot {
            p: (2, 4),
            v: (2, -3),
        };
        let next_robot = next_state(&robot, 7, 11);
        println!("{:?}", next_robot);
        assert_eq!(next_robot.p.0, 4);
        assert_eq!(next_robot.p.1, 1);
        let next_next_robot = next_state(&next_robot, 7, 11);
        println!("{:?}", next_next_robot);
        assert_eq!(next_next_robot.p.0, 6);
        assert_eq!(next_next_robot.p.1, 5);
    }

    #[test]
    fn test_state_single_robot() {
        let robots = vec![Robot {
            p: (2, 4),
            v: (2, -3),
        }];
        let state = state(&robots, 11, 7, 5);
        assert_eq!(state[3][1], 1);
    }

    #[test]
    fn test_state() {
        let robots = vec![
            Robot {
                p: (0, 4),
                v: (3, -3),
            },
            Robot {
                p: (6, 3),
                v: (-1, -3),
            },
            Robot {
                p: (10, 3),
                v: (-1, 2),
            },
            Robot {
                p: (2, 0),
                v: (2, -1),
            },
            Robot {
                p: (0, 0),
                v: (1, 3),
            },
            Robot {
                p: (3, 0),
                v: (-2, -2),
            },
            Robot {
                p: (7, 6),
                v: (-1, -3),
            },
            Robot {
                p: (3, 0),
                v: (-1, -2),
            },
            Robot {
                p: (9, 3),
                v: (2, 3),
            },
            Robot {
                p: (7, 3),
                v: (-1, 2),
            },
            Robot {
                p: (2, 4),
                v: (2, -3),
            },
            Robot {
                p: (9, 5),
                v: (-3, -3),
            },
        ];
        let state = state(&robots, 11, 7, 100);
        for row in &state {
            println!("{:?}", row);
        }
        let qr = quadrants_robots(&state);
        assert_eq!(12, qr.0 * qr.1 * qr.2 * qr.3);
    }
}
