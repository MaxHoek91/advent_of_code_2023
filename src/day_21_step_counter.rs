use std::collections::{HashMap, VecDeque};
use std::fs;

use crate::util::text_to_graph;

type Graph = HashMap<(isize, isize), u8>;


pub fn determine_visited_garden_plots(file_path: &str) -> (usize, usize) {
    let data: String = fs::read_to_string(file_path).unwrap();

    let mut graph: Graph = text_to_graph(data);
    let start = *graph
        .iter()
        .find(| ((_x, _y), char) | **char == b'S')
        .unwrap()
        .0;  // item 0 gets the x, y coordinates

    // Part One
    let visited = breadth_first_search(&graph, start, 64);
    let reachable_plots_part_one = visited
        .values()
        .filter(| distance | *distance % 2 == 0)
        .count();

    // Part Two
    let (x_max, y_max) = *graph.keys().max().unwrap();
    assert_eq!(x_max, y_max);
    let size = x_max + 1;  // +1 because the graph is zero-indexed.

    // Expand the graph by factor 5
    for ((x, y), value) in graph.clone().iter() {
        for (a, b) in itertools::iproduct!(-5..=5, -5..=5) {
            graph.insert((x + a * size, y + b * size), *value);
        }
    }

    // Find reachable for each max_distance
    let mut reachable: Vec<usize> = Vec::new();
    for i in 0..3 {
        let max_distance: u32 = 65 + i * size as u32;  // 65 is distance to first edge.

        reachable.push(
            breadth_first_search(&graph, start, max_distance)
                .values()
                .filter(| distance | *distance % 2 == max_distance % 2)
                .count()
        );
    }

    let target = (26501365 - 65) / size as usize;
    // Quadratic fit from WolframAlpha
    // TODO implement quadratic fit in Rust.
    let reachable_plots_part_two: usize = 3606 + 14377 * target + 14275 * target * target;

    (reachable_plots_part_one, reachable_plots_part_two)
}

fn breadth_first_search(
    graph: &Graph, start: (isize, isize), max_distance: u32
) -> HashMap<(isize, isize), u32> {
    let mut queue: VecDeque<((isize, isize), u32)> = VecDeque::from([(start, 0)]);
    let mut visited: HashMap<(isize, isize), u32> = HashMap::new();

    while let Some((position, distance)) = queue.pop_front() {
        if distance > max_distance {
            continue;
        }

        match visited.get(&position) {
            Some(_) => continue,
            None => visited.insert(position, distance)
        };

        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {

            let next = (position.0 + direction.0, position.1 + direction.1);
            match graph.get_key_value(&next) {
                Some((coord, tile)) if *tile != b'#' => {
                    queue.push_back((*coord, distance + 1));
                },
                Some(_) => continue,  // Rock
                None => unreachable!(),  // Guard because we should never reach an edge.
            };
        }
    }

    visited
}
