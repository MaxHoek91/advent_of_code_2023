use std::fs;

const  MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;


pub fn determine_games_and_power(file_path: &str) -> (u32, u32) {
    // Calculate the sum of game numbers for valid games
    // A game is valid if all draws have all cubes below the max count
    let data: String = fs::read_to_string(file_path).unwrap();

    let mut valid_game_sum: u32 = 0;
    let mut cube_power_sum: u32 = 0;

    for row in data.lines() {
        let (game, draws) = row.split_once(':').unwrap();

        if draws.split(';').all(valid_draw) {
            valid_game_sum += game[5..].parse::<u32>().unwrap();  // game = "Game xx" so 5.. for slice
        }

        cube_power_sum += cube_power(draws);
    }

    (valid_game_sum, cube_power_sum)
}


#[inline]
fn valid_draw(draw: &str) -> bool {
    // Determine if a draw is valid. Return immediately upon finding an invalid draw.
    for cubes in draw.split(',') {
        let (count, color) = cubes
            .trim()
            .split_once(' ')
            .unwrap();

        let count: u8 = count.parse::<u8>().unwrap();

        match color {
            "red" if count > MAX_RED => return false,
            "green" if count > MAX_GREEN => return false,
            "blue" if count > MAX_BLUE => return false,
            _ => continue
        }
    }
    true
}


#[inline]
fn cube_power(game: &str) -> u32 {
    // Determine the cube power.
    let mut min_red: u32 = 0;
    let mut min_green: u32 = 0;
    let mut min_blue: u32 = 0;

    for draw in game.split(';') {
        for cubes in draw.split(',') {
            let (count, color) = cubes.trim().split_once(' ').unwrap();
            let count: u32 = count.parse::<u32>().unwrap();

            match color {
                "red" if count > min_red => min_red = count,
                "green" if count > min_green => min_green = count,
                "blue" if count > min_blue => min_blue = count,
                _ => continue
            }
        }
    }
    min_red * min_green * min_blue
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[&str; 5] = &[
        "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    ];
    const POSSIBLE_GAMES: [bool; 5] = [true, true, false, false, true];
    const CUBE_POWER: [u32; 5] = [48, 12, 1560, 630, 36];

    #[test]
    fn test_valid_draw() {
        for (input, expected) in EXAMPLE.iter().zip(POSSIBLE_GAMES) {
            assert_eq!(input.split(';').all(valid_draw), expected);
        }
    }

    #[test]
    fn test_cube_power() {
        for (input, expected) in EXAMPLE.iter().zip(CUBE_POWER) {
            assert_eq!(cube_power(input), expected);
        }
    }
}