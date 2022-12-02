pub fn run() {
    let input = include_str!("../input/day02.txt");
    let rounds = input.lines().filter_map(|line| {
        let (key, value) = line.split_once(' ')?;
        let a = match key {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => panic!("Unknown move: {key}"),
        };
        let b = match value {
            "X" => 0,
            "Y" => 1,
            "Z" => 2,
            _ => panic!("Unknown move: {value}"),
        };
        Some((a, b))
    });

    let mut sum1 = 0;
    let mut sum2 = 0;
    for (m1, m2) in rounds {
        sum1 += m2 + 1 + (m1 * 2 + 1 + m2) % 3 * 3;

        let m3 = (m1 + 2 + m2) % 3;
        sum2 += m3 + 1 + m2 * 3;
    }

    println!("Day02a: {sum1}");
    println!("Day02b: {sum2}");
}
