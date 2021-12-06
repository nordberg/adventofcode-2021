use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Bits {
    ones: i32,
    zeros: i32,
}

pub fn power_consumption() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut bit_histogram =  BTreeMap::new();

    for line in reader.lines() {
        let bar = line.expect("Error reading the line");
        let foo = bar.trim().chars();
        foo.enumerate().for_each(|(i, c)| {
            if c == '1' {
                bit_histogram.entry(i).and_modify(|b: &mut Bits| b.ones += 1).or_insert(Bits { ones: 1, zeros: 0 });
            } else {
                bit_histogram.entry(i).and_modify(|b: &mut Bits| b.zeros +=1 ).or_insert(Bits { ones: 0, zeros: 1 });
            }
        });
    }

    let mut power_rate = String::new();
    let mut gamma_rate = String::new();

    bit_histogram.keys().for_each(|k| {
        let b = bit_histogram.get(k).unwrap();
        if b.ones > b.zeros {
            power_rate.insert(*k, '1');
            gamma_rate.insert(*k, '0');
        } else if b.ones < b.zeros {
            power_rate.insert(*k, '0');
            gamma_rate.insert(*k, '1');
        }
    });

    println!("Power rate: {:?}", isize::from_str_radix(&*power_rate, 2).unwrap()); // 1143
    println!("Gamma rate: {:?}", isize::from_str_radix(&*gamma_rate, 2).unwrap()); // 2952


    Ok(())
}

fn calculate_generator_rating(bit_strs: Vec<&str>, idx: usize) -> isize {
    if bit_strs.len() == 1 {
        return isize::from_str_radix(&*bit_strs.get(0).unwrap(), 2).unwrap();
    }

    let mut counter = Bits {
        ones: 0,
        zeros: 0,
    };

    bit_strs.iter().for_each(|bit_str| {
        if (bit_str.chars().nth(idx).unwrap()) == '1' {
            counter.ones += 1;
        } else {
            counter.zeros += 1;
        }
    });

    let most_common_bit = if counter.ones >= counter.zeros {
        '1'
    } else {
        '0'
    };

    calculate_generator_rating(bit_strs.iter().filter(|bit_str| bit_str.chars().nth(idx).unwrap() == most_common_bit).map(|s| *s).collect(), idx + 1)
}
