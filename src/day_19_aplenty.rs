use std::collections::HashMap;
use std::fs;

type Workflows<'a> = HashMap<&'a str, Instructions<'a>>;
type Rating = HashMap<u8, u32>;
type PossibleRatings = [(u32, u32); 4];


pub fn determine_rating_numbers(file_path: &str) -> (u32, u64) {
    let data: String  = fs::read_to_string(file_path).unwrap();

    let (workflows, ratings) = data.split_once("\n\n").unwrap();

    let workflows = parse_workflows(workflows);
    let ratings = parse_ratings(ratings);

    let mut ratings_part_one = 0;
    for rating in ratings.iter() {
        ratings_part_one += trace_outcome(rating, &workflows);
    }

    (ratings_part_one, distinct_combinations_of_ratings(&workflows))
}


fn parse_workflows(data: &str) -> HashMap<&str, Instructions> {
    let mut workflows: HashMap<&str, Instructions> = HashMap::new();

    for line in data.lines() {
        let (location, instructions) = line.split_once('{').unwrap();
        let instructions = parse_instructions(instructions);
        workflows.insert(location, instructions);
    }
    workflows
}

#[derive(Debug)]
struct Instructions<'a> {
    rules: Vec<Rule<'a>>,
    else_: &'a str
}

impl<'a> Instructions<'a> {

    fn apply(&'a self, rating: &HashMap<u8, u32>) -> &'a str {
        for rule in self.rules.iter() {
            if rule.operation.cmp(rating[&rule.category], rule.value) {
                return rule.target;
            }
        }
        self.else_
    }

    fn find_combinations(
        &'a self,
        mut ratings: [(u32, u32); 4],
        workflows: &Workflows,
        possible_ratings: &mut Vec<PossibleRatings>
    ) {
        let mut resolve = | target, rating | match target {
            "R" => {},
            "A" => possible_ratings.push(rating),
            target => workflows.get(target).unwrap().find_combinations(rating, workflows, possible_ratings)
        };

        for Rule { category, operation, value, target} in &self.rules {
            let x = ratings[category_to_index(category)];
            let y = match operation.find_combinations(x, *value) {
                None => continue,
                Some(y) => y
            };

            resolve(*target, update(ratings.clone(), *category, y));
            ratings[category_to_index(category)] = range_compare(x, y);
        }

        resolve(self.else_, ratings);
    }
}

fn parse_instructions(instructions: &str) -> Instructions {
    // FIXME changed from method to function because I lifetimes are hard.

    let mut instructions = instructions
        .trim_matches('}')
        .split(',')
        .rev();

    let mut rules: Vec<Rule> = Vec::new();
    let else_ = instructions.next().unwrap();

    for instruction in instructions.rev() {
        let (lhs, target) = instruction.split_once(':').unwrap();

        let value = lhs
            .get(2..)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let lhs = lhs.as_bytes();
        let category = lhs[0];
        let operation = Op::from_byte(lhs[1]);

        rules.push(Rule{ category, operation, value, target });

    };

    Instructions { rules, else_ }
}


#[derive(Debug)]
struct Rule<'a> {
    category: u8,
    operation: Op,
    value: u32,
    target: &'a str
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Gt,
    Lt
}

impl Op {

    fn from_byte(char: u8) -> Self {
        match char {
            b'>' => Op::Gt,
            b'<' => Op::Lt,
            _ => unreachable!()
        }
    }

    fn cmp(self, input: u32, other: u32) -> bool {
        match self {
            Op::Gt => input > other,
            Op::Lt => input < other
        }
    }

    fn find_combinations(self, input: (u32, u32), other: u32) -> Option<(u32, u32)> {
        match self {
            Op::Gt if input.1 <= other => None,
            Op::Gt => Some((input.0.max(other + 1), input.1)),
            Op::Lt if input.0 >= other => None,
            Op::Lt => Some((input.0, input.1.min(other - 1)))
        }
    }
}

fn parse_ratings(data: &str) -> Vec<Rating> {
    let mut ratings: Vec<_> = Vec::new();

    for line in data.lines() {
        let mut rating: HashMap<u8, u32> = HashMap::new();
        for part in  line.trim_matches(|c| c == '{' || c == '}').split(',') {
            let (category, value) = part.split_once('=').unwrap();
            rating.insert(category.as_bytes()[0], value.parse::<u32>().unwrap());
        }
        ratings.push(rating);
    }
    ratings
}

fn trace_outcome(rating: &Rating, workflows: &Workflows) -> u32 {
    let mut outcome = workflows["in"].apply(rating);

    while outcome != "A" && outcome != "R" {
        outcome = workflows[&outcome].apply(rating);
    }

    if outcome == "A" {
        rating.values().sum()
    } else {
        0
    }
}

fn distinct_combinations_of_ratings(workflows: &Workflows) -> u64 {
    // TODO cleanup later
    let possible_ratings: PossibleRatings = [(1, 4000); 4];
    let mut valid_rating: Vec<PossibleRatings> = Vec::new();

    workflows["in"].find_combinations(possible_ratings, workflows, &mut valid_rating);
    valid_rating.iter().map(valid_combinations).sum()
}

fn valid_combinations(possible_ratings: &PossibleRatings) -> u64 {
    possible_ratings.iter().map(| (start, end) | (end - start + 1) as u64).product()
}

fn range_compare(outer: (u32, u32), inner: (u32, u32)) -> (u32, u32) {
    if outer.0 == inner.0 {
        (inner.1 + 1, outer.1)
    } else {
        (outer.0, inner.0 -1)
    }
}

fn update(mut ratings: [(u32, u32); 4], category: u8, with: (u32, u32)) -> [(u32, u32); 4] {
    ratings[category_to_index(&category)] = with;
    ratings
}

fn category_to_index(category: &u8) -> usize {
    match category {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        b's' => 3,
        _ => unreachable!()
    }
}