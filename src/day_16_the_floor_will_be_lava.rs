use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use rayon::prelude::*;

use crate::util::text_to_graph;

pub fn determine_energized_tiles(file_path: &str) -> (usize, usize) {
    let data = fs::read_to_string(file_path).unwrap();
    let graph: HashMap<(isize, isize), u8> = text_to_graph(data);

    let energized_tiles_part_one: usize = breadth_first_search(&graph, ((-1, 0), b'E'));

    let mut energized_tiles_part_two: [usize; 4] = [0; 4];
    let (x_max, y_max) = *graph.keys().max().unwrap();

    energized_tiles_part_two[0] = (0..y_max)
        .into_par_iter()
        .map(| y | breadth_first_search(&graph, ((-1, y), b'E')))
        .max()
        .unwrap();

    energized_tiles_part_two[1] = (0..y_max)
        .into_par_iter()
        .map(| y | breadth_first_search(&graph, ((x_max + 1, y), b'W')))
        .max()
        .unwrap();

    energized_tiles_part_two[2] = (0..x_max)
        .into_par_iter()
        .map(| x | breadth_first_search(&graph, ((x, -1), b'S')))
        .max()
        .unwrap();

    energized_tiles_part_two[3] = (0..x_max)
        .into_par_iter()
        .map(| x | breadth_first_search(&graph, ((x, y_max + 1), b'N')))
        .max()
        .unwrap();

    (energized_tiles_part_one, *energized_tiles_part_two.iter().max().unwrap())
}


fn breadth_first_search(graph: &HashMap<(isize, isize), u8>, start: ((isize, isize), u8)) -> usize {
    let (x_max, y_max) = *graph.keys().max().unwrap();

    let mut queue: VecDeque<((isize, isize), u8)> = VecDeque::from([start]);
    let mut visited: HashSet<((isize, isize), u8)> = HashSet::new();
    let mut energized: HashSet<(isize, isize)> = HashSet::new();

    while !queue.is_empty() {
        let (mut position, direction) = match queue.pop_front() {
            Some((p, d)) => (p, d),
            None => break  // No more steps to take.
        };

        if !visited.insert((position, direction)) {
            continue;  // We got back to a path that we already visited.
        }
        energized.insert(position);

        match direction {
            b'N' if position.1 > 0 => position.1 -= 1,
            b'E' if position.0 < x_max => position.0 += 1,
            b'S' if position.1 < y_max => position.1 += 1,
            b'W' if position.0 > 0 => position.0 -= 1,
            _ => continue // New position would be outside of the board.
        }

        match (direction, graph.get(&position).unwrap()) {
            (b'N', b'/') => queue.push_back((position, b'E')),
            (b'N', b'\\') => queue.push_back((position, b'W')),
            (b'N', b'-') => { queue.push_back((position, b'W')); queue.push_back((position, b'E')); },
            (b'N', _) => queue.push_back((position, b'N')),
            (b'E', b'/') => queue.push_back((position, b'N')),
            (b'E', b'\\') => queue.push_back((position, b'S')),
            (b'E', b'|') => { queue.push_back((position, b'N')); queue.push_back((position, b'S')); },
            (b'E', _) => queue.push_back((position, b'E')),
            (b'S', b'/') => queue.push_back((position, b'W')),
            (b'S', b'\\') => queue.push_back((position, b'E')),
            (b'S', b'-') => { queue.push_back((position, b'W')); queue.push_back((position, b'E')); },
            (b'S', _) => queue.push_back((position, b'S')),
            (b'W', b'/') => queue.push_back((position, b'S')),
            (b'W', b'\\') => queue.push_back((position, b'N')),
            (b'W', b'|') => { queue.push_back((position, b'N')); queue.push_back((position, b'S')); },
            (b'W', _) => queue.push_back((position, b'W')),
            _ => unreachable!()
        };
    }
    energized.len() - 1
}