use std::fs;

use regex::Regex;

pub fn determine_part_number_and_gear_ratio(file_path: &str) -> (u32, u32) {
    // For each number check if it is adjacent a punctuation character
    // If true add to the total.
    let data: String = fs::read_to_string(file_path)
        .unwrap()
        .replace('.', " ");  // Remove periods so that all punctuation can be matched.

    let mut data = data.lines().peekable();
    let re: Regex = Regex::new(r"\d+").unwrap();

    let mut number_total: u32 = 0;
    let mut gear_ratio: u32 = 0;
    let mut previous_line: &str = "";

    while let Some(current_line) = data.next() {

        let next_line: &str = match data.peek() {
            Some(line) => line,
            None => ""  // Next line is beyond end of file
        };

        for number in re.find_iter(current_line) {
            if next_to_punctuation(current_line, number.start(), number.end())
                || search_line_for_punctuation(previous_line, number.start(), number.end())
                || search_line_for_punctuation(next_line, number.start(), number.end()) {

                number_total += number.as_str().parse::<u32>().unwrap();
            }
        }

        for (index, _gear_char) in current_line.match_indices('*') {
            gear_ratio += calculate_ratio(previous_line, current_line, next_line, &re, index);
        }

        previous_line = current_line;
    }
    (number_total, gear_ratio)
}


#[inline]
fn next_to_punctuation(line: &str, start: usize, end: usize) -> bool {
    // Check in the line if the character before start or after end is punctuation
    let before = match line.chars().nth(start.saturating_sub(1)) {
        Some(character) => character.is_ascii_punctuation(),
        None => false
    };

    let after = match line.chars().nth(end) {
        Some(character) => character.is_ascii_punctuation(),
        None => false
    };

    before || after
}


#[inline]
fn search_line_for_punctuation(line: &str, start: usize, end: usize) -> bool {
    // Search in the line for punctuation characters
    // For the matched characters check if they are between start-end
    for (index, _punctuation) in line.match_indices(|c: char| c.is_ascii_punctuation()) {
        if start.saturating_sub(1) <= index && index <= end {
            return true;
        }
    }
    false
}


#[inline]
fn calculate_ratio(
    previous_line: &str,
    current_line: &str,
    next_line: &str,
    regex: &Regex,
    index: usize
) -> u32 {
    // For a gear at "index" calculate the ratio of the surrounding numbers
    let mut adjacent_numbers: Vec<u32> = Vec::new();
    find_adjacent(&mut adjacent_numbers, previous_line, regex, index);
    find_adjacent(&mut adjacent_numbers, current_line, regex, index);
    find_adjacent(&mut adjacent_numbers, next_line, regex, index);


    if adjacent_numbers.len() > 1 {
        adjacent_numbers.iter().product()
    } else {
        0
    }
}


#[inline]
fn find_adjacent(adjacent_numbers: &mut Vec<u32>, line: &str, regex: &Regex, gear_index: usize){
    for number in regex.find_iter(line) {
        if number.start().saturating_sub(1) <= gear_index && gear_index <= number.end() {
            adjacent_numbers.push(number.as_str().parse::<u32>().unwrap());
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_ratio() {
        let re: Regex = Regex::new(r"\d+").unwrap();

        let gear_ratio = calculate_ratio(
            "467..114..",
            "...*......",
            "..35..633.",
            &re, 3);
        assert_eq!(gear_ratio, 16345);

        let gear_ratio = calculate_ratio(
            "......#...",
            "617*......",
            ".....+.58.",
            &re, 3);
        assert_eq!(gear_ratio, 0);

        let gear_ratio = calculate_ratio(
            "......755.",
            "...$.*....",
            ".664.598..",
            &re, 5);
        assert_eq!(gear_ratio, 451490);
    }
}