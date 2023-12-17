use std::fs;

use cached::{Cached, UnboundCache};


pub fn determine_sum_of_arrangements(file_path: &str) -> (usize, usize) {
    let data  = fs::read_to_string(file_path).unwrap();

    let mut total_arrangements_part_one: usize = 0;
    let mut total_arrangements_part_two: usize = 0;

    for line in data.lines() {
        let (records, groups) = line.split_once(' ').unwrap();

        let groups: Vec<usize> = groups
            .split(',')
            .map(| c | c.parse::<usize>().unwrap())
            .collect();

        let mut cache: UnboundCache<(usize, usize, usize), usize> = UnboundCache::new();
        
        total_arrangements_part_one += arrangements(
            &mut cache, records.as_bytes(), None, &groups
        );
        
        total_arrangements_part_two += arrangements(
            &mut cache, [records].repeat(5).join("?").as_bytes(), None, &groups.repeat(5)
        );
    }

    (total_arrangements_part_one, total_arrangements_part_two)
}


fn arrangements(
    cache: &mut UnboundCache<(usize, usize, usize), usize>,
    records: &[u8], within_a_group: Option<usize>, groups: &[usize]
) -> usize {
    if records.is_empty() {  // Base-case we went over all records.
        return match (groups.len(), within_a_group) {
            // At the end, no groups left => Solution found.
            (0, None) => 1,

            // At the end, last group fits => Solution found.
            (1, Some(group)) if group == groups[0] => 1,

            // At the end but unplaced or non-fitting group => No solution.
            _ => 0,
        };
    }

    // In a group but no groups left  => No solution.
    if within_a_group.is_some() && groups.is_empty() {
        return 0;
    }

    let cache_key = (records.len(), within_a_group.unwrap_or(0), groups.len());
    if let Some(&count) = cache.cache_get(&cache_key) {
        return count;
    }

    // Check the first character and decide what to do next.
    let count: usize = match(records[0], within_a_group) {
        // Not in a group and the current character is not a group => Just continue.
        (b'.', None) => arrangements(cache, &records[1..], None, groups),

        // The next group can't be placed => No solution.
        (b'.', Some(group)) if group != groups[0] => 0,

        // Next group fits => Continue to next group (_group == group[0] if we did not match previous clause).
        (b'.', Some(_group)) => arrangements(cache, &records[1..], None, &groups[1..]),

        // Currently not in a group => Start a new group and continue.
        (b'#', None) => arrangements(cache, &records[1..], Some(1), groups),

        // Still in a group => Increment within_a_group and continue (Some(x).map(| x | x + 1) == Some(x + 1)).
        (b'#', Some(_)) => arrangements(cache, &records[1..], within_a_group.map(|x | x + 1), groups),

        // Still in a group, but it can't be placed yet => Increment and continue.
        (b'?', Some(group)) if group != groups[0] => {
            arrangements(cache, &records[1..], within_a_group.map(|x| x + 1), groups)
        }

        // Next group fits => Continue to next group (because of previous claus).
        (b'?', Some(_)) => arrangements(cache, &records[1..], None, &groups[1..]),

        // We are maybe in a group => Consider both possibilities.
        (b'?', None) => {
            arrangements(cache, &records[1..], Some(1), groups) + arrangements(cache, &records[1..], None, groups)
        },

        _ => unreachable!(),
    };

    cache.cache_set(cache_key, count);
    count
}