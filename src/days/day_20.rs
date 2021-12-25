use crate::get_adjacent;
use crate::utils::get_adjacent_optionals;
use std::collections::HashMap;
use std::num::ParseIntError;

pub fn solve_day_20(input: &str) {
    let (algo_str, img) = input.split_once("\n\n").unwrap();

    let algo = algo_str
        .trim()
        .chars()
        .flat_map(|c| match c {
            '#' => Some(1),
            '.' => Some(0),
            _ => None,
        })
        .collect::<Vec<_>>();

    let first_algo = algo.first().unwrap().clone();

    let mut light_by_coord = HashMap::new();

    img.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            let is_lit = match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Invalid character"),
            };
            let (x, y) = (col as i32, row as i32);
            light_by_coord.insert((x, y), is_lit);
            for (n_x, n_y) in get_coords_surrounding((x, y)) {
                light_by_coord.entry((n_x, n_y)).or_insert(0);
            }
        })
    });

    for transformation in 1..=2 {
        let default_val = if first_algo == 1 {
            transformation % 2
        } else {
            0
        };

        let mut updates = vec![];

        for ((x, y), light) in &light_by_coord {
            let mut surrounding_points = get_coords_surrounding((*x, *y));

            // Insert middle point in the middle
            surrounding_points.insert(4, (*x, *y));

            let binary_val_as_str = surrounding_points
                .iter()
                .cloned()
                .map(|(x, y)| {
                    light_by_coord
                        .get(&(x, y))
                        .unwrap_or(&default_val)
                        .to_string()
                })
                .collect::<String>();

            let algo_idx = usize::from_str_radix(&binary_val_as_str, 2).unwrap();

            let new_val = algo[algo_idx];
            if new_val != *light {
                updates.push(((x.clone(), y.clone()), new_val));
            }
        }

        updates.iter().cloned().for_each(|((x, y), light)| {
            light_by_coord.insert((x, y), light);
            for (n_x, n_y) in get_coords_surrounding((x, y)) {
                light_by_coord.entry((n_x, n_y)).or_insert(default_val);
            }
        });
    }

    let lit_pixels = light_by_coord.values().filter(|x| **x == 1).count();

    let incorrect_answers = vec![4807, 4825, 4975, 5026, 5148, 5224, 5485, 5860];

    println!("{}", lit_pixels);
    if incorrect_answers.contains(&lit_pixels) {
        println!("Incorrect answer: {}", lit_pixels);
    }
}

