use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("../input/day23.txt");
    let mut state = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(x, ch)| {
                    if *ch == b'#' {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .collect::<HashSet<_>>();

    const CARDINALS: [(isize, isize); 8] = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
    ];

    const N: usize = 0;
    const S: usize = 1;
    const W: usize = 2;
    const E: usize = 3;
    const NW: usize = 4;
    const NE: usize = 5;
    const SW: usize = 6;
    const SE: usize = 7;

    const NORTH: usize = 1 << NW | 1 << N | 1 << NE;
    const SOUTH: usize = 1 << SW | 1 << S | 1 << SE;
    const WEST: usize = 1 << NW | 1 << W | 1 << SW;
    const EAST: usize = 1 << NE | 1 << E | 1 << SE;

    const DIRECTIONS: [(usize, (isize, isize)); 4] = [
        (NORTH, CARDINALS[N]),
        (SOUTH, CARDINALS[S]),
        (WEST, CARDINALS[W]),
        (EAST, CARDINALS[E]),
    ];

    let mut proposal = HashMap::new();
    for n in 0.. {
        proposal.clear();

        for &(x, y) in state.iter() {
            let mut occupied = 0usize;

            for (dir, (a, b)) in CARDINALS.into_iter().enumerate() {
                if state.contains(&(x + a, y + b)) {
                    occupied |= 1 << dir;
                }
            }

            if occupied == 0 {
                continue;
            }

            for i in 0..4 {
                let (mask, (a, b)) = DIRECTIONS[(n + i) % DIRECTIONS.len()];

                if occupied & mask == 0 {
                    let p = (x + a, y + b);
                    if let Some(c) = proposal.get_mut(&p) {
                        *c = (isize::MAX, isize::MAX);
                    } else {
                        proposal.insert(p, (x, y));
                    }
                    break;
                }
            }
        }

        let mut moved = false;
        for (k, v) in proposal.iter() {
            if state.remove(v) {
                state.insert(*k);
                moved = true;
            }
        }

        let round = n + 1;
        if round == 10 {
            let start = state.iter().fold((isize::MAX, isize::MAX), |acc, key| {
                (key.0.min(acc.0), key.1.min(acc.1))
            });
            let end = state.iter().fold((isize::MIN, isize::MIN), |acc, key| {
                (key.0.max(acc.0), key.1.max(acc.1))
            });

            let x = end.0 - start.0 + 1;
            let y = end.1 - start.1 + 1;

            let total = x * y;
            let empty = total as usize - state.len();

            println!("Day23a: {empty}");
        }

        if !moved {
            println!("Day23b: {round}");
            break;
        }
    }
}
