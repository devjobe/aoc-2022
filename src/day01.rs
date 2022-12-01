pub fn run() {
    let input = include_str!("../input/day01.txt");
    let paragraphs = input.split("\r\n\r\n");

    let mut lists = paragraphs
        .map(|x| {
            x.lines()
                .map(|x| i32::from_str_radix(x, 10).unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    lists.sort();

    println!("Day01a: {}", lists.last().unwrap_or(&0));
    println!("Day01b: {}", lists.iter().rev().take(3).cloned().sum::<i32>());
}
