use std::fs;

pub fn determine_winning_races(file_path: &str) -> (u32, u32) {
    // Determine the number of ways to win the boat race.
    // part one and two use the same calculation, but different inputs.
    let data = fs::read_to_string(file_path).unwrap();
    let (times, distances) = data.split_once('\n').unwrap();

    let (times, time_part_two) = parse_line(times);
    let (distances, distance_part_two) = parse_line(distances);

    let ways_to_win: u32 = times
        .iter()
        .zip(distances)
        .fold(1, | acc, (time, distance) | acc * calculate_ways_to_win(time, &distance));

    (ways_to_win, calculate_ways_to_win(&time_part_two, &distance_part_two))
}


#[inline]
fn parse_line(line: &str) -> (Vec<f64>, f64) {
    // Parse a line of the input data to extract the numbers
    let values_raw = line
        .split_whitespace()
        .skip(1);

    // Values as vector
    let values: Vec<f64> = values_raw
        .clone()
        .map(| num | num.parse::<f64>().unwrap())
        .collect();

    // Values concatenated
    let value_part_two: f64 = String::from_iter(values_raw).parse::<f64>().unwrap();

    (values, value_part_two)
}


#[inline]
fn calculate_ways_to_win(time: &f64, distance: &f64) -> u32 {
    // Calculate the number of ways to win.
    // The solution boils down to:
    // time_pressed * (time - time_pressed) > distance
    // Solve (using ABC formula):
    // -time^2 + time_pressed * time - distance = 0
    // This results in an interval in which we win

    let discriminant: f64 = (time * time - 4. * distance).sqrt();

    // Use EPSILON to fix boundary condition when start or end is exactly an integer value.
    // We only win if we exceed the distance not when we are exactly equal.
    let interval_start = ((-time + discriminant) / -2. + f64::EPSILON).ceil() as u32;
    let interval_end = ((-time - discriminant) / -2. - f64::EPSILON).floor() as u32;

    interval_end.abs_diff(interval_start) + 1
}