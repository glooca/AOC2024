use std::time::Instant;

use glam::{IVec2, UVec2};

const DIRECTIONS: [IVec2; 8] = [
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
    IVec2::new(-1, -1),
];

struct WordSearch {
    chars: Vec<Vec<char>>,
    width: u32,
    height: u32,
}

impl WordSearch {
    fn parse(input: &str) -> Self {
        let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let height = chars.len() as u32;
        let width = chars[0].len() as u32;
        Self {
            chars,
            width,
            height,
        }
    }

    fn contains(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.y >= 0 && (pos.x as u32) < self.width && (pos.y as u32) < self.height
    }

    fn positions(&self) -> impl Iterator<Item = UVec2> + use<'_> {
        (0..self.height).flat_map(|x| (0..self.width).map(move |y| (x, y).into()))
    }

    fn find_characters(&self, char: char) -> impl Iterator<Item = UVec2> + use<'_> {
        self.positions()
            .filter(move |pos| self.chars[pos.y as usize][pos.x as usize] == char)
    }

    fn word_match(&self, word: &str, position: IVec2, offset: IVec2) -> bool {
        word.chars().enumerate().all(|(i, char)| {
            let pos = position + offset * i as i32;
            if !self.contains(pos) {
                return false;
            }
            self.chars[pos.y as usize][pos.x as usize] == char
        })
    }

    fn word_match_reversible(&self, word: &str, position: IVec2, offset: IVec2) -> bool {
        self.word_match(word, position, offset)
            || self.word_match(word, position + offset * (word.len() as i32 - 1), -offset)
    }

    fn find_words<'a>(&self, word: &'a str) -> impl Iterator<Item = (UVec2, IVec2)> + use<'_, 'a> {
        let starts = self.find_characters(word.chars().next().expect("Invalid word"));

        let candidates =
            starts.flat_map(move |start| DIRECTIONS.iter().map(move |dir| (start, *dir)));
        candidates.filter(|(start, dir)| self.word_match(word, start.as_ivec2(), *dir))
    }
}

fn part1(input: &str) -> usize {
    let ws = WordSearch::parse(input);
    ws.find_words("XMAS").count()
}

fn part2(input: &str) -> usize {
    let ws = WordSearch::parse(input);

    let word = "MAS";
    let dir1 = IVec2::new(1, 1);
    let dir2 = IVec2::new(1, -1);

    ws.find_characters('A')
        .filter(|pos| pos.x > 0 && pos.y > 0 && pos.x < ws.width - 1 && pos.y < ws.height - 1)
        .filter(|pos| {
            let pos = pos.as_ivec2();
            if !ws.word_match_reversible(&word, pos - dir1, dir1) {
                return false;
            }
            if !ws.word_match_reversible(&word, pos - dir2, dir2) {
                return false;
            }
            true
        })
        .count()
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
