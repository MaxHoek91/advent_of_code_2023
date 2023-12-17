use std::collections::HashSet;
use std::fs;

const EXPANSION_PART_ONE: usize = 1;
const EXPANSION_PART_TWO: usize = 999_999;


pub fn determine_sum_of_distances(file_path: &str) -> (usize, usize) {
    let data  = fs::read_to_string(file_path).unwrap();

    // Parse the data and extract the galaxies
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (y, line) in data.lines().enumerate() {
        galaxies.extend(line
            .match_indices('#')
            .map(| item | (item.0, y))
        );
    }

    // Unzip X and Y coordinates of galaxies, determine X/Y max
    let (gal_x, gal_y): (Vec<usize>, Vec<usize>) = galaxies.iter().cloned().unzip();
    let expansion_x: Vec<usize> = generate_expansion_vec(gal_x.clone());
    let expansion_y: Vec<usize> = generate_expansion_vec(gal_y.clone());

    // Expand the galaxies and calculate the distance
    let gal_x_expanded: Vec<usize> = expand_galaxy(gal_x.clone(), &expansion_x, EXPANSION_PART_ONE);
    let gal_y_expanded: Vec<usize> = expand_galaxy(gal_y.clone(), &expansion_y, EXPANSION_PART_ONE);
    let distance_part_one: usize = manhattan_distance(gal_x_expanded, gal_y_expanded);

    let gal_x_expanded: Vec<usize> = expand_galaxy(gal_x, &expansion_x, EXPANSION_PART_TWO);
    let gal_y_expanded: Vec<usize> = expand_galaxy(gal_y, &expansion_y, EXPANSION_PART_TWO);
    let distance_part_two: usize = manhattan_distance(gal_x_expanded, gal_y_expanded);

    (distance_part_one, distance_part_two)
}


fn generate_expansion_vec(occupied_indices: Vec<usize>) -> Vec<usize> {
    // Generate the vector for expanding the galaxy based on where the empty indices are.
    let max_index = *occupied_indices.iter().max().unwrap();

    let empty_indices: HashSet<usize> = (0..max_index)
        .collect::<HashSet<usize>>()
        .difference(&HashSet::from_iter(occupied_indices))
        .cloned()
        .collect();

    let mut expansion_vector: Vec<usize> = vec![0; max_index + 1];

    for e_x in empty_indices.iter() {
        for i in *e_x..max_index {
            expansion_vector[i + 1] += 1;
        }
    }

    expansion_vector
}


fn expand_galaxy(mut galaxies: Vec<usize>, expansion: &[usize],  expansion_coefficient: usize) -> Vec<usize>{
    // Expand the galaxy by the expansion coefficient.
    for gal in galaxies.iter_mut() {
        *gal += expansion[*gal] * expansion_coefficient;
    }
    galaxies
}


fn manhattan_distance(x_values: Vec<usize>, y_values: Vec<usize>) -> usize {
    // Efficient approach to determine the manhattan distance between all combinations of points.
    manhattan_distance_single_axis(x_values) + manhattan_distance_single_axis(y_values)
}


#[inline]
fn manhattan_distance_single_axis(mut values: Vec<usize>) -> usize {
    values.sort();

    let mut combined_distance: usize = 0;
    let mut cumulative_sum: usize = 0;
    for (i, item) in values.iter().enumerate() {
        combined_distance += item * i - cumulative_sum;
        cumulative_sum += item;
    }

    combined_distance
}