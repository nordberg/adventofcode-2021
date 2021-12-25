pub fn get_adjacent<T>(
    v: &[T],
    i: usize,
    width: usize,
    include_diagonals: bool,
    include_mid: bool,
) -> Vec<usize> {
    let i_left = if i > 0 {
        if i % width == 0 && i != 0 {
            Option::None
        } else {
            Option::Some(i - 1)
        }
    } else {
        Option::None
    };
    let i_right = if i + 1 < v.len() {
        if (i + 1) % width == 0 {
            Option::None
        } else {
            Option::Some(i + 1)
        }
    } else {
        Option::None
    };
    let i_above = if i >= width {
        Option::Some(i - width)
    } else {
        Option::None
    };
    let i_left_above = if i_above.is_some() && i_left.is_some() {
        Option::Some(i_above.unwrap() - 1)
    } else {
        Option::None
    };
    let i_right_above = if i_above.is_some() && i_right.is_some() {
        Option::Some(i_above.unwrap() + 1)
    } else {
        Option::None
    };
    let i_below = if i + width < v.len() {
        Option::Some(i + width)
    } else {
        Option::None
    };
    let i_below_left = if i_below.is_some() && i_left.is_some() {
        Option::Some(i_below.unwrap() - 1)
    } else {
        Option::None
    };
    let i_below_right = if i_below.is_some() && i_right.is_some() {
        Option::Some(i_below.unwrap() + 1)
    } else {
        Option::None
    };

    let mut adj = if include_diagonals {
        vec![
            i_above,
            i_left,
            i_below,
            i_right,
            i_below_left,
            i_below_right,
            i_right_above,
            i_left_above,
        ]
        .iter()
        .flat_map(|x| *x)
        .collect::<Vec<usize>>()
    } else {
        vec![i_above, i_left, i_below, i_right]
            .iter()
            .flat_map(|x| *x)
            .collect::<Vec<usize>>()
    };

    if include_mid {
        adj.push(i);
    }

    adj.sort();

    adj
}

pub fn get_adjacent_optionals<T>(
    v: &[T],
    i: usize,
    width: usize,
    include_diagonals: bool,
    include_mid: bool,
) -> Vec<Option<usize>> {
    let i_left = if i > 0 {
        if i % width == 0 && i != 0 {
            Option::None
        } else {
            Option::Some(i - 1)
        }
    } else {
        Option::None
    };
    let i_right = if i + 1 < v.len() {
        if (i + 1) % width == 0 {
            Option::None
        } else {
            Option::Some(i + 1)
        }
    } else {
        Option::None
    };
    let i_above = if i >= width {
        Option::Some(i - width)
    } else {
        Option::None
    };
    let i_left_above = if i_above.is_some() && i_left.is_some() {
        Option::Some(i_above.unwrap() - 1)
    } else {
        Option::None
    };
    let i_right_above = if i_above.is_some() && i_right.is_some() {
        Option::Some(i_above.unwrap() + 1)
    } else {
        Option::None
    };
    let i_below = if i + width < v.len() {
        Option::Some(i + width)
    } else {
        Option::None
    };
    let i_below_left = if i_below.is_some() && i_left.is_some() {
        Option::Some(i_below.unwrap() - 1)
    } else {
        Option::None
    };
    let i_below_right = if i_below.is_some() && i_right.is_some() {
        Option::Some(i_below.unwrap() + 1)
    } else {
        Option::None
    };

    let x = vec![
        i_left_above,
        i_above,
        i_right_above,
        i_left,
        Option::Some(i),
        i_right,
        i_below_left,
        i_below,
        i_below_right,
    ];

    x
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_upper_left_corner_without_diagonals() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let width = 3;

        let expected_adj = vec![1, 3];

        let actual_adj = super::get_adjacent(&v, 0, width, false, false);

        assert_eq!(expected_adj, actual_adj);
    }

    #[test]
    fn test_upper_left_corner_with_diagonals() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

        let width = 3;

        let expected_adj = vec![1, 3, 4];

        let actual_adj = super::get_adjacent(&v, 0, width, true, false);

        assert_eq!(expected_adj, actual_adj);
    }

    #[test]
    fn test_upper_mid_without_diagonals() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

        let width = 3;

        let expected_adj = vec![0, 2, 4];

        let actual_adj = super::get_adjacent(&v, 1, width, false, false);

        assert_eq!(expected_adj, actual_adj);
    }

    #[test]
    fn test_upper_mid_with_diagonals() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

        let width = 3;

        let expected_adj = vec![0, 2, 3, 4, 5];

        let actual_adj = super::get_adjacent(&v, 1, width, true, false);

        assert_eq!(expected_adj, actual_adj);
    }

    #[test]
    fn test_upper_right_corner_without_diagonals() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

        let width = 3;

        let expected_adj = vec![1, 5];

        let actual_adj = super::get_adjacent(&v, 2, width, false, false);

        assert_eq!(expected_adj, actual_adj);
    }

    #[test]
    fn test_upper_right_corner_with_diagonals() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

        let width = 3;

        let expected_adj = vec![1, 4, 5];

        let actual_adj = super::get_adjacent(&v, 2, width, true, false);

        assert_eq!(expected_adj, actual_adj);
    }

    #[test]
    fn test_left_edge_mid_without_diagonals() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

        let width = 3;

        let expected_adj = vec![0, 4, 6];

        let actual_adj = super::get_adjacent(&v, 3, width, false, false);

        assert_eq!(expected_adj, actual_adj);
    }

    #[test]
    fn test_left_edge_mid_with_diagonals() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

        let width = 3;

        let expected_adj = vec![0, 1, 4, 6, 7];

        let actual_adj = super::get_adjacent(&v, 3, width, true, false);

        assert_eq!(expected_adj, actual_adj);
    }
}
