use std::time::Instant;

#[derive(Debug)]
struct Report<'a> {
    line: &'a str,
}

impl<'a> Report<'a> {
    fn levels(&self) -> impl Iterator<Item = u8> + use<'_> {
        self.line.split(" ").flat_map(str::parse::<u8>)
    }
    fn parse(input: &'a str) -> impl Iterator<Item = Report<'a>> {
        input.lines().map(|line| Self { line })
    }
    fn level_diffs(&self) -> impl Iterator<Item = i8> + use<'_> {
        self.levels()
            .zip(self.levels().skip(1))
            .map(|(current, next)| next as i8 - current as i8)
    }
}

fn part1(input: &str) -> usize {
    #[inline]
    fn safe(report: &Report) -> bool {
        let mut level_diffs = report.level_diffs();
        let mut report_sign = 0;
        level_diffs.all(|diff| {
            let abs = diff.abs();
            let sign = diff.signum();
            if report_sign == 0 {
                report_sign = sign;
            }
            sign == report_sign && (1..=3).contains(&abs)
        })
    }

    Report::parse(input).filter(safe).count()
}

fn part2(input: &str) -> usize {
    #[inline]
    fn levels_safe(mut level_diffs: impl Iterator<Item = i8>) -> bool {
        let mut report_sign = 0;
        level_diffs.all(|diff| {
            let abs = diff.abs();
            let sign = diff.signum();
            if report_sign == 0 {
                report_sign = sign;
            }
            sign == report_sign && (1..=3).contains(&abs)
        })
    }

    #[inline]
    fn safe(report: &Report) -> bool {
        let levels = report.levels().collect::<Vec<_>>();
        if levels_safe(
            levels
                .iter()
                .zip(levels.iter().skip(1))
                .map(|(a, b)| *b as i8 - *a as i8),
        ) {
            return true;
        }
        let l = levels.len();
        for n in 0..l {
            let levels = levels
                .iter()
                .enumerate()
                .filter_map(|(i, e)| if i != n { Some(e) } else { None });
            let diffs = levels
                .clone()
                .zip(levels.skip(1))
                .map(|(a, b)| *b as i8 - *a as i8);
            if levels_safe(diffs) {
                return true;
            }
        }
        false
    }

    Report::parse(input).filter(safe).count()
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
