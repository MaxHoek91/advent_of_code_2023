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


pub fn lcm(nums: &[usize]) -> usize {
    // Compute Least Common Multiple of the values using recursion.
    if nums.len() == 1 { return nums[0]; } // Base Case

    let a = nums[0];
    let b = lcm(&nums[1..]);

    a * b / gcd_of_two_numbers(a, b)
}


#[inline]
fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    // Compute the greatest common divisor of two numbers with recursion.
    if b == 0 { return a; } // Base Case

    gcd_of_two_numbers(b, a % b)
}

