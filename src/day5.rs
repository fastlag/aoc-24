use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(rules_and_orders) = read_input() {
        println!(
            "a={:?}",
            sum_middle_values(&valid_orders(&rules_and_orders.0, &rules_and_orders.1))
        );

        println!(
            "b={:?}",
            sum_middle_values(&order_orders(
                &rules_and_orders.0,
                &invalid_orders(&rules_and_orders.0, &rules_and_orders.1)
            ))
        );
    }
}

fn sum_middle_values(orders: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for order in orders {
        if let Some(&middle_value) = order.get(order.len() / 2) {
            sum += middle_value;
        }
    }
    return sum;
}

fn order_orders(rules: &Vec<(i32, i32)>, orders: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for (key, value) in rules {
        rule_map
            .entry(key.clone())
            .or_insert_with(Vec::new)
            .push(value.clone());
    }
    return orders
        .iter()
        .map(|order| sort_order(rules, &rule_map, order))
        .collect();
}

fn sort_order(
    rules: &Vec<(i32, i32)>,
    rule_map: &HashMap<i32, Vec<i32>>,
    order: &Vec<i32>,
) -> Vec<i32> {
    let mut sorted_order = order.clone();
    while !valid_order(rules, &sorted_order) {
        let mut index = 0;
        while index < sorted_order.len() {
            let page = sorted_order[index];

            if let Some(successors) = rule_map.get(&page) {
                for &successor in successors {
                    if let Some(successor_index) = sorted_order.iter().position(|&x| x == successor)
                    {
                        if successor_index < index {
                            sorted_order.remove(successor_index);
                            sorted_order.insert(index, successor);
                        }
                    }
                }
            }
            index += 1;
        }
    }
    sorted_order
}

fn valid_orders(rules: &Vec<(i32, i32)>, orders: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut valid_orders = Vec::new();
    for order in orders {
        if valid_order(&rules, order) {
            valid_orders.push(order.clone());
        }
    }
    return valid_orders;
}

fn valid_order(rules: &Vec<(i32, i32)>, order: &Vec<i32>) -> bool {
    for rule in rules {
        if !check_rule(rule, order) {
            return false;
        }
    }
    return true;
}

fn invalid_orders(rules: &Vec<(i32, i32)>, orders: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut invalid_orders = Vec::new();
    for order in orders {
        let mut rule_failed = false;
        for rule in rules {
            if !check_rule(rule, order) {
                rule_failed = true;
                break;
            }
        }
        if rule_failed {
            invalid_orders.push(order.clone());
        }
    }
    return invalid_orders;
}

fn check_rule(rule: &(i32, i32), order: &Vec<i32>) -> bool {
    if let Some(i) = order.iter().position(|&x| x == rule.0) {
        if let Some(j) = order.iter().position(|&x| x == rule.1) {
            return i < j;
        }
    }
    return true;
}

fn read_input() -> io::Result<(Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    let filename = Path::new("day_5_input");
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut rules = Vec::new();
    let mut orders = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.contains("|") {
            let numbers: Vec<i32> = line
                .split("|")
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            rules.push((numbers[0], numbers[1]));
        } else if line.contains(",") {
            let numbers: Vec<i32> = line
                .split(",")
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            orders.push(numbers);
        }
    }

    Ok((rules, orders))
}
