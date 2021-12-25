use std::cmp::{max, min};
use std::collections::HashMap;

pub fn solve_day_5(input: &str) {
    let mut points = HashMap::new();

    for line in input.split('\n') {
        let mut left_right = line.split(" -> ");

        let left = left_right.next().unwrap();
        let right = left_right.next().unwrap();

        let (x1s, y1s) = left.split_once(',').unwrap();
        let (x2s, y2s) = right.split_once(',').unwrap();

        let x1 = x1s.parse::<i32>().unwrap();
        let x2 = x2s.parse::<i32>().unwrap();
        let y1 = y1s.parse::<i32>().unwrap();
        let y2 = y2s.parse::<i32>().unwrap();

        if x1 == x2 {
            for y in min(y1, y2)..=max(y1, y2) {
                points.entry((x1, y)).and_modify(|x| *x += 1).or_insert(1);
            }
        } else if y1 == y2 {
            for x in min(x1, x2)..=max(x1, x2) {
                points.entry((x, y1)).and_modify(|x| *x += 1).or_insert(1);
            }
        } else {
            let steps = max((x2 - x1).abs(), (y2 - y1).abs());
            for s in 0..=steps {
                let point = match (x1 < x2, y1 < y2) {
                    (true, true) => (x1 + s, y1 + s),
                    (false, true) => (x1 - s, y1 + s),
                    (false, false) => (x1 - s, y1 - s),
                    (true, false) => (x1 + s, y1 - s),
                };
                println!("{:?}", point);
                points.entry(point).and_modify(|x| *x += 1).or_insert(1);
            }
        }
    }
    println!("{:?}", points);

    let answer = points.values().filter(|x| **x > 1).count();

    let max_x = points.keys().map(|x| x.0).max().unwrap() + 1;
    let max_y = points.keys().map(|x| x.1).max().unwrap() + 1;

    for y in 0..max_y {
        for x in 0..max_x {
            let p = points.get(&(x, y));
            if p.is_some() {
                print!("{}", p.unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("{}", answer);
}
