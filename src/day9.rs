use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(disk_map) = read_input() {
        println!(
            "a={:?}",
            checksum(&order_disk_space(&disk_space(&disk_map)))
        );
        println!(
            "b={:?}",
            checksum(&compact_disk_space(&disk_space(&disk_map), &disk_map))
        );
    }
}

fn checksum(ordered_disk_space: &Vec<i64>) -> i64 {
    let mut checksum = 0;
    for position in 0..ordered_disk_space.len() {
        if ordered_disk_space[position] != -1 {
            checksum += (position as i64) * ordered_disk_space[position];
        }
    }
    return checksum;
}

fn order_disk_space(disk_space: &Vec<i64>) -> Vec<i64> {
    let mut ordered_space = disk_space.clone();
    let mut first_free = 0;
    let mut last_occupied = disk_space.len() - 1;
    loop {
        if first_free == last_occupied || first_free > last_occupied {
            break;
        }
        if disk_space[first_free] != -1 {
            first_free += 1;
        }
        if disk_space[last_occupied] == -1 {
            last_occupied -= 1;
        }
        if disk_space[first_free] == -1 && disk_space[last_occupied] != -1 {
            ordered_space.swap(first_free, last_occupied);
            first_free += 1;
            last_occupied -= 1;
        }
    }
    return ordered_space;
}

fn compact_disk_space(disk_space: &Vec<i64>, disk_map: &Vec<usize>) -> Vec<i64> {
    let mut compact_disk_space = disk_space.clone();
    let mut free_map = free_map(&disk_map);
    let mut position = disk_space.len();
    while position > 0 {
        position -= 1;
        if compact_disk_space[position] != -1 {
            let num = compact_disk_space[position];
            let mut start = position;
            while start > 0 && compact_disk_space[start - 1] == num {
                start -= 1;
            }
            let size = position - start + 1;

            let mut moved = false;
            for i in 0..free_map.len() {
                let (free_start, free_size) = free_map[i];
                if free_size >= size && free_start < position {
                    moved = true;
                    // move entire block
                    for offset in 0..size {
                        compact_disk_space.swap(start + offset, free_start + offset);
                    }
                    free_map.remove(i);
                    if free_size > size {
                        // add back any remaining space
                        free_map.insert(i, (free_start + size, free_size - size))
                    }
                    break;
                }
            }
            if !moved {
                position = start;
            }
        }
    }
    return compact_disk_space;
}

fn free_map(disk_map: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut free_map = Vec::new();
    let mut free = false;
    let mut position = 0;
    for &num in disk_map {
        if free {
            free_map.push((position, num));
            free = false;
        } else {
            free = true;
        }
        position += num;
    }
    return free_map;
}

fn disk_space(disk_map: &Vec<usize>) -> Vec<i64> {
    let mut disk_space = Vec::new();
    let mut id = 0;
    let mut free = false;
    for &num in disk_map {
        if free {
            disk_space.extend(std::iter::repeat(-1).take(num));
            free = false;
        } else {
            disk_space.extend(std::iter::repeat(id).take(num));
            free = true;
            id += 1;
        }
    }
    return disk_space;
}

fn read_input() -> io::Result<Vec<usize>> {
    let filename = Path::new("data/day_9_input");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let content: String = reader.lines().collect::<Result<_, _>>()?;
    let numbers: Vec<usize> = content
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|n| n as usize))
        .collect();

    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_space() {
        let disk_map = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
        let expected = vec![
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];
        let disk_space = disk_space(&disk_map);
        assert_eq!(disk_space, expected);
    }

    #[test]
    fn test_free_map() {
        let disk_map = vec![2, 3, 3, 3, 1, 3, 3, 1];
        let expected = vec![(2, 3), (8, 3), (12, 3), (18, 1)];
        let free_map = free_map(&disk_map);
        assert_eq!(free_map, expected);
    }

    #[test]
    fn test_checksum_and_order() {
        let disk_space = vec![
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];
        let expected = 1928;
        let checksum = checksum(&order_disk_space(&disk_space));
        assert_eq!(checksum, expected);
    }

    #[test]
    fn test_checksum_and_compact_order() {
        let disk_space = vec![
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];
        println!("{:?}", disk_space);
        let disk_map = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
        println!("{:?}", free_map(&disk_map));
        let expected = 2858;
        let compact_disk_space = compact_disk_space(&disk_space, &disk_map);
        println!("{:?}", compact_disk_space);
        let checksum = checksum(&compact_disk_space);
        assert_eq!(checksum, expected);
    }
}
