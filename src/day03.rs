pub fn run() {
    let input = include_str!("../input/day03.txt");
    let lines = input
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            assert_eq!(a.len(), b.len());
            a.chars().find(|&x| b.chars().any(|y| x == y))
        })
        .map(|x| match x {
            'a'..='z' => 1 + x as i32 - 'a' as i32,
            'A'..='Z' => 27 + x as i32 - 'A' as i32,
            _ => 0,
        });

    println!("Day03a: {}", lines.sum::<i32>());

    let lines = input
        .lines()
        .array_chunks()
        .filter_map(|[a, b, c]| {
            a.chars()
                .find(|&x| b.chars().any(|y| x == y) && c.chars().any(|z| x == z))
        })
        .map(|x| match x {
            'a'..='z' => 1 + x as i32 - 'a' as i32,
            'A'..='Z' => 27 + x as i32 - 'A' as i32,
            _ => 0,
        });

    println!("Day03b: {}", lines.sum::<i32>());
}
