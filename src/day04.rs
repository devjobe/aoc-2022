use std::ops::RangeInclusive;
pub fn run() {
    let input = include_str!("../input/day04.txt");

    fn to_range(text: &str) -> Option<RangeInclusive<i32>> {
        let (a, b) = text.split_once('-')?;
        Some(a.parse::<i32>().ok()?..=b.parse::<i32>().ok()?)
    }

    let sections: Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> = input
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once(',')?;
            Some((to_range(a)?, to_range(b)?))
        })
        .collect();

    let overlap_completely = sections
        .iter()
        .filter(|&(a, b)| {
            (a.contains(&b.start()) && a.contains(&b.end()))
                || (b.contains(&a.start()) && b.contains(&a.end()))
        })
        .count();
    let overlap_some = sections
        .iter()
        .filter(|(a, b)| {
            (a.contains(&b.start()) || a.contains(&b.end()))
                || (b.contains(&a.start()) || b.contains(&a.end()))
        })
        .count();

    println!("Day04a: {}", overlap_completely);
    println!("Day04b: {}", overlap_some);
}
