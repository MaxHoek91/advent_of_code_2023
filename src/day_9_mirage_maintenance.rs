use std::fs;

use itertools::Itertools;

pub fn determine_sum_of_values(file_path: &str) -> (isize, isize) {
    let data = fs::read_to_string(file_path).unwrap();

    let mut prediction_sum_part_one: isize = 0;
    let mut prediction_sum_part_two: isize = 0;

    for line in data.lines() {
        let mut values: Vec<isize> = line
            .split_whitespace()
            .map(| num | num.parse::<isize>().unwrap())
            .collect();

        prediction_sum_part_one += predict_number(values.clone());

        values.reverse();
        prediction_sum_part_two += predict_number(values);
    }

    (prediction_sum_part_one, prediction_sum_part_two)
}


#[inline]
fn predict_number(mut values: Vec<isize>) -> isize {
    // Predict the next value.
    //
    // Perform subtraction on windows
    // Add last value
    // Continue until all values are equal (because for next step they would all be zero)
    let mut prediction: isize = *values.last().unwrap();

    while !values.iter().all_equal() {
        values = values
            .windows(2)
            .map(| num | num[1] - num[0])
            .collect();

        prediction += values.last().unwrap();
    }
    prediction
}