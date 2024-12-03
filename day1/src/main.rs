use std::{collections::HashMap, time::Instant};

struct Values {
    left: Box<dyn Iterator<Item = i32>>,
    right: Box<dyn Iterator<Item = i32>>,
}

impl Values {
    fn parse(input: &'static str) -> Self {
        let values = input
            .lines()
            .map(|l| l.split_whitespace().flat_map(|n| n.parse::<i32>()));

        let left = values.clone().flat_map(|mut l| l.nth(0));
        let right = values.flat_map(|mut l| l.nth(1));

        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

fn part1(input: &'static str) -> usize {
    let values = Values::parse(input);

    let mut left = values.left.collect::<Vec<_>>();
    let mut right = values.right.collect::<Vec<_>>();

    left.sort();
    right.sort();

    let left = left.iter();
    let right = right.iter();

    left.zip(right).map(|(l, r)| (l - r).abs() as usize).sum()
}

fn part2(input: &'static str) -> usize {
    let values = Values::parse(input);

    let mut counts = HashMap::<i32, usize>::new();
    for val in values.right {
        *counts.entry(val).or_default() += 1;
    }

    values
        .left
        .map(|left| left as usize * counts.get(&left).unwrap_or(&0))
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
