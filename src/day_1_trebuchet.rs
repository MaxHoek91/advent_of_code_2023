use std::fs;

use phf::phf_map;

// Add just enough padding characters to deal with possible overlapping words.
static NUMBER_WORDS: phf::Map<&'static str, &str> = phf_map! {
    "one" => "o1e",
    "two" => "t2o",
    "three" => "t3e",
    "four" => "4",
    "five" => "5e",
    "six" => "6",
    "seven" => "7n",
    "eight" => "e8t",
    "nine" => "n9e"
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


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[&str; 4] = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    const SOLUTION: [u32; 4] = [12, 38, 15, 77];

    #[test]
    fn test_calculate_sum() {
        for (input, expected) in EXAMPLE.iter().zip(SOLUTION) {
            assert_eq!(calculate_sum(input), expected);
        }
    }
}