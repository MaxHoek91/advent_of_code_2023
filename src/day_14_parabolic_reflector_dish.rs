use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::string::ParseError;

pub fn determine_total_load(file_path: &str) -> (usize, usize) {
    let data: String = fs::read_to_string(file_path)
        .unwrap()
        .replace('#', "0")
        .replace('O', "1")
        .replace('.', "2");

    // Part One
    let mut dish: ReflectorDish = ReflectorDish::from_str(&data).unwrap();
    dish.rotate_anti_clockwise();
    dish.tilt();
    let total_load_part_one = dish.get_beam_load();

    // Part Two
    // To find the cycle keep a history of dishes
    // Once we find a repeating cycle in our history, align with the target number and take the beam load.
    let mut dish: ReflectorDish = ReflectorDish::from_str(&data).unwrap();
    let mut total_load_part_two: usize = 0;
    let mut dish_history: HashMap<ReflectorDish, usize> = HashMap::new();

    dish.rotate_anti_clockwise(); // Prime the board because the beam load is defined from top to bottom.
    for i in 0..1_000_000_000 {
        if let Some(previous_i) = dish_history.insert(dish.clone(), i) {
            if (1_000_000_000 - i) % (i - previous_i) == 0 {
                total_load_part_two = dish.get_beam_load();
                break;
            }
        }
        dish.spin_cycle();
    }

    (total_load_part_one, total_load_part_two)
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ReflectorDish {
    dish: Vec<Vec<u8>>
}

impl ReflectorDish {
    #[inline]
    fn rotate_clockwise(&mut self) {
        // Rotate clockwise 90 degrees.
        let len: usize = self.dish[0].len();
        let mut iters: Vec<_> = self.dish.iter().map(|n| n.iter()).collect();

        let vec: Vec<Vec<u8>> = (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(| item| *item.next().unwrap())
                    .rev()
                    .collect::<Vec<u8>>()
            })
            .collect();

        self.dish = vec;
    }

    #[inline]
    fn rotate_anti_clockwise(&mut self) {
        let len: usize = self.dish[0].len();
        let mut iters: Vec<_> = self.dish.iter().map(|n| n.iter()).collect();

        let mut vec: Vec<Vec<u8>> = (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(| item| *item.next().unwrap())
                    .collect::<Vec<u8>>()
            })
            .collect();
        vec.reverse();
        self.dish = vec;
    }

    #[inline]
    fn tilt(&mut self) {
        let indices: Vec<Vec<usize>> = self.find_indices(0);

        for (line, idx) in self.dish.iter_mut().zip(indices) {
            for win in idx.windows(2) {
                line[win[0]..win[1]].sort();
            }
        }
    }

    #[inline]
    fn spin_cycle(&mut self) {
        // Perform a spin cycle.
        self.tilt();
        self.rotate_clockwise();
        self.tilt();
        self.rotate_clockwise();
        self.tilt();
        self.rotate_clockwise();
        self.tilt();
        self.rotate_clockwise();
    }

    #[inline]
    fn get_beam_load(&mut self) -> usize {
        let scoring: Vec<usize> = (0..=self.dish.len()).rev().collect();

        self.dish
            .iter()
            .map(| vec | { vec
                .iter()
                .enumerate()
                .filter(|(_, &val)| val == 1)
                .map(|(index, _)| scoring[index])
                .sum::<usize>()
            })
            .sum()
    }

    #[inline]
    fn find_indices(&self, find: u8) -> Vec<Vec<usize>> {
        // Find the indices where vec == find.
        // Put in between start and end so they may be used as ranges for sorting.
        let mut indices: Vec<Vec<usize>> = Vec::new();

        for vec in self.dish.iter() {
            let mut idx = vec![0];
            idx.extend(vec
                    .iter()
                    .enumerate()
                    .filter(| (_, &val) | val == find)
                    .map(| (index, _ ) | index)
            );
            idx.push(vec.len());
            indices.push(idx);
        }
        indices
    }
}

impl FromStr for ReflectorDish {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dish: Vec<Vec<u8>> = Vec::new();
        for line in s.lines() {
            dish.push(
                line
                    .chars()
                    .map(| char | char.to_digit(10).unwrap() as u8)
                    .collect()
            );
        }
        Ok(Self { dish })
    }
}


