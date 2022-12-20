pub fn run() {
    let input = include_str!("../input/day20.txt");

    let encrypted = input
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    fn decrypt(encrypted: &Vec<i64>, key: usize, count: usize) -> i64 {
        let mut successors = (0..encrypted.len())
            .map(|x| {
                (
                    ((encrypted.len() + x - 1) % encrypted.len()),
                    (x + 1) % encrypted.len(),
                )
            })
            .collect::<Vec<_>>();
        let n_mod = encrypted.len() - 1;
        for _ in 0..count {
            for (index, i) in encrypted.iter().enumerate() {
                let (mut prev, mut next) = successors[index];
                successors[prev].1 = next;
                successors[next].0 = prev;
                if *i < 0 {
                    for _ in 0..((i.abs() as usize * key) % n_mod) {
                        next = prev;
                        prev = successors[prev].0;
                    }
                } else if *i > 0 {
                    for _ in 0..((*i as usize * key) % n_mod) {
                        prev = next;
                        next = successors[next].1;
                    }
                }
    
                successors[index] = (prev, next);
                successors[prev].1 = index;
                successors[next].0 = index;
            }    
        }

        let start = encrypted.iter().position(|x| *x == 0).unwrap();
        let mut current = start;
        let mut mix = Vec::with_capacity(encrypted.len());
        loop {
            mix.push(encrypted[current]);
            current = successors[current].1;
            if current == start {
                break;
            }
        }

        mix.iter()
            .cycle()
            .step_by(1000)
            .skip(1)
            .take(3)
            .map(|x| x * key as i64)
            .sum::<i64>()
    }

    println!("Day20a: {}", decrypt(&encrypted, 1, 1));
    println!("Day20b: {}", decrypt(&encrypted, 811589153, 10));
}

