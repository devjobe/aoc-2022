use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input/day09.txt");

    let motions: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let (d, n) = line.split_once(' ')?;
            let v: i32 = n.parse().ok()?;
            match d {
                "D" => Some((0, -v)),
                "U" => Some((0, v)),
                "L" => Some((-v, 0)),
                "R" => Some((v, 0)),
                _ => panic!("Unexpected direction"),
            }
        })
        .collect();

    {
        let mut h = (0, 0);
        let mut t = (0, 0);
        let mut positions = HashSet::new();

        positions.insert(t);

        for &(x, y) in motions.iter() {
            h = (h.0 + x, h.1 + y);
            if (h.0 - t.0).abs() <= 1 && (h.1 - t.1).abs() <= 1 {
                continue;
            }

            let (x, y) = (x.signum(), y.signum());

            if x != 0 {
                t = (t.0, h.1);
                while t.0 + x != h.0 {
                    t.0 += x;
                    positions.insert(t);
                }
            } else {
                t = (h.0, t.1);
                while t.1 + y != h.1 {
                    t.1 += y;
                    positions.insert(t);
                }
            }
        }

        println!("Day09a: {}", positions.len());
    }

    {
        let mut rope = [(0, 0); 10];
        let mut positions = HashSet::new();
        positions.insert(rope[0]);

        for &(x, y) in motions.iter() {
            let mut s = rope[0];
            rope[0] = (s.0 + x, s.1 + y);
            let (dx, dy) = (x.signum(), y.signum());

            while s != rope[0] {
                let mut h = (s.0 + dx, s.1 + dy);
                s = h;

                let updated = rope.iter_mut().skip(1).all(|t| {
                    let mut dx = h.0 - t.0;
                    let mut dy = h.1 - t.1;

                    let mut any = false;
                    if dx.abs() >= 2 {
                        dx -= dx.signum();
                        any = true;
                    }

                    if dy.abs() >= 2 {
                        dy -= dy.signum();
                        any = true;
                    }

                    if any {
                        h = (t.0 + dx, t.1 + dy);
                        *t = h;
                    }
                    return any;
                });

                if updated {
                    positions.insert(rope[9]);
                }
            }
        }

        println!("Day09b: {}", positions.len());
    }
}
