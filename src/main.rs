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

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.expect("Could not read line");
    }

    Ok(())
}
