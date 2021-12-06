use std::collections::{VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn fish_population(reader: BufReader<File>) -> i64 {
    let mut fish_pop = VecDeque::with_capacity(9);

    for _ in 0..9 {
        fish_pop.push_back(0);
    }

    for line in reader.lines() {
        let read_line = line.expect("Error reading the line");
        let fish_timers = read_line.split(",");
        for fish_timer_str in fish_timers {
            let fish_timer = fish_timer_str.parse::<i32>().unwrap();
            fish_pop[fish_timer as usize] += 1;
        }
    }

    for _ in 0..256 {
        let num_respawning_fishes = fish_pop.get(0).unwrap().clone();
        fish_pop.rotate_left(1);
        fish_pop[6] += num_respawning_fishes;
    }

    fish_pop.iter().sum::<i64>()
}