fn print_map(lights: &HashMap<(i32, i32), i32>) {
    let min_x = lights.keys().map(|(x, _)| x).min().unwrap().clone();
    let min_y = lights.keys().map(|(_, y)| y).min().unwrap().clone();
    let max_x = lights.keys().map(|(x, _)| x).max().unwrap().clone();
    let max_y = lights.keys().map(|(_, y)| y).max().unwrap().clone();
    let add_min_x = if min_x < 0 { min_x.abs() } else { 0 };
    let add_min_y = if min_y < 0 { min_y.abs() } else { 0 };

    let mut grid =
        vec![vec![' '; (max_x + add_min_x + 1) as usize]; (max_y + add_min_y + 1) as usize];

    for ((x, y), light) in lights {
        grid[(*y + add_min_y) as usize][(*x + add_min_x) as usize] =
            if *light == 1 { '#' } else { '.' };
    }

    println!();
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

fn get_coords_surrounding(point: (i32, i32)) -> Vec<(i32, i32)> {
    return vec![
        (point.0 - 1, point.1 - 1),
        (point.0, point.1 - 1),
        (point.0 + 1, point.1 - 1),
        (point.0 - 1, point.1),
        (point.0 + 1, point.1),
        (point.0 - 1, point.1 + 1),
        (point.0, point.1 + 1),
        (point.0 + 1, point.1 + 1),
    ];
}

pub fn solve_day_20_1(input: &str) {
    let (algo_str, img) = input.split_once("\n\n").unwrap();

    let algo = algo_str
        .trim()
        .chars()
        .flat_map(|c| {
            if c == '#' {
                Option::Some(1)
            } else if c == '.' {
                Option::Some(0)
            } else {
                Option::None
            }
        })
        .collect::<Vec<_>>();

    let mut width = 0;
    let mut img_vec = vec![];

    for line in img.lines() {
        width = line.len();

        line.chars().for_each(|c| {
            img_vec.push(if c == '#' {
                1
            } else if c == '.' {
                0
            } else {
                panic!("Unknown char: {}", c)
            });
        });
    }

    //let (padded_img, padded_width) = pad_image(&img_vec, width, 10);

    let answer = transform_image(&algo, &img_vec, width, 2);

    //print_grid(&answer, padded_width, 0);

    let lit_pixels = answer.iter().filter(|&&x| x == 1).count();

    let incorrect_answers = vec![4807, 5860];
    println!("{}", lit_pixels);
    if incorrect_answers.contains(&lit_pixels) {
        println!("Incorrect answer: {}", lit_pixels);
    }
}

fn transform_image(algo: &[i32], img: &[i32], width: usize, num_transformations: i32) -> Vec<i32> {
    print_grid(img, width, 0);
    if num_transformations == 0 {
        return img.to_vec();
    }

    let result_img = (0..img.len())
        .map(|i| {
            let adj = get_adjacent_optionals(&img, i, width, true, true)
                .iter()
                .map(|x| x.unwrap_or(0))
                .collect::<Vec<_>>();
            let maps_to_algo_idx = vec_into_dec(&adj, &img);
            algo[maps_to_algo_idx as usize]
        })
        .collect::<Vec<_>>();

    transform_image(algo, &result_img, width, num_transformations - 1)
}

fn vec_into_dec(indices: &[usize], img: &[i32]) -> i32 {
    i32::from_str_radix(
        &indices
            .iter()
            .map(|i| img[*i].to_string())
            .collect::<String>(),
        2,
    )
    .unwrap()
}

#[cfg(test)]
mod test_vec_into_bin {
    use crate::days::day_20::vec_into_dec;
    use crate::get_adjacent;

    #[test]
    fn test_example() {
        let img = [0, 0, 0, 1, 0, 0, 0, 1, 0];

        let mut indices = get_adjacent(&img, 4, 3, true, true);

        let actual_bin = vec_into_dec(&indices, &img);

        assert_eq!(actual_bin, 34);
    }
}

fn pad_image(img: &[i32], width: usize, pad_size: usize) -> (Vec<i32>, usize) {
    let mut padded_img = vec![];
    let rows = img.len() / width;
    let new_width = width + 2 * pad_size;

    for _ in 0..new_width * pad_size {
        padded_img.push(0);
    }

    for r in 0..rows {
        for _ in 0..pad_size {
            padded_img.push(0);
        }

        for r_i in r * width..(r + 1) * width {
            padded_img.push(img[r_i]);
        }

        for _ in 0..pad_size {
            padded_img.push(0);
        }
    }

    for _ in 0..new_width * pad_size {
        padded_img.push(0);
    }

    (padded_img, new_width)
}

#[cfg(test)]
mod test_pad_image {
    use crate::days::day_20::pad_image;

    #[test]
    fn test_padding_1() {
        let img = vec![0, 0, 0, 1, 0, 0, 0, 1, 0];

        let expected_padded_img = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
        ];

        let (actual_padded_img, new_width) = pad_image(&img, 3, 1);

        assert_eq!(new_width, 5);
        assert_eq!(actual_padded_img, expected_padded_img);
    }
}

/// Print vec of 0 and 1 as grid
fn print_grid(grid: &[i32], width: usize, i: usize) {
    let mut row = String::new();
    grid.iter().enumerate().for_each(|(zz, c)| {
        if zz == i {
            row.push_str("O");
        } else {
            row.push_str(if *c == 1 { "#" } else { "." });
        }
        if row.len() == width {
            println!("{}", row);
            row = String::new();
        }
    });
    println!();
}
