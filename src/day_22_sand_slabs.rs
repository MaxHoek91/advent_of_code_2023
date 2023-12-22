use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

pub fn determine_safe_bricks(file_path: &str) -> (u16, u32) {
    let data: String = fs::read_to_string(file_path).unwrap();

    let mut bricks: Vec<Vec<u16>>  = data
        .lines()
        .map( | line | line
            .split(&[',', '~'])
            .map(| c| c.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
        )
        .sorted_by_key(| vec | vec[2])
        .collect();

    // Drop bricks to generate the real starting positions
    drop_bricks(&mut bricks, &Vec::new());

    let mut safe_bricks: u16 = 0;
    let mut sum_of_fallen_bricks: u32 = 0;

    for skip_brick in &bricks {
        // Start each attempt with the same brick stack => clone is necessary.
        let fallen_bricks =  drop_bricks(& mut bricks.clone(), skip_brick);

        sum_of_fallen_bricks += fallen_bricks as u32;
        if fallen_bricks == 0 {
            safe_bricks += 1;
        }
    }

    (safe_bricks, sum_of_fallen_bricks)
}


fn drop_bricks(bricks: &mut [Vec<u16>], skip_brick: &Vec<u16>) -> u16 {
    let mut high_points: HashMap<(u16, u16), u16> = HashMap::new();
    let mut fallen_bricks: u16 = 0;

    for brick in bricks.iter_mut() {
        if skip_brick == brick {
            // By continuing the brick is not entered into the high_points map.
            // Therefore it is effectively disintegrated.
            continue
        }

        // Store the z positions because those are mutated later
        let z1 = brick[2];
        let z2  = brick[5];

        // Generate the area of the brick
        let area: Vec<(u16, u16)> = itertools::iproduct!(
            brick[0]..=brick[3], brick[1]..=brick[4]
        ).collect();

        // Determine the current highest_point in the area
        let highest_point: u16 = area
            .iter()
            .map(| a | high_points.get(a).unwrap_or(&0))
            .max()
            .unwrap() + 1;

        // Put the brick on top of the highest point.
        area
            .iter()
            .for_each(| a | {
                high_points.insert(*a, highest_point + z2 - z1);
            });

        // Check if the brick has fallen.
        if z1 > highest_point {
            fallen_bricks += 1;

            // Update the z-positions if it did fall.
            brick[2] = highest_point;
            brick[5] = highest_point + z2 - z1;
        }
    }

    fallen_bricks
}