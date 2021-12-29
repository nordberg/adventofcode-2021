use std::cmp::{max, min};
use memoize::memoize;
use lazy_static;

struct Dice {
    value: i32,
    dice_rolls: i32,
}

impl Dice {
    fn roll(&mut self) -> i32 {
        self.value = (self.value % 100) + 1;
        self.dice_rolls += 1;

        self.value
    }
}

pub fn solve_day_21() {
    let mut p1_pos = 6;
    let mut p2_pos = 4;
    let mut dice = Dice { value: 0, dice_rolls: 0 };

    let mut p1_score = 0;
    let mut p2_score = 0;

    for turn in 1.. {
        if p1_score >= 1000 || p2_score >= 1000 {
            println!("Turn: {}", turn);
            println!("P1: {}, P2: {}", p1_score, p2_score);
            let answer = min(p1_score, p2_score) * dice.dice_rolls;
            let wrong_answers = vec![
                264350,
                442668,
            ];

            if wrong_answers.contains(&answer) {
                println!("Wrong answer: {}", answer);
            } else {
                println!("New answer: {}", answer);
            }
            break;
        }

        let roll_1 = dice.roll();
        let roll_2 = dice.roll();
        let roll_3 = dice.roll();
        let roll_sum = roll_1 + roll_2 + roll_3;

        if turn % 2 == 1 {
            let new_pos = (p1_pos + roll_sum) % 10;
            p1_pos = if new_pos == 0 { 10 } else { new_pos };
            p1_score += p1_pos;
            println!("P1 rolls {}+{}+{}={} and moves to {}", roll_1, roll_2, roll_3, roll_sum, p1_pos);
        } else {
            let new_pos = (p2_pos + roll_sum) % 10;
            p2_pos = if new_pos == 0 { 10 } else { new_pos };
            p2_score += p2_pos;
            println!("P2 rolls {}+{}+{}={} and moves to {}", roll_1, roll_2, roll_3, roll_sum, p2_pos);
        }
    }
}

pub fn solve_day_21_pt2() {
    let p1_pos = 4;
    let p2_pos = 8;

    let result = next_roll(vec![], vec![], p1_pos, p2_pos, 0, 0);

    let wrong_answers = vec![
        510300,
        531441,
        510300,
        1320706553,
        11997614504960505
    ];
    let answer = max(result.0, result.1);

    if !wrong_answers.contains(&answer) {
        println!("New answer: {}", answer);
    } else {
        println!("Wrong answer: {}", answer);
    }
}

#[memoize]
fn sum_mem(rolls: Vec<i32>) -> i32 {
    assert_eq!(rolls.len(), 3);
    rolls.iter().sum::<i32>()
}

/// Keeps track of players' rolls up until three rolls are made.
#[memoize]
fn next_roll(p1_rolls: Vec<i32>, p2_rolls: Vec<i32>, p1_pos: i32, p2_pos: i32, p1_score: i32, p2_score: i32) -> (i64, i64) {
    let mut res_p1_wins = 0;
    let mut res_p2_wins = 0;

    // Game ended
    if p1_score >= 21 {
        return (1, 0);
    } else if p2_score >= 21 {
        return (0, 1);
    }

    // P1 turn
    if p1_rolls.len() == p2_rolls.len() && p1_rolls.len() == 3 {
        // Both have three rolls
        let new_p1_pos_c = p1_pos + sum_mem(p1_rolls);
        let new_p1_pos = if new_p1_pos_c % 10 == 0 { 10 } else { new_p1_pos_c % 10 };

        let new_p2_pos_c = p2_pos + sum_mem(p2_rolls);
        let new_p2_pos = if new_p2_pos_c % 10 == 0 { 10 } else { new_p2_pos_c % 10 };

        let (p1_wins, p2_wins) = next_roll(vec![], vec![], new_p1_pos, new_p2_pos, p1_score + new_p1_pos, p2_score + new_p2_pos);

        res_p1_wins += p1_wins;
        res_p2_wins += p2_wins;

    } else if p1_rolls.len() < 3 {
        for i in 1..=3 {
            let mut universe_rolls = p1_rolls.clone();
            universe_rolls.push(i);
            let (p1_wins, p2_wins) = next_roll(universe_rolls, p2_rolls.clone(), p1_pos, p2_pos, p1_score, p2_score);
            res_p1_wins += p1_wins;
            res_p2_wins += p2_wins;
        }
    } else if p2_rolls.len() < 3 {
        for i in 1..=3 {
            let mut universe_rolls = p2_rolls.clone();
            universe_rolls.push(i);
            let (p1_wins, p2_wins) = next_roll(p1_rolls.clone(), universe_rolls, p1_pos, p2_pos, p1_score, p2_score);
            res_p1_wins += p1_wins;
            res_p2_wins += p2_wins;
        }
    } else {
        panic!("Should not happen");
    }

    (res_p1_wins, res_p2_wins)
}
