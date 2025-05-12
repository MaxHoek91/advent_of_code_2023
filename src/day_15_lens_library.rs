use std::fs;

use indexmap::IndexMap;


pub fn determine_hash(file_path: &str) -> (usize, usize) {
    let data = fs::read_to_string(file_path)
        .unwrap()
        .replace('\n', "");

    // Part 1
    let hash_sum: usize = data
        .split(',')
        .map(| section | section
            .chars()
            .fold(0, hash_character)
        ).sum();

    // Part 2
    // Put everything in boxes
    let mut boxes: Vec<IndexMap<&str, usize>> = vec![IndexMap::new(); 256];
    for section in data.split(',') {
        let (label, focal_length) = section.split_once(['=', '-']).unwrap();
        let box_idx = label.chars().fold(0, hash_character);

        match section.contains('=') {
            true => boxes[box_idx].insert(label, focal_length.parse::<usize>().unwrap()),
            false => boxes[box_idx].shift_remove(label)
        };
    }

    // Determine the focusing power
    let focusing_power: usize = boxes
        .iter()
        .enumerate()
        .fold(0, | acc, (box_idx, slots) |
            acc + (box_idx + 1) * slots
            .values()
            .enumerate()
            .fold(0, | acc, (slot_idx, focal_length) |
                acc + (slot_idx + 1) * focal_length
            )
        );

    (hash_sum, focusing_power)
}


#[inline]
fn hash_character(current_value: usize, character: char) -> usize {
    (current_value + character as usize) * 17 % 256
}
