use std::path::Path;
use std::time::Instant;

mod day_1_trebuchet;
mod day_2_cube_conundrum;
mod day_3_gear_ratios;
mod day_4_scratchcards;
mod day_5_seed_soil;
mod day_6_wait_for_it;
mod day_7_camel_cards;
mod day_8_haunted_wasteland;
mod day_9_mirage_maintenance;
mod day_10_pipe_maze;
mod day_11_cosmic_expansion;
mod day_12_hot_springs;
mod day_13_point_of_incidence;
mod day_14_parabolic_reflector_dish;
mod day_15_lens_library;
mod day_16_the_floor_will_be_lava;
mod util;


fn main() {
    println!("Advent of Code 2023");

    day_one();
    day_two();
    day_three();
    day_four();
    day_five();
    day_six();
    day_seven();
    day_eight();
    day_nine();
    day_ten();
    day_eleven();
    day_twelve();
    day_thirteen();
    day_fourteen();
    day_fifteen();
    day_sixteen();

}


fn day_one() {
    let day_1_file = Path::new("./data/day_1_trebuchet.txt").to_str().unwrap();
    let timer = Instant::now();
    let cal_value = day_1_trebuchet::determine_calibration_values(day_1_file);

    println!("Day 1: Trebuchet");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Sum of Calibration Values 1: {}", cal_value.0);
    println!("Sum of Calibration Values 2: {}\n", cal_value.1);
    assert_eq!(cal_value.0, 53386);
    assert_eq!(cal_value.1, 53312);
}


fn day_two() {
    let day_2_file = Path::new("./data/day_2_cube_conundrum.txt").to_str().unwrap();
    let timer = Instant::now();
    let (possible_games_sum, cube_power) = day_2_cube_conundrum::determine_games_and_power(day_2_file);

    println!("Day 2: Cube Conundrum");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Possible Games: {possible_games_sum}");
    println!("Cube Power: {cube_power}\n");
    assert_eq!(possible_games_sum, 1853);
    assert_eq!(cube_power, 72706);
}


fn day_three() {
    let day_3_file = Path::new("./data/day_3_gear_ratios.txt").to_str().unwrap();
    let timer = Instant::now();
    let (part_number, gear_ratio) = day_3_gear_ratios::determine_part_number_and_gear_ratio(day_3_file);

    println!("Day 3: Gear Ratios");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Part Number: {part_number}");
    println!("Gear Ratio: {gear_ratio}\n");
    assert_eq!(part_number, 539_433);
    assert_eq!(gear_ratio, 75_847_567);
}


fn day_four() {
    let day_4_file = Path::new("./data/day_4_scratchcards.txt").to_str().unwrap();
    let timer = Instant::now();
    let (points, cards) = day_4_scratchcards::determine_scratchcards_and_points(day_4_file);

    println!("Day 4: Scratchcards");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Scratchcard Points: {points}");
    println!("Scratchcard Total: {cards}\n");
    assert_eq!(points, 21_568);
    assert_eq!(cards, 11_827_296);

}


fn day_five() {
    let day_5_file = Path::new("./data/day_5_seed_soil.txt").to_str().unwrap();
    let timer = Instant::now();
    let closest_location = day_5_seed_soil::determine_closest_location(day_5_file);

    println!("Day 5: Seed-to-Location");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Closest Location 1: {}", closest_location.0);
    println!("Closest Location 2: {}\n", closest_location.1);
    assert_eq!(closest_location.0, 173_706_076);
    assert_eq!(closest_location.1, 11_611_182);
}


fn day_six() {
    let day_6_file = Path::new("./data/day_6_boat_race.txt").to_str().unwrap();
    let timer = Instant::now();
    let ways_to_win = day_6_wait_for_it::determine_winning_races(day_6_file);

    println!("Day 6: Wait for it (Boat Race)");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Ways to Win 1: {}", ways_to_win.0);
    println!("Ways to Win 2: {}\n", ways_to_win.1);
    assert_eq!(ways_to_win.0, 5_133_600);
    assert_eq!(ways_to_win.1, 40_651_271);
}


fn day_seven() {
    let day_7_file = Path::new("./data/day_7_camel_cards.txt").to_str().unwrap();
    let timer = Instant::now();
    let winnings = day_7_camel_cards::determine_total_winnings(day_7_file);

    println!("Day 7: Camel Cards");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Winnings 1: {}", winnings.0);
    println!("Winnings 2: {}\n", winnings.1);
    assert_eq!(winnings.0, 254_024_898);
    assert_eq!(winnings.1, 254_115_617);
}


