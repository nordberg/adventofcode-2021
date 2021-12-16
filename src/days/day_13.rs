use std::collections::HashSet;

pub fn fold(points: Vec<(i32, i32)>, mut folds: Vec<(String, i32)>) {
    if folds.is_empty() {
        let grid = points_to_grid(&points);
        print_paper(&grid);
        return;
    }
    let (x_or_y_fold, fold_at) = folds.pop().unwrap();
    let grid = points_to_grid(&points);

    let result_points = match x_or_y_fold.as_str() {
        "x" => {
            fold_rows(fold_at, &grid, false)
        }
        "y" => {
            let transposed_grid = transpose_grid(&grid);
            fold_rows(fold_at, &transposed_grid, true)
        }
        _ => panic!("Unknown fold"),
    };

    fold(result_points, folds);
}

fn split_vec_and_reverse_right<T: Clone>(v: &Vec<T>, at: usize) -> (Vec<T>, Vec<T>) {
    let (_left, _right) = v.split_at(at);
    let mut left = _left.to_vec();
    let mut right = _right.to_vec();
    right.reverse();
    (left.to_vec(), right.to_vec())
}

fn fold_rows(fold_at: i32, grid: &Vec<Vec<i32>>, is_transposed: bool) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for (col, row) in grid.iter().enumerate() {
        let (left, right) = split_vec_and_reverse_right(row, fold_at as usize);

        for ((row_idx, left_val), right_val) in left.iter().enumerate().zip(right.iter()) {
            if *left_val > 0 || *right_val > 0 {
                if is_transposed {
                    result.push((col as i32, row_idx as i32))
                } else {
                    result.push((row_idx as i32, col as i32))
                }
            }
        }
    }
    result
}


fn transpose_grid(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut transposed_grid = vec![vec![0; grid.len()]; grid[0].len()];

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            transposed_grid[col_idx][row_idx] = *col;
        }
    }

    transposed_grid
}

fn points_to_grid(points: &[(i32, i32)]) -> Vec<Vec<i32>> {
    let max_x = points.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = points.iter().map(|(_, y)| *y).max().unwrap();
    let mut grid = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];

    for (x, y) in points {
        grid[*y as usize][*x as usize] = 1;
    }

    grid
}

fn print_paper(paper: &Vec<Vec<i32>>) {
    for row in paper {
        for cell in row {
            if *cell == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test_fold_row {
    use crate::days::day_13::fold_rows;

    #[test]
    fn test_folding_row() {
        let fold_at = 2;
        let grid = vec![
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 1],
            vec![0, 1, 0, 0],
        ];

        let folded_grid = fold_rows(fold_at, &grid, false);

        assert_eq!(folded_grid, vec![
            (1, 0),
            (0, 1),
            (1, 1),
            (1, 2)
        ]);
    }

    fn test_folding_minimal_grid() {
        let fold_at = 1;
        let grid = vec![
            vec![0, 1],
        ];

        let folded_grid = fold_rows(fold_at, &grid, false);

        assert_eq!(folded_grid, vec![(0, 0)]);
    }

    fn test_folding_minimal_grid_with_rows() {
        let fold_at = 1;
        let grid = vec![
            vec![0, 1],
            vec![0, 1]
        ];

        let folded_grid = fold_rows(fold_at, &grid, false);

        assert_eq!(folded_grid, vec![(0, 0), (0, 1)]);
    }

    fn test_folding_minimal_grid_with_rows_left_ones() {
        let fold_at = 1;
        let grid = vec![
            vec![1, 0],
            vec![1, 0]
        ];

        let folded_grid = fold_rows(fold_at, &grid, false);

        assert_eq!(folded_grid, vec![(0, 0), (0, 1)]);
    }

    fn test_folding_minimal_grid_with_rows_only_ones() {
        let fold_at = 0;
        let grid = vec![
            vec![1, 1],
            vec![1, 1]
        ];

        let folded_grid = fold_rows(fold_at, &grid, false);

        assert_eq!(folded_grid, vec![(0, 0), (0, 1)]);
    }
}

#[cfg(test)]
mod split_tests {
    use crate::days::day_13::split_vec_and_reverse_right;

    #[test]
    fn test_split_vec_and_reverse_right() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (left, right) = split_vec_and_reverse_right(&v, 3);
        assert_eq!(left, vec![1, 2, 3]);
        assert_eq!(right, vec![10, 9, 8, 7, 6, 5, 4]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FoldDirection {
    Vertical,
    Horizontal,
}

fn fold_paper(points: &HashSet<(i32, i32)>, fold_at: i32, fold_direction: FoldDirection) -> HashSet<(i32, i32)> {
    points.iter().map(|(x, y)| {
        match fold_direction {
            FoldDirection::Vertical => {
                if *x > fold_at {
                    (fold_at - (*x - fold_at), *y)
                } else {
                    (*x, *y)
                }
            }
            FoldDirection::Horizontal => {
                if *y > fold_at {
                    (*x, fold_at - (*y - fold_at))
                } else {
                    (*x, *y)
                }
            }
        }
    }).collect::<HashSet<(i32, i32)>>()
}