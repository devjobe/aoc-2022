pub fn run() {
    let input = include_str!("../input/day05.txt");

    let Some((p1, p2)) = input.split_once("\r\n\r\n") else {
        println!("Expected two paragraphs");
        return;
    };

    let mut stacks: [Vec<u8>; 9] = std::array::from_fn(|_| Vec::with_capacity(32));

    for line in p1.lines().map(|line| {
        let mut iter = line.chars().skip(1).step_by(4);
        let list: [char; 9] = std::array::from_fn(move |_n| iter.next().unwrap_or(' '));
        list
    }) {
        for (stack, value) in stacks.iter_mut().zip(line.iter()) {
            match value {
                'A'..='Z' => stack.push(*value as u8),
                _ => (),
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let mut stacks9001 = stacks.clone();
    p2.lines().for_each(|line| {
        // move x to y from z
        let mut iter = line.split_whitespace().skip(1).step_by(2);
        let [count, src, dst] = std::array::from_fn(move |_n| {
            iter.next()
                .unwrap_or("")
                .parse::<usize>()
                .unwrap_or_default()
        });

        assert_ne!(count, 0);
        assert_ne!(dst, 0);
        assert_ne!(src, 0);

        assert!(dst <= 9);
        assert!(src <= 9);

        assert_ne!(dst, src);

        let dst = dst - 1;
        let src = src - 1;
        for _ in 0..count {
            if let Some(x) = stacks[src].pop() {
                stacks[dst].push(x)
            }
        }
        let n = stacks9001[dst].len();
        for _ in 0..count {
            if let Some(x) = stacks9001[src].pop() {
                stacks9001[dst].push(x)
            }
        }
        stacks9001[dst][n..].reverse();
    });

    let result: String = String::from_utf8(
        stacks
            .iter()
            .filter_map(|s| s.last().cloned())
            .collect::<Vec<u8>>(),
    )
    .unwrap_or_default();
    println!("Day05a: {}", result);

    let result: String = String::from_utf8(
        stacks9001
            .iter()
            .filter_map(|s| s.last().cloned())
            .collect::<Vec<u8>>(),
    )
    .unwrap_or_default();
    println!("Day05b: {}", result);
}
