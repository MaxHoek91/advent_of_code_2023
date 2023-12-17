use std::collections::HashSet;
use std::fs;

pub fn determine_scratchcards_and_points(file_path: &str) -> (u32, u32) {
    let data: String = fs::read_to_string(file_path).unwrap();

    let mut total_points: u32 = 0;
    let mut card_count: Vec<u32> = vec![1; data.lines().count()];

    for (row, line) in data.lines().enumerate() {
        let (winning_numbers, our_numbers) = line.split_once('|').unwrap();
        let (_game, winning_numbers) = winning_numbers.split_once(':').unwrap();

        let winning_numbers: HashSet<u32> = string_to_num_set(winning_numbers);
        let our_numbers: HashSet<u32> = string_to_num_set(our_numbers);

        let wins: usize = our_numbers.intersection(&winning_numbers).count();

        if wins > 0 {
            total_points += 1 << (wins - 1);
        }

        for i in 1..wins + 1 {
            card_count[row + i] += card_count[row]
        }
    }
    (total_points, card_count.into_iter().sum())
}


#[inline]
fn string_to_num_set(line: &str) -> HashSet<u32> {
    // Parse a line of numbers (separated by whitespace) to a Hashset of the values.
    HashSet::from_iter(
        line
            .split_whitespace()
            .map(| num | num.trim().parse::<u32>().unwrap())
    )
}