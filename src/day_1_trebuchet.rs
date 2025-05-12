use std::fs;

use bstr::ByteSlice;
use phf::phf_map;

const NUMERIC: [u8; 10] = *b"0123456789";

// Add just enough padding characters to deal with possible overlapping words.
static NUMBER_WORDS: phf::Map<&[u8], &[u8]> = phf_map! {
    b"one" => b"o1e",
    b"two" => b"t2o",
    b"three" => b"th3ee",
    b"four" => b"fo4r",
    b"five" => b"fi5e",
    b"six" => b"s6x",
    b"seven" => b"se7en",
    b"eight" => b"ei8ht",
    b"nine" => b"n9ne"
};

pub fn determine_calibration_values(file_path: &str) -> (u32, u32) {
    let mut data: Vec<u8> = fs::read(file_path).unwrap();

    // Part 1 before changing the words to numbers
    let part_one = calculate_sum(&data);

    // Part 2 change words to numbers and calculate again
    NUMBER_WORDS
        .into_iter()
        .for_each(| (word, num) | data = data.replace(word, num));

    let part_two = calculate_sum(&data);

    (part_one, part_two)
}


/// Calculate the sum of all numbers.
/// Where the number are the first and last digits in each row.
fn calculate_sum(data: &[u8]) -> u32 {
    let mut number_sum: u32 = 0;

    for line in data.lines() {
        let first_digit: u8 =  match line.find_byteset(NUMERIC) {
            Some(index) => line[index],
            None => continue  // No numeric chars in the entire line.
        };

        let last_digit: u8 = match line.rfind_byteset(NUMERIC) {
            Some(index) => line[index],
            None => first_digit  // Only 1 digit in the line.
        };

        let number: u8 = (first_digit - 48) * 10 + (last_digit - 48);
        number_sum += number as u32;
    }
    number_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [&[u8]; 4] = [b"1abc2", b"pqr3stu8vwx", b"a1b2c3d4e5f", b"treb7uchet"];
    const SOLUTION: [u32; 4] = [12, 38, 15, 77];

    #[test]
    fn test_calculate_sum() {
        for (input, expected) in EXAMPLE.iter().zip(SOLUTION) {
            assert_eq!(calculate_sum(input), expected);
        }
    }
}
