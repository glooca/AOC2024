use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use glam::IVec2;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    antennae: HashMap<char, Vec<IVec2>>,
}

impl Map {
    fn parse(input: &str) -> Map {
        let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let width = chars.len();
        let height = chars.get(0).map(Vec::len).unwrap_or_default();
        let mut antennae: HashMap<char, Vec<IVec2>> = HashMap::new();
        chars
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .flat_map(move |(x, symbol)| match symbol {
                        '.' => None,
                        symbol => Some((*symbol, IVec2::new(x as i32, y as i32))),
                    })
            })
            .for_each(|(symbol, pos)| {
                antennae.entry(symbol).or_default().push(pos);
            });
        Map {
            width,
            height,
            antennae,
        }
    }

    fn contains(&self, pos: &IVec2) -> bool {
        return pos.x >= 0
            && pos.y >= 0
            && (pos.x as usize) < self.width
            && (pos.y as usize) < self.height;
    }
}

fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    let mut antinodes: HashSet<IVec2> = HashSet::new();
    for (_symbol, positions) in &map.antennae {
        for a1 in positions {
            for a2 in positions {
                if a1 == a2 {
                    continue;
                }
                let diff = a1 - a2;
                let a1 = a1 + diff;
                if map.contains(&a1) {
                    antinodes.insert(a1);
                }
                let a2 = a2 - diff;
                if map.contains(&a2) {
                    antinodes.insert(a2);
                }
            }
        }
    }
    antinodes.len()
}

fn part2(input: &str) -> usize {
    let map = Map::parse(input);
    let mut antinodes: HashSet<IVec2> = HashSet::new();
    for (_symbol, positions) in &map.antennae {
        for a1 in positions {
            for a2 in positions {
                if a1 == a2 {
                    antinodes.insert(*a1);
                    continue;
                }
                let diff = a1 - a2;
                let mut a1 = a1 + diff;
                while map.contains(&a1) {
                    antinodes.insert(a1);
                    a1 += diff;
                }
                let mut a2 = a2 - diff;
                while map.contains(&a2) {
                    antinodes.insert(a2);
                    a2 -= diff;
                }
            }
        }
    }
    antinodes.len()
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
