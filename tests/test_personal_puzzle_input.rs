use std::path::Path;

use advent_of_code_2023::*;

#[test]
fn test_day_1() {
    let file = Path::new("./data/day_1_trebuchet.txt").to_str().unwrap();
    let cal_value = day_1_trebuchet::determine_calibration_values(file);

    assert_eq!(cal_value.0, 53386);
    assert_eq!(cal_value.1, 53312);
}

#[test]
fn test_day_2() {
    let file = Path::new("./data/day_2_cube_conundrum.txt").to_str().unwrap();
    let (possible_games_sum, cube_power) = day_2_cube_conundrum::determine_games_and_power(file);

    assert_eq!(possible_games_sum, 1853);
    assert_eq!(cube_power, 72706);
}

#[test]
fn test_day_3() {
    let file = Path::new("./data/day_3_gear_ratios.txt").to_str().unwrap();
    let (part_number, gear_ratio) = day_3_gear_ratios::determine_part_number_and_gear_ratio(file);

    assert_eq!(part_number, 539_433);
    assert_eq!(gear_ratio, 75_847_567);
}

#[test]
fn test_day_4() {
    let file = Path::new("./data/day_4_scratchcards.txt").to_str().unwrap();
    let (points, cards) = day_4_scratchcards::determine_scratchcards_and_points(file);

    assert_eq!(points, 21_568);
    assert_eq!(cards, 11_827_296);
}

#[test]
fn test_day_5() {
    let file = Path::new("./data/day_5_seed_to_location.txt").to_str().unwrap();
    let closest_location = day_5_seed_soil::determine_closest_location(file);

    assert_eq!(closest_location.0, 173_706_076);
    assert_eq!(closest_location.1, 11_611_182);
}

#[test]
fn test_day_6() {
    let file = Path::new("./data/day_6_boat_race.txt").to_str().unwrap();
    let ways_to_win = day_6_wait_for_it::determine_winning_races(file);

    assert_eq!(ways_to_win.0, 5_133_600);
    assert_eq!(ways_to_win.1, 40_651_271);
}

#[test]
fn test_day_7() {
    let file = Path::new("./data/day_7_camel_cards.txt").to_str().unwrap();
    let winnings = day_7_camel_cards::determine_total_winnings(file);

    assert_eq!(winnings.0, 254_024_898);
    assert_eq!(winnings.1, 254_115_617);
}

#[test]
fn test_day_8() {
    let file = Path::new("./data/day_8_haunted_wasteland.txt").to_str().unwrap();
    let number_of_steps = day_8_haunted_wasteland::determine_steps_through_wasteland(file);

    assert_eq!(number_of_steps.0, 12361);
    assert_eq!(number_of_steps.1, 18_215_611_419_223);
}

#[test]
fn test_day_9() {
    let file = Path::new("./data/day_9_mirage_maintenance.txt").to_str().unwrap();
    let sum_of_values = day_9_mirage_maintenance::determine_sum_of_values(file);

    assert_eq!(sum_of_values.0, 1_757_008_019);
    assert_eq!(sum_of_values.1, 995);
}

#[test]
fn test_day_10() {
    let file = Path::new("./data/day_10_pipe_maze.txt").to_str().unwrap();
    let (farthest_distance, enclosed) = day_10_pipe_maze::determine_longest_loop_and_enclosed_tiles(file);

    assert_eq!(farthest_distance, 6640);
    assert_eq!(enclosed, 411);
}

#[test]
fn test_day_11() {
    let file = Path::new("./data/day_11_cosmic_expansion.txt").to_str().unwrap();
    let sum_of_distances = day_11_cosmic_expansion::determine_sum_of_distances(file);

    assert_eq!(sum_of_distances.0, 9_805_264);
    assert_eq!(sum_of_distances.1, 779_032_247_216);
}

#[test]
fn test_day_12() {
    let file = Path::new("./data/day_12_hot_springs.txt").to_str().unwrap();
    let total_arrangements = day_12_hot_springs::determine_sum_of_arrangements(file);

    assert_eq!(total_arrangements.0, 7674);
    assert_eq!(total_arrangements.1, 4_443_895_258_186);
}


#[test]
fn test_day_13() {
    let file = Path::new("./data/day_13_point_of_incidence.txt").to_str().unwrap();
    let sum_of_notes = day_13_point_of_incidence::determine_sum_of_notes(file);

    assert_eq!(sum_of_notes.0, 33122);
    assert_eq!(sum_of_notes.1, 32312);
}

#[test]
fn test_day_14() {
    let file = Path::new("./data/day_14_parabolic_reflector_dish.txt").to_str().unwrap();
    let total_load = day_14_parabolic_reflector_dish::determine_total_load(file);

    assert_eq!(total_load.0, 105461);
    assert_eq!(total_load.1, 102829);
}

#[test]
fn test_day_15() {
    let file = Path::new("./data/day_15_lens_library.txt").to_str().unwrap();
    let (hash_sum, focusing_power) = day_15_lens_library::determine_hash(file);

    assert_eq!(hash_sum, 511343);
    assert_eq!(focusing_power, 294474);
}

#[test]
fn test_day_16() {
    let file = Path::new("./data/day_16_the_floor_will_be_lava.txt").to_str().unwrap();
    let energized_tiles = day_16_the_floor_will_be_lava::determine_energized_tiles(file);

    assert_eq!(energized_tiles.0, 8125);
    assert_eq!(energized_tiles.1, 8489);
}

#[test]
fn test_day_17() {
    let file = Path::new("./data/day_17_clumsy_crucible.txt").to_str().unwrap();
    let heat_loss = day_17_clumsy_crucible::determine_path_of_minimal_heat_loss(file);
    assert_eq!(heat_loss.0, 686);
    assert_eq!(heat_loss.1, 801);
}

#[test]
fn test_day_18() {
    let file = Path::new("./data/day_18_lavaduct_lagoon.txt").to_str().unwrap();
    let lagoon_volume =  day_18_lavaduct_lagoon::determine_lava_volume(file);

    assert_eq!(lagoon_volume.0, 52231);
    assert_eq!(lagoon_volume.1, 57196493937398);
}

#[test]
fn test_day_19() {
    let file = Path::new("./data/day_19_aplenty.txt").to_str().unwrap();
    let (rating_numbers, distinct_combinations) =  day_19_aplenty::determine_rating_numbers(file);

    assert_eq!(rating_numbers, 350678);
    assert_eq!(distinct_combinations, 124831893423809);
}

#[test]
fn test_day_20() {
    let file = Path::new("./data/day_20_pulse_propagation.txt").to_str().unwrap();
    let (pulse_product, fewest_presses) =  day_20_pulse_propagation::determine_pulse_product(file);

    assert_eq!(pulse_product, 806332748);
    assert_eq!(fewest_presses, 228060006554227);
}

#[test]
fn test_day_21() {
    let file = Path::new("./data/day_21_step_counter.txt").to_str().unwrap();
    let plots_visited =  day_21_step_counter::determine_visited_garden_plots(file);

    assert_eq!(plots_visited.0, 3503);
    assert_eq!(plots_visited.1, 584211423220706);
}

#[test]
fn test_day_22() {
    let file = Path::new("./data/day_22_sand_slabs.txt").to_str().unwrap();
    let (safe_bricks, fallen_bricks) =  day_22_sand_slabs::determine_safe_bricks(file);

    assert_eq!(safe_bricks, 471);
    assert_eq!(fallen_bricks, 68525);
}
