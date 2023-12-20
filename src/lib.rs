use std::path::Path;
use std::time::Instant;

pub mod day_1_trebuchet;
pub mod day_2_cube_conundrum;
pub mod day_3_gear_ratios;
pub mod day_4_scratchcards;
pub mod day_5_seed_soil;
pub mod day_6_wait_for_it;
pub mod day_7_camel_cards;
pub mod day_8_haunted_wasteland;
pub mod day_9_mirage_maintenance;
pub mod day_10_pipe_maze;
pub mod day_11_cosmic_expansion;
pub mod day_12_hot_springs;
pub mod day_13_point_of_incidence;
pub mod day_14_parabolic_reflector_dish;
pub mod day_15_lens_library;
pub mod day_16_the_floor_will_be_lava;
pub mod day_17_clumsy_crucible;
pub mod day_18_lavaduct_lagoon;
pub mod day_19_aplenty;
pub mod day_20_pulse_propagation;

mod util;


pub fn solve_all() {
    println!("Advent of Code 2023\n");

    day_1();
    day_2();
    day_3();
    day_4();
    day_5();
    day_6();
    day_7();
    day_8();
    day_9();
    day_10();
    day_11();
    day_12();
    day_13();
    day_14();
    day_15();
    day_16();
    day_17();
    day_18();
    day_19();
    day_20();
}

fn day_1() {
    let file = Path::new("./data/day_1_trebuchet.txt").to_str().unwrap();
    let timer = Instant::now();
    let cal_value = day_1_trebuchet::determine_calibration_values(file);

    println!(
        "Day 1: Trebuchet\n\
        Run Time: {:?}\n\
        Sum of Calibration Values 1: {}\n\
        Sum of Calibration Values 2: {}\n",
        timer.elapsed(), cal_value.0, cal_value.1
    );
}

fn day_2() {
    let file = Path::new("./data/day_2_cube_conundrum.txt").to_str().unwrap();
    let timer = Instant::now();
    let (possible_games_sum, cube_power) = day_2_cube_conundrum::determine_games_and_power(file);

    println!(
        "Day 2: Cube Conundrum\n\
        Run Time: {:?}\n\
        Possible Games: {}\n\
        Cube Power: {}\n",
        timer.elapsed(), possible_games_sum, cube_power
    );
}

fn day_3() {
    let file = Path::new("./data/day_3_gear_ratios.txt").to_str().unwrap();
    let timer = Instant::now();
    let (part_number, gear_ratio) = day_3_gear_ratios::determine_part_number_and_gear_ratio(file);

    println!(
        "Day 3: Gear Ratios\n\
        Run Time: {:?}\n\
        Part Number: {}\n\
        Gear Ratio: {}\n",
        timer.elapsed(), part_number, gear_ratio
    );
}

fn day_4() {
    let file = Path::new("./data/day_4_scratchcards.txt").to_str().unwrap();
    let timer = Instant::now();
    let (points, cards) = day_4_scratchcards::determine_scratchcards_and_points(file);

    println!(
        "Day 4: Scratchcards\n\
        Run Time: {:?}\n\
        Scratchcard Points: {}\n\
        Scratchcard Total: {}\n",
        timer.elapsed(), points, cards
    );
}

fn day_5() {
    let file = Path::new("./data/day_5_seed_to_location.txt").to_str().unwrap();
    let timer = Instant::now();
    let closest_location = day_5_seed_soil::determine_closest_location(file);

    println!(
        "Day 5: Seed-to-Location\n\
        Run Time: {:?}\n\
        Closest Location 1: {}\n\
        Closest Location 2: {}\n",
        timer.elapsed(), closest_location.0, closest_location.1
    );
}

fn day_6() {
    let file = Path::new("./data/day_6_boat_race.txt").to_str().unwrap();
    let timer = Instant::now();
    let ways_to_win = day_6_wait_for_it::determine_winning_races(file);

    println!(
        "Day 6: Wait for it (Boat Race)\n\
        Run Time: {:?}\n\
        Ways to Win 1: {}\n\
        Ways to Win 2: {}\n",
        timer.elapsed(), ways_to_win.0, ways_to_win.1
    );
}

fn day_7() {
    let file = Path::new("./data/day_7_camel_cards.txt").to_str().unwrap();
    let timer = Instant::now();
    let winnings = day_7_camel_cards::determine_total_winnings(file);

    println!(
        "Day 7: Camel Cards\n\
        Run Time: {:?}\n\
        Winnings 1: {}\n\
        Winnings 2: {}\n",
        timer.elapsed(), winnings.0, winnings.1
    );
}

fn day_8() {
    let file = Path::new("./data/day_8_haunted_wasteland.txt").to_str().unwrap();
    let timer = Instant::now();
    let number_of_steps = day_8_haunted_wasteland::determine_steps_through_wasteland(file);

    println!(
        "Day 8: Haunted Wasteland\n\
        Run Time: {:?}\n\
        Number of Steps 1: {}\n\
        Number of Steps 2: {}\n",
        timer.elapsed(), number_of_steps.0, number_of_steps.1
    );
}

fn day_9() {
    let file = Path::new("./data/day_9_mirage_maintenance.txt").to_str().unwrap();
    let timer = Instant::now();
    let sum_of_values = day_9_mirage_maintenance::determine_sum_of_values(file);

    println!(
        "Day 9: Mirage Maintenance\n\
        Run Time: {:?}\n\
        Sum of Values 1: {}\n\
        Sum of Values 2: {}\n",
        timer.elapsed(), sum_of_values.0, sum_of_values.1
    );
}

