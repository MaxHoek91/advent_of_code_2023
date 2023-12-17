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