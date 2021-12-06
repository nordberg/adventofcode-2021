pub fn num_of_window_increases(sonar_measurements: &[i32]) -> i32 {
    sonar_measurements
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|window| if window[1] > window[0] { 1 } else { 0 })
        .sum()
}