fn day_10() {
    let file = Path::new("./data/day_10_pipe_maze.txt").to_str().unwrap();
    let timer = Instant::now();
    let (farthest_distance, enclosed) = day_10_pipe_maze::determine_longest_loop_and_enclosed_tiles(file);

    println!(
        "Day 10: Pipe Maze\n\
        Run Time: {:?}\n\
        Farthest Distance: {}\n\
        Enclosed Tiles: {}\n",
        timer.elapsed(), farthest_distance, enclosed
    );
}

fn day_11() {
    let file = Path::new("./data/day_11_cosmic_expansion.txt").to_str().unwrap();
    let timer = Instant::now();
    let sum_of_distances = day_11_cosmic_expansion::determine_sum_of_distances(file);

    println!(
        "Day 11: Cosmic Expansion\n\
        Run Time: {:?}\n\
        Sum of Distances 1: {}\n\
        Sum of Distances 2: {}\n",
        timer.elapsed(), sum_of_distances.0, sum_of_distances.1
    );
}

fn day_12() {
    let file = Path::new("./data/day_12_hot_springs.txt").to_str().unwrap();
    let timer = Instant::now();
    let total_arrangements = day_12_hot_springs::determine_sum_of_arrangements(file);

    println!(
        "Day 12: Hot Springs\n\
        Run Time: {:?}\n\
        Total Arrangements 1: {}\n\
        Total Arrangements 2: {}\n",
        timer.elapsed(), total_arrangements.0, total_arrangements.1
    );
}

fn day_13() {
    let file = Path::new("./data/day_13_point_of_incidence.txt").to_str().unwrap();
    let timer = Instant::now();
    let sum_of_notes = day_13_point_of_incidence::determine_sum_of_notes(file);

    println!(
        "Day 13: Point of Incidence\n\
        Run Time: {:?}\n\
        Sum of Notes: {}\n\
        Sum of Notes with Smudge: {}\n",
        timer.elapsed(), sum_of_notes.0, sum_of_notes.1
    );
}

fn day_14() {
    let file = Path::new("./data/day_14_parabolic_reflector_dish.txt").to_str().unwrap();
    let timer = Instant::now();
    let total_load = day_14_parabolic_reflector_dish::determine_total_load(file);

    println!(
        "Day 14: Parabolic Reflector Dish\n\
        Run Time: {:?}\n\
        Total Load 1: {}\n\
        Total Load 2: {}\n",
        timer.elapsed(), total_load.0, total_load.1
    );
}

fn day_15() {
    let file = Path::new("./data/day_15_lens_library.txt").to_str().unwrap();
    let timer = Instant::now();
    let (hash_sum, focusing_power) = day_15_lens_library::determine_hash(file);

    println!(
        "Day 15: Lens Library\n\
        Run Time: {:?}\n\
        Hash Sum: {}\n\
        Focusing Power: {}\n",
        timer.elapsed(), hash_sum, focusing_power
    );
}

fn day_16() {
    let file = Path::new("./data/day_16_the_floor_will_be_lava.txt").to_str().unwrap();
    let timer = Instant::now();
    let energized_tiles = day_16_the_floor_will_be_lava::determine_energized_tiles(file);

    println!(
        "Day 16: The Floor will be Lava\n\
        Run Time: {:?}\n\
        Energized Tiles 1: {}\n\
        Energized Tiles 2: {}\n",
        timer.elapsed(), energized_tiles.0, energized_tiles.1
    );
}

fn day_17() {
    let file = Path::new("./data/day_17_clumsy_crucible.txt").to_str().unwrap();
    let timer = Instant::now();
    let heat_loss = day_17_clumsy_crucible::determine_path_of_minimal_heat_loss(file);

    println!(
        "Day 17: Clumsy Crucible\n\
        Run Time: {:?}\n\
        Path of Minimum Heat Loss 1: {}\n\
        Path of Minimum Heat Loss 2: {}\n",
        timer.elapsed(), heat_loss.0, heat_loss.1
    );
}

fn day_18() {
    let file = Path::new("./data/day_18_lavaduct_lagoon.txt").to_str().unwrap();
    let timer = Instant::now();
    let lagoon_volume =  day_18_lavaduct_lagoon::determine_lava_volume(file);

    println!(
        "Day 18: Lavaduct Lagoon\n\
        Run Time: {:?}\n\
        Lagoon Volume 1: {}\n\
        Lagoon Volume 2: {}\n",
        timer.elapsed(), lagoon_volume.0, lagoon_volume.1
    );
}

fn day_19() {
    let file = Path::new("./data/day_19_aplenty.txt").to_str().unwrap();
    let timer = Instant::now();
    let (rating_numbers, distinct_combinations) =  day_19_aplenty::determine_rating_numbers(file);

    println!(
        "Day 19: Aplenty\n\
        Run Time: {:?}\n\
        Rating Numbers of Accepted Parts: {}\n\
        Distinct Combinations of Ratings: {}\n",
        timer.elapsed(), rating_numbers, distinct_combinations
    );
}

fn day_20() {
    let file = Path::new("./data/day_20_pulse_propagation.txt").to_str().unwrap();
    // let file = Path::new("./data/test.txt").to_str().unwrap();
    let timer = Instant::now();
    let (pulse_product, fewest_presses) =  day_20_pulse_propagation::determine_pulse_product(file);

    println!(
        "Day 20: Pulse Propagation\n\
        Run Time: {:?}\n\
        Pulse Product: {}\n\
        Fewest Presses to reach RX: {}\n",
        timer.elapsed(), pulse_product, fewest_presses
    );
}