use std::fs;

pub fn determine_lava_volume(file_path: &str) -> (isize, isize) {
    let data: String = fs::read_to_string(file_path).unwrap();

    let (vertices, circumference) = generate_vertices_part_one(&data);
    let area_part_one = shoelace_area(vertices) + circumference / 2 + 1;

    let (vertices, circumference) = generate_vertices_part_two(&data);
    let area_part_two = shoelace_area(vertices) + circumference / 2 + 1;

    (area_part_one, area_part_two)
}


fn generate_vertices_part_one(data: &str) -> (Vec<(isize, isize)>, isize) {
    let mut vertices:  Vec<(isize, isize)> = Vec::new();
    let mut circumference = 0;

    let mut previous_vertex = (0, 0);
    vertices.push(previous_vertex);

    for line in data.lines() {
        let line_data: Vec<&str> = line.split_whitespace().collect();

        let direction = match line_data[0] {
            "U" => (0, 1),
            "R" => (1, 0),
            "D" => (0, -1),
            "L" => (-1, 0),
            _ => unreachable!()
        };

        let distance: isize = line_data[1].parse().unwrap();
        circumference += distance;

        let next_vertex = (
            previous_vertex.0 + direction.0 * distance,
            previous_vertex.1 + direction.1 * distance
        );

        vertices.push(next_vertex);
        previous_vertex = next_vertex;
    }

    (vertices, circumference)
}


fn generate_vertices_part_two(data: &str) -> (Vec<(isize, isize)>, isize) {
    let mut vertices:  Vec<(isize, isize)> = Vec::new();
    let mut circumference = 0;

    let mut previous_vertex = (0, 0);
    vertices.push(previous_vertex);

    for line in data.lines() {
        let (_, hex) = line.split_once('#').unwrap();

        let direction = match hex.as_bytes()[5] {
            b'3' => (0, 1),  // Up
            b'0' => (1, 0),  // Right
            b'1' => (0, -1), // Down
            b'2' => (-1, 0), // Left
            _ => unreachable!()
        };

        let distance = isize::from_str_radix(hex.get(0..5).unwrap(), 16).unwrap();
        circumference += distance;

        let next_vertex = (
            previous_vertex.0 + direction.0 * distance,
            previous_vertex.1 + direction.1 * distance
        );

        vertices.push(next_vertex);
        previous_vertex = next_vertex;
    }

    (vertices, circumference)
}


fn shoelace_area(vertices: Vec<(isize, isize)>) -> isize {
    // Calculate the shoelace are held within the vertices.
    let mut total = 0;

    for idx in 0..(vertices.len() - 1) {
        total +=
            vertices[idx].0 * vertices[idx + 1].1
            - vertices[idx].1 * vertices[idx + 1].0;
    }

    // NOTE: This version assumes that vertices.first() == vertices.last()
    // If that is not the case, uncomment these two lines.
    // total += vertices[vertices.len() - 1].0 * vertices[0].1
    //     - vertices[vertices.len() - 1].1 * vertices[0].0;

    total.abs() / 2
}