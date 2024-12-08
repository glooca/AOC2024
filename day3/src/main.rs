use std::time::Instant;

fn part1(input: &str) -> usize {
    input
        .split("mul(")
        .flat_map(|text| text.split_once(")").map(|(txt, _)| txt))
        .flat_map(|text| text.split_once(","))
        .flat_map(|(a, b)| {
            let Ok(a) = a.parse::<usize>() else {
                return None;
            };
            let Ok(b) = b.parse::<usize>() else {
                return None;
            };
            Some(a * b)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .split("do()")
        .flat_map(|v| v.split("don't()").next())
        .flat_map(|v| v.split("mul("))
        .flat_map(|v| v.split_once(")").map(|(v, _)| v))
        .flat_map(|v| v.split_once(","))
        .flat_map(|(a, b)| {
            let Ok(a) = a.parse::<usize>() else {
                return None;
            };
            let Ok(b) = b.parse::<usize>() else {
                return None;
            };
            Some(a * b)
        })
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
