pub fn run() {
    let input = include_str!("../input/day06.txt");
    let chars: Vec<_> = input.chars().collect();

    fn find_window(input: &[char], size: usize) -> usize {
        let n = input.windows(size).position(|window| {
            for (i, a) in window.iter().enumerate() {
                for b in window[i + 1..].iter() {
                    if a == b {
                        return false;
                    }
                }
            }
            true
        });

        n.map(|x| x + size).unwrap_or_default()
    }
    println!("Day06a: {}", find_window(&chars, 4));
    println!("Day06b: {}", find_window(&chars, 14));
}
