use std::fs;

use phf::phf_map;

static NUMBER_WORDS: phf::Map<&'static str, &str> = phf_map! {
    "one" => "one1one",
    "two" => "two2two",
    "three" => "three3three",
    "four" => "four4four",
    "five" => "five5five",
    "six" => "six6six",
    "seven" => "seven7seven",
    "eight" => "eight8eight",
    "nine" => "nine9nine"
};

pub fn determine_calibration_values(file_path: &str) -> (u32, u32) {
    let mut data: String = fs::read_to_string(file_path).unwrap();

    // Part 1 before changing the words to numbers
    let part_one = calculate_sum(&data);

    // Part 2 change words to numbers and calculate again
    for (word, num) in NUMBER_WORDS.into_iter() {
        data = data.replace(word, num);
    }
    let part_two = calculate_sum(&data);

    (part_one, part_two)
}


fn calculate_sum(data: &str) -> u32 {
    // Calculate the sum of all numbers.
    // Where the number are the first and last digits in each row.
    let mut number_sum: u32 = 0;

    for line in data.lines() {
        let first_digit: char =  match line.find(char::is_numeric) {
            Some(index) => line.chars().nth(index).unwrap(),
            None => continue  // No numeric chars in the entire line.
        };

        let last_digit: char = match line.rfind(char::is_numeric) {
            Some(index) => line.chars().nth(index).unwrap(),
            None => first_digit  // Only 1 digit in the line.
        };

        let mut number: String = String::from(first_digit);
        number.push(last_digit);
        number_sum += number.parse::<u32>().unwrap();
    }
    number_sum
}