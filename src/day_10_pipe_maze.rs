use std::collections::HashMap;
use std::fs;

use crate::util::text_to_graph;

pub fn determine_longest_loop_and_enclosed_tiles(file_path: &str) -> (usize, usize) {
    let data = fs::read_to_string(file_path).unwrap();
    let graph: HashMap<(isize, isize), u8> = text_to_graph(data);

    let start = *graph
        .iter()
        .find(| ((_x, _y), char) | **char == b'S')
        .unwrap()
        .0;  // item 0 gets the x, y coordinates

    // Go through the graph until we get back to the start
    let mut position = start;
    let mut distance: usize = 0;
    let mut direction = get_start_direction(&graph, start.0, start.1);
    let mut visited: HashMap<(isize, isize), usize> = HashMap::new();  // value is distance from start

    loop {
        visited.insert(position, distance);

        distance += 1;
        match direction {
            b'N' => position.1 -= 1,
            b'E' => position.0 += 1,
            b'S' => position.1 += 1,
            b'W' => position.0 -= 1,
            _ => unreachable!()
        };

        if position == start {
            break  // Made the full loop
        }
        direction = get_new_direction(&graph, &position, direction);
    }

    // Part two just go over the graph again and use the visited map for counting
    let mut enclosed_tiles: usize = 0;
    let ((x_max, y_max), _) = graph.iter().max().unwrap();

    for y in 0..*y_max {
        let mut winding_level = 0;
        for x in 0..*x_max {
            if let (Some(dist_1), Some(dist_2)) = (visited.get(&(x, y)), visited.get(&(x, y + 1))) {
                if *dist_2 == (dist_1 + 1) % distance {
                    winding_level += 1
                } else if *dist_1 == (dist_2 + 1) % distance {
                    winding_level -= 1
                }
            }

            if !visited.contains_key(&(x, y)) && winding_level != 0 {
                enclosed_tiles += 1;
            }
        }
    }

    (distance / 2, enclosed_tiles)
}


fn get_start_direction(
    graph: &HashMap<(isize, isize), u8>, start_x: isize, start_y: isize
) -> u8{
    // Ugly way to determine where we can go from the starting pipe.

    let mut start_direction = b'?';

    if graph
        .get(&(start_x, start_y - 1))
        .is_some_and(| val | *val == b'|' || *val == b'7' || *val == b'F') {

        start_direction = b'N';

    } else if graph
        .get(&(start_x -  1, start_y))
        .is_some_and(| val | *val == b'-' || *val == b'J' || *val == b'7') {

        start_direction = b'E';

    } else if graph
        .get(&(start_x, start_y + 1))
        .is_some_and(| val | *val == b'|' || *val == b'L' || *val == b'J') {

        start_direction = b'S';

    } else if graph
        .get(&(start_x + 1, start_y))
        .is_some_and(| val | *val == b'-' || *val == b'L' || *val == b'F') {

        start_direction = b'W';
    }

    start_direction
}


#[inline]
fn get_new_direction(
    graph: &HashMap<(isize, isize), u8>,
    position: &(isize, isize), previous_direction: u8
) -> u8 {
    // Ugly way to find the next direction based on the previous direction and the pipe piece.
    match (previous_direction, graph.get(position).unwrap()) {
        (b'N', b'|') => b'N',
        (b'N', b'F') => b'E',
        (b'N', b'7') => b'W',
        (b'E', b'-') => b'E',
        (b'E', b'J') => b'N',
        (b'E', b'7') => b'S',
        (b'S', b'|') => b'S',
        (b'S', b'J') => b'W',
        (b'S', b'L') => b'E',
        (b'W', b'-') => b'W',
        (b'W', b'L') => b'N',
        (b'W', b'F') => b'S',
        _ => unreachable!()
    }
}
