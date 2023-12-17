use std::collections::HashMap;

/// Convert the data input to a graph representation.
/// The graph consists of keys (x, y) and the character value as bytes.
pub fn text_to_graph(data: String) -> HashMap<(isize, isize), u8> {
    let mut graph: HashMap<(isize, isize), u8> = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, character) in line.as_bytes().iter().enumerate() {
            graph.insert((x as isize, y as isize), *character);
        }
    }
    graph
}