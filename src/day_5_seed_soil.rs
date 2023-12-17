use std::fs;

struct MapTraversal {
    groups: Vec<MapGroup>
}

impl MapTraversal {
    fn new(groups: Vec<MapGroup>) -> Self {
        Self { groups }
    }

    fn find_location(&self, seed: usize) -> usize {
        self.groups.iter().fold(seed, | map, group | group.find_range(map))
    }

    fn find_source(&self, destination: usize, group_idx: usize) -> usize {
        let mut value = destination;
        for map in self.groups.iter().take(group_idx).rev() {
            value = map.reverse_find(value);
        }
        value
    }

    fn find_from_specified_level(&self, source: usize, group_idx: usize) -> usize {
        self.groups[group_idx..]
            .iter()
            .fold(source, | map, group | group.find_range(map))
    }

    fn levels(&self) -> usize {
        self.groups.len()
    }

    fn boundaries(&self, group_idx: usize) -> impl Iterator<Item = usize> + '_ {
        self.groups[group_idx].boundaries()
    }
}

#[derive(Debug)]
struct MapGroup {
    maps: Vec<MapItem>
}

impl MapGroup {
    fn new(mut items: Vec<MapItem>) -> Self {
        items.sort_by_key(| item | item.source);
        Self { maps: items }
    }

    fn find_range(&self, source: usize) -> usize {
        match self.maps.iter().find(| item | item.contains_source(source)) {
            Some(item) => item.map(source),
            None => source
        }
    }

    fn reverse_find(&self, destination: usize) -> usize {
        match self.maps.iter().find(| item | item.contains_destination(destination)) {
            Some(item) => item.reverse_map(destination),
            None => destination
        }
    }

    fn boundaries(&self) -> impl Iterator<Item = usize> + '_ {
        std::iter::once(0)
            .chain(
                self.maps
                    .iter()
                    .flat_map(| item | [item.source, item.source + item.length])
            )
            .chain(std::iter::once(usize::MAX))
    }
}

#[derive(Debug, Clone)]
struct MapItem {
    destination: usize,
    source: usize,
    length: usize
}

impl MapItem {
    #[inline]
    fn contains_source(&self, source: usize) -> bool {
        // Checks if the source value is within the range of this item.
        self.source <= source && source < self.source + self.length
    }

    #[inline]
    fn contains_destination(&self, destination: usize) -> bool {
        // Checks if the destination value is within the range of this item.
        self.destination <= destination && destination < self.destination + self.length
    }

    #[inline]
    fn map(&self, source: usize) -> usize {
        // Map a source value to a destination.
        self.destination + source - self.source
    }

    #[inline]
    fn reverse_map(&self, destination: usize) -> usize {
        // Map  a destination to a source.
        self.source + destination - self.destination
    }
}

impl From<&str> for MapItem {
    fn from(value: &str) -> Self {
        let items: Vec<usize> = value
            .split_whitespace()
            .map(| num | num.parse::<usize>().unwrap())
            .collect();

        Self { destination: items[0], source: items[1], length: items[2] }
    }
}


pub fn determine_closest_location(file_path: &str) -> (usize, usize) {
    let data = fs::read_to_string(file_path).unwrap();
    let mut groups = data.split("\n\n");

    // Parse the list of seeds
    let seeds: Vec<usize> = groups
        .next()
        .unwrap()
        [7..]  // remove "seeds: "
        .split_whitespace()
        .map(| num | num.parse::<usize>().unwrap())
        .collect();

    // Create the seed ranges for part 2
    let seed_ranges: Vec<std::ops::Range<usize>> = seeds
        .windows(2)
        .step_by(2)
        .map(| x | x[0]..(x[0] + x[1]))
        .collect();

    // Create the MapTraversal object that contains the groups and items used to traverse the map.
    let mut map_traversal: Vec<MapGroup> = Vec::new();
    for group in groups {
        let mut map_group: Vec<MapItem> = Vec::new();

        for line in group.lines().skip(1) {
            map_group.push(MapItem::from(line));
        }
        map_traversal.push(MapGroup::new(map_group));
    }
    let map_traversal = MapTraversal::new(map_traversal);

    // Traverse through the map to find the closest location.
    let closest_location_part_one: usize = seeds
        .into_iter()
        .map(| seed | map_traversal.find_location(seed))
        .min()
        .unwrap();

    // Traverse the map using every range start
    // Then traverse along the boundary conditions of the ranges
    // Retain only the lowest value.
    let mut closest_location_part_two: usize = seed_ranges
        .iter()
        .map(| range | range.start)
        .map(| seed | map_traversal.find_location(seed))
        .min()
        .unwrap();

    for level in 0..map_traversal.levels() {
        let alternate_route = map_traversal
            .boundaries(level)
            .filter(| &value | {
                let source = map_traversal.find_source(value, level);
                seed_ranges.iter().any(| range | range.contains(&source))
            })
            .map(| start | map_traversal.find_from_specified_level(start, level))
            .min()
            .unwrap();

        closest_location_part_two = closest_location_part_two.min(alternate_route)
    }

    (closest_location_part_one, closest_location_part_two)
}