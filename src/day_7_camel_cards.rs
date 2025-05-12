use std::collections::HashMap;
use std::fs;

use phf::phf_map;

pub fn determine_total_winnings(file_path: &str) -> (usize, usize) {
    // To determine the total winnings we must do a few things:
    // Score each hand
    // Value each card in the hand (in order)
    // Sort the hands by strength Score -> Value[card[0]] -> Value[card[1]] -> etc.
    // Calculate the winnings = sum( bid * rank )
    let data: String = fs::read_to_string(file_path).unwrap();

    let mut hands_part_one: Vec<CardStrength> = Vec::new();
    let mut hands_part_two: Vec<CardStrength> = Vec::new();

    for line in data.lines() {
        let (hand, bid): (Vec<char>, usize) = match line.split_once(' ') {
            Some((h, b)) => (h.chars().collect(), b.parse::<usize>().unwrap()),
            None => continue
        };

        let (score_part_one, score_part_two) = calculate_card_score(&hand);
        let (hand_values_part_one, hand_values_part_two) = encode_hand(&hand);

        hands_part_one.push(CardStrength::new(score_part_one, hand_values_part_one, bid));
        hands_part_two.push(CardStrength::new(score_part_two, hand_values_part_two, bid));
    }

    hands_part_one.sort();
    hands_part_two.sort();

    let winnings_part_one: usize = hands_part_one
        .into_iter()
        .enumerate()
        .fold(0, | acc, (index, hand) | acc + hand.bid * (index + 1));

    let winnings_part_two: usize = hands_part_two
        .into_iter()
        .enumerate()
        .fold(0, | acc, (index, hand) | acc + hand.bid * (index + 1));

    (winnings_part_one, winnings_part_two)
}


static CARD_CODES_PART_ONE: phf::Map<char, u8> = phf_map! {
    'A' => 12,
    'K' => 11,
    'Q' => 10,
    'J' => 9,
    'T' => 8,
    '9' => 7,
    '8' => 6,
    '7' => 5,
    '6' => 4,
    '5' => 3,
    '4' => 2,
    '3' => 1,
    '2' => 0
};

static CARD_CODES_PART_TWO: phf::Map<char, u8> = phf_map! {
    'A' => 12,
    'K' => 11,
    'Q' => 10,
    'T' => 9,
    '9' => 8,
    '8' => 7,
    '7' => 6,
    '6' => 5,
    '5' => 4,
    '4' => 3,
    '3' => 2,
    '2' => 1,
    'J' => 0
};


#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
struct CardStrength {
    score: u8,
    first: u8,
    second: u8,
    third: u8,
    fourth: u8,
    fifth: u8,
    bid: usize
}

impl CardStrength {
    fn new(score: u8, hand_values: [u8; 5], bid: usize) -> Self{
        CardStrength {
            score,
            first: hand_values[0],
            second: hand_values[1],
            third: hand_values[2],
            fourth: hand_values[3],
            fifth: hand_values[4],
            bid
        }
    }
}


#[inline]
fn calculate_card_score(hand_chars: &Vec<char>) -> (u8, u8) {
    // Calculate the card score by counting each character and then converting that into a score.

    // Create a map with the count of each character
    let mut char_counts: HashMap<char, u8> = HashMap::new();
    for character in hand_chars {
        *char_counts.entry(*character).or_insert(0) += 1;
    }

    // Get the score without any modifications
    let score_part_one = convert_count_to_score(&char_counts);

    // Remove the jokers and add them onto the card with the highest count
    let jokers = match char_counts.remove_entry(&'J') {
        Some((_char, x)) => x,
        None => 0  // There were no jokers in the hand.
    };
    let max_key = match key_of_max_value(&char_counts) {
        Some(key) => *key,
        None => 'J' // Joker was the only card in the hand.
    };
    *char_counts.entry(max_key).or_insert(0) += jokers;

    // Get the score of the "joker modified" map
    let score_part_two = convert_count_to_score(&char_counts);

    (score_part_one, score_part_two)
}


#[inline]
fn convert_count_to_score(char_counts: &HashMap<char, u8>) -> u8 {
    // Convert the counts to the scores listed below
    // "Full house" and "Two pair" require a guard.
    //
    // "Five of a kind" => 6
    // "Four of a kind" => 5
    // "Full house" => 4
    // "Three of a kind" => 3
    // "Two pair" => 2
    // "One pair" => 1
    // "High card" => 0
    match char_counts.values().max() {
        Some(5) => 6,
        Some(4) => 5,
        Some(3) if char_counts.values().any(| x | *x == 2) => 4,
        Some(3) => 3,
        Some(2) if char_counts.values().filter(| x | **x == 2 ).count() == 2 => 2,
        Some(2) => 1,
        _ => 0
    }
}


#[inline]
fn key_of_max_value<K, V>(hash_map: &HashMap<K, V>) -> Option<&K> where V: Ord, {
    // Retrieve the key that is associated with the highest value
    hash_map
        .iter()
        .max_by(|key, val| key.1.cmp(val.1))
        .map(|(key, _val)| key)
}


#[inline]
fn encode_hand(hand_chars: &[char]) -> ([u8; 5], [u8; 5]) {
    // Encode a hand into the value of each card.
    let mut hand_values_part_one: [u8; 5] = [0; 5];
    let mut hand_values_part_two: [u8; 5] = [0; 5];

    for (i, character) in hand_chars.iter().enumerate() {
        hand_values_part_one[i] = *CARD_CODES_PART_ONE.get(character).unwrap();
        hand_values_part_two[i] = *CARD_CODES_PART_TWO.get(character).unwrap();
    }
    (hand_values_part_one, hand_values_part_two)
}