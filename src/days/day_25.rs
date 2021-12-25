use std::collections::HashMap;
use std::process::exit;

#[derive(Eq, PartialEq)]
enum Direction {
    Right,
    Down,
}

pub fn solve(input: &str) {
    let mut cucumbers = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    input.lines().enumerate().for_each(|(i, line)| {
        width = line.len();
        height = i + 1;
        line.chars().enumerate().for_each(|(j, c)| match c {
            '>' => {
                cucumbers.insert((j, i), Direction::Right);
            }
            'v' => {
                cucumbers.insert((j, i), Direction::Down);
            }
            _ => {}
        });
    });

    for step in 1.. {
        let mut moving_cucumbers = 0;

        let mut updates = vec![];

        cucumbers
            .iter()
            .filter(|(_, dir)| **dir == Direction::Right)
            .for_each(|((x, y), _)| {
                if !cucumbers.contains_key(&((x + 1) % width, *y)) {
                    updates.push((*x, *y));
                    moving_cucumbers += 1;
                }
            });

        updates.iter().for_each(|(x, y)| {
            cucumbers.insert(((*x + 1) % width, *y), Direction::Right);
            cucumbers.remove(&(*x, *y));
        });

        updates.clear();

        cucumbers
            .iter()
            .filter(|(_, dir)| **dir == Direction::Down)
            .for_each(|((x, y), _)| {
                if !cucumbers.contains_key(&(*x, (y + 1) % height)) {
                    updates.push((*x, *y));
                    moving_cucumbers += 1;
                }
            });

        updates.iter().for_each(|(x, y)| {
            cucumbers.insert((*x, (y + 1) % height), Direction::Down);
            cucumbers.remove(&(*x, *y));
        });

        if moving_cucumbers == 0 {
            println!("No movement after {} steps", step);
            break;
        } else {
            println!("{} cucumbers moving after {} steps", moving_cucumbers, step);
        }
    }
}
