use std::time::Instant;

#[derive(Debug)]
struct Equation {
    test_value: usize,
    values: Vec<usize>,
}

impl Equation {
    fn parse(line: &'static str) -> Option<Self> {
        let Some((test_value, values)) = line.split_once(":") else {
            return None;
        };
        let Ok(test_value) = test_value.parse() else {
            return None;
        };
        let values = values
            .trim()
            .split_whitespace()
            .flat_map(|v| v.parse())
            .collect();

        Some(Self { test_value, values })
    }

    fn is_possible(&self, operators: &[fn(usize, usize) -> Option<usize>]) -> bool {
        let mut values = self.values.iter();
        let Some(&first) = values.next() else {
            return false;
        };
        let mut solutions = vec![first];
        for &value in values {
            solutions = operators
                .iter()
                .flat_map(|op| solutions.iter().flat_map(|&v| op(v, value)))
                .filter(|&v| v <= self.test_value)
                .collect();
            if solutions.contains(&self.test_value) {
                return true;
            }
        }
        false
    }
}

fn concatenate(a: usize, b: usize) -> Option<usize> {
    let mut digits = 0;
    let mut temp = b;
    while temp > 0 {
        digits += 1;
        temp /= 10;
    }
    Some(a * 10_usize.pow(digits) + b)
}

fn part1(input: &'static str) -> usize {
    input
        .lines()
        .flat_map(Equation::parse)
        .filter(|e| e.is_possible(&[usize::checked_add, usize::checked_mul]))
        .map(|e| e.test_value)
        .sum()
}

fn part2(input: &'static str) -> usize {
    input
        .lines()
        .flat_map(Equation::parse)
        .filter(|e| e.is_possible(&[usize::checked_add, usize::checked_mul, concatenate]))
        .map(|e| e.test_value)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");

    let start1 = Instant::now();
    let part1 = part1(input);
    let time1 = Instant::now().duration_since(start1);

    let start2 = Instant::now();
    let part2 = part2(input);
    let time2 = Instant::now().duration_since(start2);

    println!(
        "Part 1: {} in {:?}\nPart 2: {} in {:?}",
        part1, time1, part2, time2
    );
}
