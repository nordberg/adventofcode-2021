use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_steps() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut positions = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let splitted = line.split(",").collect::<Vec<_>>();

        for split in splitted {
            let x = split.parse::<i32>();
            positions.push(x.unwrap());
        }
    }

    let min_val = positions.iter().min().unwrap().clone();
    let max_val = positions.iter().max().unwrap().clone();

    let mut my_map = HashMap::new();

    for available_positions in min_val..max_val {
        for crab_position in positions.iter() {
            let steps = (crab_position.clone() - available_positions).abs();
            let fuel = (steps * (steps + 1)) / 2;
            my_map.entry(available_positions).and_modify(|e| *e += fuel).or_insert(fuel);
        }
    }

    let answer = my_map.values().min().unwrap().clone();


    println!("{:?}", answer);
    Ok(())

}

#[derive(Debug)]
struct BingoCard {
    rows: Vec<Vec<i32>>,
    has_won: bool
}

impl BingoCard {
    pub fn has_bingo(&self, bingo_numbers: HashSet<i32>) -> bool {
        self.has_horizontal_bingo(bingo_numbers.clone())
            || self.has_diagonal_bingo(bingo_numbers.clone())
            || self.has_vertical_bingo(bingo_numbers.clone())
    }

    fn has_vertical_bingo(&self, bingo_numbers: HashSet<i32>) -> bool {
        let row_size = self.rows[0].len();
        let mut columns = vec![];
        for i in 0..row_size {
            let mut column = vec![];
            for row in &self.rows {
                column.push(row[i].clone());
            }
            columns.push(column);
        }

        columns.iter().any(|column| column.iter().all(|number| bingo_numbers.contains(number)))
    }

    fn has_horizontal_bingo(&self, bingo_numbers: HashSet<i32>) -> bool {
        self.rows.iter().any(|row| row.iter().all(|&n| bingo_numbers.contains(&n)))
    }

    fn has_diagonal_bingo(&self, bingo_numbers: HashSet<i32>) -> bool {
        let row_size = self.rows[0].len();
        let mut diagonal_row = vec![];
        let mut diagonal_row2 = vec![];
        for i in 0..row_size {
            diagonal_row.push(self.rows[i][i].clone());
            diagonal_row2.push(self.rows[i][row_size - i - 1].clone());
        }

        diagonal_row.iter().all(|&n| bingo_numbers.contains(&n)) || diagonal_row2.iter().all(|&n| bingo_numbers.contains(&n))
    }

    fn numbers_not_in_bingo(&self, bingo_numbers: HashSet<i32>) -> Vec<i32> {
        self.rows.iter().flat_map(|row| row.iter()).filter(|&n| !bingo_numbers.contains(&n)).cloned().collect()
    }

    fn win(&mut self) {
        self.has_won = true;
    }
}

fn bingo() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut reader_lines = reader.lines();

    let bingo_numbers = reader_lines.next().unwrap().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    reader_lines.next();

    let mut bingo_cards = vec![];

    loop {
        let lines = vec![
            reader_lines.next(),
            reader_lines.next(),
            reader_lines.next(),
            reader_lines.next(),
            reader_lines.next()
        ];
        let mut bingo_card = BingoCard { rows: vec![], has_won: false };
        for line in lines {
            let numbers = line.unwrap().unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            bingo_card.rows.push(numbers);
        }
        bingo_cards.push(bingo_card);
        if reader_lines.next().is_none() {
            break;
        }
    }

    let mut bingo_numbers_set = HashSet::new();

    for number in bingo_numbers {
        bingo_numbers_set.insert(number);

        for card in &mut bingo_cards {
            if card.has_won {
                continue;
            }

            if card.has_bingo(bingo_numbers_set.clone()) {
                println!("{:?}", card);
                println!("{:?}", card.numbers_not_in_bingo(bingo_numbers_set.clone()).iter().sum::<i32>() * number);
                card.win();
            }
        }
    }
    Ok(())
}