use std::collections::HashSet;
use crate::get_adjacent;

const PART_TWO: bool = false;

fn count_flashes(mut oct: Vec<u32>, width: usize, steps: usize) -> Vec<u32> {
    let mut flash_count = 0;

    for step in 1..=steps {
        println!("Step {}", step);
        oct = oct.iter().map(|o| *o + 1).collect::<Vec<u32>>();
        flash_count += flash_octo(oct.as_mut(), width, 0, HashSet::new());
    }

    println!("{}", flash_count);
    oct
}

fn print_oct(oct: &[u32], width: usize) {
    for i in 0..oct.len() {
        if i % width == 0 {
            print!("\n");
        }
        let xx = if oct[i] < 10 { oct[i].to_string() } else { "#".to_string() };
        print!("{}", xx);
    }
    println!("\n");
}

fn flash_octo(oct: &mut Vec<u32>, width: usize, flashes: usize, mut flashing: HashSet<usize>) -> usize {
    let mut something_flashed = false;
    let mut fff = 0;
    for i in 0..oct.len() {
        if oct[i] > 9 && !flashing.contains(&i) {
            oct[i] = 0;
            something_flashed = true;
            fff += 1;
            flashing.insert(i);

            let adj = get_adjacent(oct.as_slice(), i, width, true);

            for a in adj {
                if !flashing.contains(&a) {
                    oct[a] += 1;
                }
            }
        }
    }

    if PART_TWO {
        if flashing.len() == oct.len() {
            panic!("All 100 are flashing");
        }
    }

    return if something_flashed {
        flash_octo(oct, width, flashes + fff, flashing)
    } else {
        flashes + fff
    };
}

#[cfg(test)]
mod tests {
    use crate::days::day_11::print_oct;

    #[test]
    fn test_flash_octo() {
        let mut oct = vec![
            1, 1, 1, 1, 1,
            1, 9, 9, 9, 1,
            1, 9, 1, 9, 1,
            1, 9, 9, 9, 1,
            1, 1, 1, 1, 1,
        ];
        let expected_oct = vec![
            4, 5, 6, 5, 4,
            5, 1, 1, 1, 5,
            6, 1, 1, 1, 6,
            5, 1, 1, 1, 5,
            4, 5, 6, 5, 4,
        ];
        let actual_oct = super::count_flashes(oct, 5, 3);
        print_oct(actual_oct.as_slice(), 5);
        print_oct(expected_oct.as_slice(), 5);
        assert_eq!(actual_oct, expected_oct);
    }

    #[test]
    fn test_flash_octo2() {
        let mut oct = vec![
            1, 1, 1, 1, 1,
            1, 9, 9, 9, 1,
            1, 9, 1, 9, 1,
            1, 9, 9, 9, 1,
            1, 1, 1, 1, 1,
        ];
        let expected_oct = vec![
            3, 4, 5, 4, 3,
            4, 0, 0, 0, 4,
            5, 0, 0, 0, 5,
            4, 0, 0, 0, 4,
            3, 4, 5, 4, 3,
        ];
        let actual_oct = super::count_flashes(oct, 5, 1);
        print_oct(actual_oct.as_slice(), 5);
        print_oct(expected_oct.as_slice(), 5);
        assert_eq!(actual_oct, expected_oct);
    }
}
