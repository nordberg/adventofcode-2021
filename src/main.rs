mod days;
mod utils;

#[macro_use]
extern crate strum_macros;

use crate::utils::get_adjacent;
use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Binary, Debug, Formatter, Write};
use std::fs::{read, File};
use std::io::{self, prelude::*, BufReader};
use std::ops::Index;
use std::usize;
use crate::days::day_13::fold;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut points = vec![];
    let mut folds = vec![];

    for (i, line) in reader.lines().enumerate() {
        let l = line.expect("Could not read line");

        if l.is_empty() {
            continue;
        }

        if i < 904 {
            let (x, y) = l.split_once(',').unwrap();
            points.push((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
        } else {
            let foo = l.split_whitespace().collect::<Vec<_>>();
            let foobar = foo.get(2).unwrap();
            let (x_or_y, x_or_y_val) = foobar.split_once('=').unwrap();
            folds.push((String::from(x_or_y), x_or_y_val.parse::<i32>().unwrap()));
        }
    }

    folds.reverse();

    fold(points.clone(), folds.clone());

    Ok(())
}