use std::collections::HashSet;
use crate::get_adjacent;

pub fn solve_day_9(input: &str) {
    let mut input_vec = vec![];
    let mut width = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        chars.for_each(|c| input_vec.push(c.to_digit(10).unwrap()));
        if width == 0 {
            width = input_vec.len() as usize;
        }
    }

    find_lowest_points(input_vec, width);
}

fn find_lowest_points(points: Vec<u32>, width: usize) -> Vec<u32> {
    let lowest = points.iter().enumerate().flat_map(|(i, point)| {
        let adj = get_adjacent(&points, i, width, false);
        if adj.iter().all(|j| points[*j] > *point) {
            Option::Some(i)
        } else {
            Option::None
        }
    }).collect::<HashSet<_>>();

    println!("{:?}", lowest);

    let mut basins = lowest.iter().map(|i| {
        let mut visited = HashSet::new();
        find_basin_size(&points, width, *i, &mut visited) as u32
    }).collect::<Vec<u32>>();

    basins.sort();
    basins.reverse();

    println!("{:?}", basins);
    println!("{:?}", basins[0] * basins[1] * basins[2]);

    basins
}

fn find_basin_size(points: &[u32], width: usize, index: usize, visited: &mut HashSet<usize>) -> i32 {
    if visited.contains(&index) {
        return 0;
    }

    visited.insert(index);

    let adj_points = get_adjacent(points, index, width, false);

    1 + adj_points.iter().map(|i| {
        if points[*i] < 9 {
            find_basin_size(points, width, *i, visited)
        } else {
            0
        }
    }).sum::<i32>()
}

#[cfg(test)]
mod test_find_basins {
    use crate::days::day_9::{find_basin_size, find_lowest_points};

    #[test]
    fn test_basin_size() {
        let points: Vec<u32> = vec![
            0, 9,
            9, 9
        ];

        let expected_basin_sizes = vec![1];

        assert_eq!(expected_basin_sizes, find_lowest_points(points, 2));
    }

    #[test]
    fn test_basin_size_2() {
        let points: Vec<u32> = vec![
            0, 5, 9,
            2, 3, 9,
            3, 9, 4
        ];

        let expected_basin_sizes = vec![5, 1];

        assert_eq!(expected_basin_sizes, find_lowest_points(points, 3));
    }

    #[test]
    fn test_multiple_basins() {
        let points = vec![
            0, 1, 9, 9,
            9, 9, 9, 9,
            9, 9, 5, 7,
            9, 9, 2, 9
        ];

        let expected_basin_sizes = vec![2, 3];

        assert_eq!(expected_basin_sizes, find_lowest_points(points, 4));
    }
}