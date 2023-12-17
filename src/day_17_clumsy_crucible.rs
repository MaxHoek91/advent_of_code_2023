use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeMap};
use std::fs;

pub fn determine_path_of_minimal_heat_loss(file_path: &str) -> (isize, isize) {
    let data: String = fs::read_to_string(file_path).unwrap();

    let mut graph: BTreeMap<(isize, isize), isize> = BTreeMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            graph.insert(
                (x as isize, y as isize), character.to_digit(10).unwrap() as isize
            );
        }
    }

    let finish: (isize, isize) = *graph.keys().max().unwrap();
    (
        dijkstra(&graph, (0, 0), finish, 1, 3),
        dijkstra(&graph, (0, 0), finish, 4, 10)
    )
}


fn dijkstra(
    graph: &BTreeMap<(isize, isize), isize>,
    start: (isize, isize),
    finish: (isize, isize),
    min_step: isize,
    max_step: isize
) -> isize {
    // Dijkstra's algorithm to find path of minimum heat loss.

    let mut route = BTreeMap::new();
    let mut queue = BinaryHeap::new();

    // Reverse BinaryHeap acts as Min-heap
    queue.push(Reverse((0, start, (0, 0))));

    while let Some(Reverse((heat_loss, current, prev_dir))) = queue.pop() {
        if current == finish {
            return heat_loss;
        }

        if route.get(&(current, prev_dir)).is_some_and(| &loss | heat_loss > loss) {
            continue;  // We already have a better route to this point
        }

        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if prev_dir == direction || prev_dir == (-direction.0, -direction.1) {
                // Not allowed to go in the same direction again.
                // No point to go back to where we came from.
                continue;
            }

            let mut next_heat_loss: isize = heat_loss;
            for distance in 1..=max_step {
                // TODO find cleaner method of adding all the heat losses.
                //      currently we just take this part of the loop until distance < min_step

                let next = (
                    current.0 + direction.0 * distance,
                    current.1 + direction.1 * distance
                );

                next_heat_loss += match graph.get(&next) {
                    Some(loss) => loss,
                    None => continue  // next step moved outside graph bounds.
                };

                if distance < min_step {
                    continue;  // Need to take more steps in the same direction.
                }

                if route.get(&(next, direction)).unwrap_or(&isize::MAX) > &next_heat_loss {
                    // Insert if new route or improved compared to previous.
                    queue.push(Reverse((next_heat_loss, next, direction)));
                    route.insert((next, direction), next_heat_loss);
                }
            }
        }
    }
    unreachable!()  // We should find the finish before the queue is empty.
}