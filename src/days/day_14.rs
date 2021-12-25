use std::cmp::min;
use std::collections::HashMap;
use std::fmt::Debug;

pub fn solve_day_14(input: &str) {
    let mut chunks = input.split("\n\n");

    let mut start_chunk = chunks.next().unwrap();

    let mut rules = HashMap::new();

    for rule in chunks.next().unwrap().split("\n") {
        let (key, value) = rule.split_once(" -> ").unwrap();
        let mut ccc = key.chars();
        let char1 = ccc.next().unwrap();
        let char2 = ccc.next().unwrap();
        let unwraps_to = format!(
            "{}{}{}",
            char1.to_string(),
            value.to_string(),
            char2.to_string()
        );
        rules.insert((char1, char2), unwraps_to);
    }

    let result = solve_solve_part_2(String::from(start_chunk), rules, 0);

    /*    let mut result_map = HashMap::new();
        result.chars().for_each(|c| {
            result_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        });

        let mut minn = result_map.values().min().unwrap();
        let mut maxx = result_map.values().max().unwrap();

        println!("{:?}", (maxx - minn));
    */
}

fn solve_solve_part_2(input: String, rules: HashMap<(char, char), String>, steps: i32) -> String {
    let mut counter: HashMap<(char, char), i64> = HashMap::new();
    let chars = input.chars().collect::<Vec<_>>();

    chars.windows(2).for_each(|w| {
        let char_window = (w[0], w[1]);
        counter
            .entry(char_window)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });

    for _ in 0..40 {
        let mut adds = vec![];
        for (char_pair, maps_to) in &rules {
            let count = counter.get(char_pair).unwrap_or(&0).clone();
            let ccc = maps_to.chars().collect::<Vec<_>>();
            ccc.windows(2).for_each(|w| {
                let w_window = (w[0], w[1]);
                adds.push((w_window, count));
            });
            adds.push((*char_pair, -count));
        }
        for (entry, add) in adds {
            counter
                .entry(entry)
                .and_modify(|e| *e += add)
                .or_insert(add);
        }
    }

    println!("{:?}", counter);

    let mut count_by_char = HashMap::new();

    counter.iter().for_each(|(k, v)| {
        count_by_char
            .entry(k.0)
            .and_modify(|e| *e += *v)
            .or_insert(*v);
        count_by_char
            .entry(k.1)
            .and_modify(|e| *e += *v)
            .or_insert(*v);
    });

    //count_by_char.entry('N').and_modify(|e| *e -= 1).or_insert(0);
    //count_by_char.entry('B').and_modify(|e| *e -= 1).or_insert(0);
    // H 161
    // N 865
    // C 298
    // B 1749

    let mut r = vec![];

    count_by_char.iter().for_each(|(k, v)| {
        println!("{} {}", k, v / 2);
        if (k.clone() == 'C' || k.clone() == 'K') {
            r.push((v / 2) + 1);
        } else {
            r.push(v / 2);
        }
    });

    let maxx = r.iter().max().unwrap();
    let minn = r.iter().min().unwrap();

    let answer = maxx - minn;

    println!("{:?}", r);
    println!("Answer: {:?}", answer);

    String::new()
}

fn solve_solve_part_1(
    mut input: String,
    rules: &HashMap<(char, char), String>,
    steps: i32,
) -> String {
    println!("{}", steps);
    if steps == 40 {
        return input;
    }
    let chars = input.chars().collect::<Vec<_>>();
    let mut result = String::new();

    chars.windows(2).for_each(|window| {
        let bigram = (window[0], window[1]);
        let unwrapped = rules.get(&bigram);
        match unwrapped {
            Some(value) => result.push_str(value),
            None => result.push_str(format!("{}{}", bigram.0, bigram.1).as_str()),
        }
    });

    return solve_solve_part_1(result, &rules, steps + 1);
}
