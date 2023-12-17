use std::collections::HashMap;
use std::fs;

pub fn determine_steps_through_wasteland(file_path: &str) -> (usize, usize) {
    let data: String = fs::read_to_string(file_path).unwrap();
    let mut lines = data.lines();

    // Get the loop of steps and convert to indices
    let step_indices: Vec<usize> = lines
        .next()
        .unwrap()
        .replace('L', "0")
        .replace('R', "1")
        .chars()
        .map(| c: char | c.to_digit(10).unwrap() as usize)  // indices must be usize
        .collect();

    // Construct the map
    let mut map: HashMap<&str, [&str; 2]> = HashMap::new();
    for line in lines.skip(1) {
        let (key, val) = line.split_once('=').unwrap();
        let (val_1, val_2) = val.split_once(',').unwrap();

        map.insert(
            key.trim(),
            [
                val_1.strip_prefix(" (").unwrap(),
                val_2.trim().strip_suffix(')').unwrap()
            ]
        );
    }

    // Traverse through the map part one
    let mut current_position = "AAA";
    let mut n_steps_part_one: usize = 0;
    let mut indices = step_indices.iter().cycle();
    while current_position != "ZZZ" {
        current_position = map.get(current_position).unwrap()[*indices.next().unwrap()];
        n_steps_part_one += 1;
    }

    // Traverse through the map part two
    // The trick is to do each route individually and then compute the LCM for the solution.
    let mut current_positions: Vec<&str> = map
        .clone()
        .into_keys()
        .filter(| key | key.ends_with('A'))
        .collect();
    let mut n_steps_part_two: Vec<usize> = Vec::new();

    for pos in current_positions.iter_mut() {
        let mut indices = step_indices.iter().cycle();
        let mut n_steps: usize = 0;

        while !pos.ends_with('Z') {
            *pos = map.get(pos).unwrap()[*indices.next().unwrap()];
            n_steps += 1
        }
        n_steps_part_two.push(n_steps);
    }

    (n_steps_part_one, lcm(&n_steps_part_two))
}


fn lcm(nums: &[usize]) -> usize {
    // Compute Least Common Multiple of the values using recursion.
    if nums.len() == 1 { return nums[0]; } // Base Case

    let a = nums[0];
    let b = lcm(&nums[1..]);

    a * b / gcd_of_two_numbers(a, b)
}


#[inline]
fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    // Compute the greatest common divisor of two numbers with recursion.
    if b == 0 { return a; } // Base Case

    gcd_of_two_numbers(b, a % b)
}