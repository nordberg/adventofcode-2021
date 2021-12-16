use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet};
use crate::get_adjacent;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn solve_day_15(input: &str) {
    let mut points: Vec<u32> = vec![];
    let mut width = 0;

    for (i, line) in input.lines().enumerate() {
        let mut line_points = vec![];
        line.chars().for_each(|c| {
            line_points.push(c.to_digit(10).unwrap());
        });

        let mut adds = vec![];
        for i in 1..=4 {
            let copied = line_points.clone().iter().map(|x| do_this(i, x)).collect::<Vec<u32>>();
            adds.push(copied);
        }

        for add in adds {
            line_points.extend(add);
        }
        width = line_points.len();

        points.extend(line_points);
    }

    let mut adds = vec![];

    for i in 1..=4 {
        let nex = points.clone().iter().map(|x| do_this(i, x)).collect::<Vec<u32>>();
        adds.push(nex);
    }

    points.extend(adds.iter().flatten().cloned());

    //print_grid(&points, width);

    let answer = lowest_risk_to_end(&points, width);
    if answer.is_some() {
        println!("The lowest risk to end is {}", answer.unwrap());
    } else {
        println!("No answer found");
    }
}

fn lowest_risk_to_end(points: &[u32], width: usize) -> Option<u32> {
    let end_point = points.len() - 1;
    let mut lowest_so_far = HashMap::new();
    let mut b_heap = BinaryHeap::new();
    b_heap.push(Reverse((0, 0)));

    lowest_so_far.insert(0, 0);  // 0 is the start point

    let mut visited = HashSet::new();

    loop {
        let lowest_idx = lowest_so_far.iter()
            .filter(|(i, _)| !visited.contains(*i))
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0
            .clone();

        visited.insert(lowest_idx);

        let adj = get_adjacent(&points, lowest_idx, width, false);

        let new_risks_for_adj = adj.iter().filter(|i| !visited.contains(*i)).map(|&i| {
            let risk_to_i = lowest_so_far.get(&i).unwrap_or(&u32::MAX).clone();
            let risk_until_here = lowest_so_far.get(&lowest_idx).unwrap_or(&u32::MAX).clone();
            (i, min(risk_to_i, risk_until_here + points[i]))
        }).collect::<Vec<(usize, u32)>>();

        for (j, risk) in new_risks_for_adj {
            lowest_so_far.insert(j, risk);
        }

        if lowest_so_far.contains_key(&end_point) {
            break;
        }
    }

    lowest_so_far.get(&end_point).cloned()
}

fn do_this(i: u32, x: &u32) -> u32 {
    let res = (x + i) % 9 ;
    if res == 0 {
        9
    } else {
        res
    }
}


/// Print grid from vector of points
fn print_grid(points: &Vec<u32>, width: usize) {
    for i in 0..points.len() {
        if i % width == 0 {
            println!()
        }
        print!("{}", points[i]);
    }
}

fn lowest_risk(points: &[u32], current_point: usize, goal: usize, width: usize, mut visited: HashSet<usize>) -> u32 {
    if current_point == goal {
        return 0;
    }
    visited.insert(current_point);

    points[current_point] + get_adjacent(points, current_point, width, false)
        .iter()
        .filter(|i| !visited.contains(i))
        .map(|i| lowest_risk(points, *i, goal, width, visited.clone()))
        .min()
        .unwrap_or(0)
}