mod days;
mod utils;

#[macro_use]
extern crate strum_macros;

use crate::days::day_13::fold;
use crate::days::day_14::solve_day_14;
use crate::days::day_15::solve_day_15;
use crate::days::day_16::solve_day_16;
use crate::days::day_17::solve_day_17;
use crate::days::day_20::solve_day_20;
use crate::days::day_25::solve;
use crate::days::day_5::solve_day_5;
use crate::days::day_9::solve_day_9;
use crate::utils::get_adjacent;
use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Binary, Debug, Formatter, Write};
use std::fs::{read, File};
use std::io::{self, prelude::*, BufReader};
use std::ops::Index;
use std::usize;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    solve_day_15(&input);

    Ok(())
}
