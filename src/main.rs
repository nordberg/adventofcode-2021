mod days;

#[macro_use]
extern crate strum_macros;

use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fmt::Binary;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::Index;


fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);


    for line in reader.lines() {
        let read_line = line.expect("Error reading the line");
    }

    Ok(())
}
