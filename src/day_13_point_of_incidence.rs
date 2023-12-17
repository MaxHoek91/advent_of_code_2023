use std::fs;


pub fn determine_sum_of_notes(file_path: &str) -> (usize, usize) {
    // Determine the sum of notes by finding the palindrome in each pattern.
    let data  = fs::read_to_string(file_path)
        .unwrap()
        .replace('.', "0")
        .replace('#', "1");

    let mut sum_of_notes: usize = 0;
    let mut sum_of_notes_with_smudge: usize = 0;

    for pattern in data.split("\n\n") {
        let (line, column) = encode_pattern(pattern);

        // For part 2 we want to find a new solution when using a smudge.
        // To achieve this we provide the previous start_end so that we can exclude that solution.
        let (line_count, start_end) = find_palindrome(&line, (0, 0), 0);
        let (line_smudge, _) = find_palindrome(&line, start_end, 1);

        let (col_count, start_end) = find_palindrome(&column, (0, 0), 0);
        let (col_smudge, _) = find_palindrome(&column, start_end, 1);

        sum_of_notes += line_count * 100 + col_count;
        sum_of_notes_with_smudge += line_smudge * 100 + col_smudge;
    }
    (sum_of_notes, sum_of_notes_with_smudge)
}


#[inline]
fn encode_pattern(pattern: &str) -> (Vec<u32>, Vec<u32>) {
    // Turn the pattern of 0s and 1s into u32 bitfields.
    // For generating the columns, some awkward loops are used
    // These push the characters to the correct column strings.

    // First newline indicates the length of a row -> number of columns.
    let mut columns: Vec<String> = vec![String::new(); pattern.find('\n').unwrap()];

    let mut line_code: Vec<u32> = Vec::new();
    for line in pattern.lines() {

        for (i, char) in line.chars().enumerate() {
            columns[i].push(char);
        }

        line_code.push(u32::from_str_radix(line, 2).unwrap());
    }

    let column_code: Vec<u32> = columns
        .iter()
        .map(| str | u32::from_str_radix(str, 2).unwrap())
        .collect();

    (line_code, column_code)
}


#[inline]
fn find_palindrome(pattern: &Vec<u32>, previous: (usize, usize), smudges: u32) -> (usize, (usize, usize)) {
    // Loop over all indices and try to expand a palindrome from there.
    // We stop at the first answer that satisfies all conditions.
    for index in 0..pattern.len() {
        let (start, end) = expand_from_center(pattern, index, index + 1, smudges);

        // Pattern needs to touch either end
        // And be larger than 0
        // For part 2 when working with a smudge we also want a different solution that previous.
        if (start == 0 || end == (pattern.len() - 1))
            && end > 0
            && (start != previous.0 || end != previous.1) {
            return ((start + end + 2) / 2, (start, end));  // +2 because counting starts at 1
        }
    }
    (0, (0, 0))
}


#[inline]
fn expand_from_center(
    pattern: &[u32], left_start: usize, right_start: usize, total_smudges: u32
) -> (usize, usize) {
    // Start the palindrome from left/right_start and expand in both directions while valid.

    let (mut start, mut end): (usize, usize) = (0, 0);
    let (mut left, mut right, mut smudge) = (left_start, right_start, total_smudges);

    while right < pattern.len() {

        // XOR of patterns would be all 0 for the equal patterns
        // Mismatches could be fixed if we have "smudges" remaining.
        let mismatches: u32 = (pattern[left] ^ pattern[right]).count_ones();
        if mismatches > smudge {
            break;
        } else {
            smudge -= mismatches;
        }

        start = left;
        end = right;

        if left == 0 { break; }

        left -= 1;
        right += 1;
    }
    (start, end)
}