fn day_eight() {
    let day_8_file = Path::new("./data/day_8_haunted_wasteland.txt").to_str().unwrap();
    let timer = Instant::now();
    let number_of_steps = day_8_haunted_wasteland::determine_steps_through_wasteland(day_8_file);

    println!("Day 8: Haunted Wasteland");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Number of Steps 1: {}", number_of_steps.0);
    println!("Number of Steps 2: {}\n", number_of_steps.1);
    assert_eq!(number_of_steps.0, 12361);
    assert_eq!(number_of_steps.1, 18_215_611_419_223);
}


fn day_nine() {
    let day_9_file = Path::new("./data/day_9_mirage_maintenance.txt").to_str().unwrap();
    let timer = Instant::now();
    let sum_of_values = day_9_mirage_maintenance::determine_sum_of_values(day_9_file);

    println!("Day 9: Mirage Maintenance");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Sum of Values part 1: {}", sum_of_values.0);
    println!("Sum of Values part 2: {}\n", sum_of_values.1);
    assert_eq!(sum_of_values.0, 1_757_008_019);
    assert_eq!(sum_of_values.1, 995);
}


fn day_ten() {
    let day_10_file = Path::new("./data/day_10_pipe_maze.txt").to_str().unwrap();
    let timer = Instant::now();
    let (farthest_distance, enclosed) = day_10_pipe_maze::determine_longest_loop_and_enclosed_tiles(day_10_file);

    println!("Day 10: Pipe Maze");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Farthest Distance: {farthest_distance}");
    println!("Enclosed Tiles: {enclosed}\n");
    assert_eq!(farthest_distance, 6640);
    assert_eq!(enclosed, 411);
}


fn day_eleven() {
    let day_11_file = Path::new("./data/day_11_cosmic_expansion.txt").to_str().unwrap();
    let timer = Instant::now();
    let sum_of_distances = day_11_cosmic_expansion::determine_sum_of_distances(day_11_file);

    println!("Day 11: Cosmic Expansion");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Sum of Distances 1: {}", sum_of_distances.0);
    println!("Sum of Distances 2: {}\n", sum_of_distances.1);
    assert_eq!(sum_of_distances.0, 9_805_264);
    assert_eq!(sum_of_distances.1, 779_032_247_216);
}


fn day_twelve() {
    let day_12_file = Path::new("./data/day_12_hot_springs.txt").to_str().unwrap();
    let timer = Instant::now();
    let total_arrangements = day_12_hot_springs::determine_sum_of_arrangements(day_12_file);

    println!("Day 12: Hot Springs");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Total Arrangements 1: {}", total_arrangements.0);
    println!("Total Arrangements 2: {}\n", total_arrangements.1);
    assert_eq!(total_arrangements.0, 7674);
    assert_eq!(total_arrangements.1, 4_443_895_258_186);
}


fn day_thirteen() {
    let day_13_file = Path::new("./data/day_13_point_of_incidence.txt").to_str().unwrap();
    let timer = Instant::now();
    let sum_of_notes = day_13_point_of_incidence::determine_sum_of_notes(day_13_file);

    println!("Day 13: Point of Incidence");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Sum of Notes: {}", sum_of_notes.0);
    println!("Sum of Notes With Smudge: {}\n", sum_of_notes.1);
    assert_eq!(sum_of_notes.0, 33122);
    assert_eq!(sum_of_notes.1, 32312);
}


fn day_fourteen() {
    let day_14_file = Path::new("./data/day_14_parabolic_reflector_dish.txt").to_str().unwrap();
    let timer = Instant::now();
    let total_load = day_14_parabolic_reflector_dish::determine_total_load(day_14_file);

    println!("Day 14: Parabolic Reflector Dish");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Total Load 1: {}", total_load.0);
    println!("Total Load 2: {}\n", total_load.1);
    assert_eq!(total_load.0, 105461);
    assert_eq!(total_load.1, 102829);
}


fn day_fifteen() {
    let day_15_file = Path::new("./data/day_15_len_library.txt").to_str().unwrap();
    let timer = Instant::now();
    let (hash_sum, focusing_power) = day_15_lens_library::determine_hash(day_15_file);

    println!("Day 15: Lens Library");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Hash Sum: {hash_sum}");
    println!("Focusing Power: {focusing_power}\n");
    assert_eq!(hash_sum, 511343);
    assert_eq!(focusing_power, 294474);
}


fn day_sixteen() {
    let day_16_file = Path::new("./data/day_16_the_floor_will_be_lava.txt").to_str().unwrap();
    // let day_16_file = Path::new("./data/test.txt").to_str().unwrap();
    let timer = Instant::now();

    let energized_tiles = day_16_the_floor_will_be_lava::determine_energized_tiles(day_16_file);

    println!("Day 16: The Floor will be Lava");
    println!("Run Time: {:?}", timer.elapsed());
    println!("Energized Tiles 1: {}", energized_tiles.0);
    println!("Energized Tiles 2: {}", energized_tiles.1);
    assert_eq!(energized_tiles.0, 8125);
    assert_eq!(energized_tiles.1, 8489);
}