use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

pub fn run() {
    let input = include_str!("../input/day24.txt");

    let mut map: Vec<&[u8]> = input
        .lines()
        .skip(1)
        .map(|line| {
            let b = line.as_bytes();
            &b[1..b.len() - 1]
        })
        .collect();
    map.pop();

    let height = map.len();
    let width = map[0].len();

    let state = |visited: &mut HashSet<((usize, usize), usize)>,
                 pos: (usize, usize),
                 t0: usize,
                 minute: usize| {
        if visited.insert((pos, minute)) == false {
            return false;
        }

        let row = map[pos.1];

        let w_offset = (t0 + minute) % width;
        for (n, &ch) in row.iter().enumerate() {
            let t = match ch {
                b'<' => (width + n - w_offset) % width,
                b'>' => (n + w_offset) % width,
                _ => continue,
            };

            if t == pos.0 {
                return false;
            }
        }

        let h_offset = (t0 + minute) % height;

        for (n, ch) in map.iter().map(|row| row[pos.0]).enumerate() {
            let t = match ch {
                b'^' => (height + n - h_offset) % height,
                b'v' => (n + h_offset) % height,
                _ => continue,
            };

            if t == pos.1 {
                return false;
            }
        }

        true
    };

    fn gcd(a: usize, b: usize) -> usize {
        fn inner_gcd(a: usize, b: usize) -> usize {
            let m = b % a;
            if m == 0 {
                return a;
            }
            inner_gcd(m, a)
        }

        if a < b {
            inner_gcd(a, b)
        } else {
            inner_gcd(b, a)
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        (a * b) / gcd(a, b)
    }

    let period = lcm(width, height);

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Item((usize, usize), usize, usize);

    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(
                match match other.2.cmp(&self.2) {
                    Ordering::Equal => other.1.cmp(&self.1),
                    r => return Some(r),
                } {
                    Ordering::Equal => self.0.cmp(&other.0),
                    r => r,
                },
            )
        }
    }
    impl Ord for Item {
        fn cmp(&self, other: &Self) -> Ordering {
            match match other.2.cmp(&self.2) {
                Ordering::Equal => other.1.cmp(&self.1),
                r => return r,
            } {
                Ordering::Equal => self.0.cmp(&other.0),
                r => return r,
            }
        }
    }

    let find_best = |start: (usize, usize), end: (usize, usize), t0: usize| -> usize {
        let (pos_estimate, neg_estimate, t0_estimate) = if start.0 < end.0 {
            (2, 0, end.0 + end.1)
        } else {
            (0, 2, start.0 + start.1)
        };

        let mut queue = BinaryHeap::new();
        let mut best = usize::MAX;
        let mut visited = HashSet::new();
        for t in 1..=period {
            if t + t0_estimate >= best {
                break;
            }
            if !state(&mut visited, start, t0, t) {
                continue;
            }

            queue.clear();
            queue.push(Item(start, t, t + t0_estimate));
            while let Some(Item(pos, t, estimate)) = queue.pop() {
                if pos == end {
                    if t < best {
                        best = t;
                    }
                    break;
                }

                let t = t + 1;
                if state(&mut visited, pos, t0, t) {
                    queue.push(Item(pos, t, estimate + 1));
                }

                if pos.0 > 0 && state(&mut visited, (pos.0 - 1, pos.1), t0, t) {
                    queue.push(Item((pos.0 - 1, pos.1), t, estimate + neg_estimate));
                }

                if pos.0 + 1 < width && state(&mut visited, (pos.0 + 1, pos.1), t0, t) {
                    queue.push(Item((pos.0 + 1, pos.1), t, estimate + pos_estimate));
                }

                if pos.1 > 0 && state(&mut visited, (pos.0, pos.1 - 1), t0, t) {
                    queue.push(Item((pos.0, pos.1 - 1), t, estimate + neg_estimate));
                }

                if pos.1 + 1 < height && state(&mut visited, (pos.0, pos.1 + 1), t0, t) {
                    queue.push(Item((pos.0, pos.1 + 1), t, estimate + pos_estimate));
                }
            }
        }
        best + 1
    };
    let start = (0, 0);
    let end = (map[0].len() - 1, map.len() - 1);

    let c1 = find_best(start, end, 0);
    let c2 = find_best(end, start, c1);
    let c3 = find_best(start, end, c1 + c2);

    println!("Day24a: {c1}");
    println!("Day24b: {}", c1 + c2 + c3);
}
