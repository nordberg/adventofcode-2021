extern crate strum;

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(EnumString)]
enum Direction {
    up,
    down,
    forward,
}

struct Movement {
    direction: Direction,
    distance: i32,
}

#[derive(Debug, Clone)]
struct Point {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

pub fn calculate_movements() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut movements: Vec<Movement> = Vec::new();

    for line in reader.lines() {
        let bar = line.expect("Error reading the line");
        let mut foo = bar.trim().split_whitespace();
        let dir = foo
            .next()
            .unwrap()
            .parse::<Direction>()
            .expect("Error parsing direction");
        let dis = foo
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("Error parsing distance");
        movements.push(Movement {
            direction: dir,
            distance: dis,
        });
    }

    let pos = Point {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    let final_pos = movements.iter().fold(pos, |acc, m| acc.apply(m));

    println!("{:?}", final_pos.depth * final_pos.horizontal);
}

impl Point {
    pub fn apply(&self, m: &Movement) -> Point {
        match m.direction {
            Direction::up => Point {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim - m.distance,
            },
            Direction::down => Point {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + m.distance,
            },
            Direction::forward => Point {
                horizontal: self.horizontal + m.distance,
                depth: self.depth + (self.aim * m.distance),
                aim: self.aim,
            },
        }
    }
}
