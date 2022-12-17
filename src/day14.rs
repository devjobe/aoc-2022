use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input/day14.txt");
    let rock_paths = input.lines().map(|line| {
        line.split(" -> ").filter_map(|item| {
            let (x, y) = item.split_once(',')?;
            Some((x.parse::<i32>().ok()?, y.parse::<i32>().ok()?))
        })
    });

    let mut rocks = HashSet::new();

    let mut max_y = 0;

    for mut path in rock_paths {
        let Some(mut from) = path.next() else {
            continue;
        };
        for to in path {

            if from.0 == to.0 {
                for y in from.1.min(to.1)..=from.1.max(to.1) {
                    rocks.insert((from.0, y));
                }
            } else {
                assert_eq!(from.1, to.1);
                for x in from.0.min(to.0)..=from.0.max(to.0) {
                    rocks.insert((x, from.1));
                }
            }
            
            max_y = max_y.max(from.1);
            from = to;
        }
    }

    let initial = rocks.len();
    'done: loop {
        let (mut x, mut y) = (500, 0);

        loop {
            if y >= max_y {
                break 'done;
            }

            if rocks.contains(&(x, y+1)) == false {
                y += 1;
            } else if rocks.contains(&(x-1, y+1)) == false {
                x -= 1;
                y += 1;
            } else if rocks.contains(&(x+1, y+1)) == false {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        if !rocks.insert((x, y)) {
            break;
        }
    }

    println!("Day14a: {}", rocks.len()-initial);

    let floor = max_y + 1;
    loop {
        let (mut x, mut y) = (500, 0);

        loop {
            if y >= floor {
                break;
            }

            if rocks.contains(&(x, y+1)) == false {
                y += 1;
            } else if rocks.contains(&(x-1, y+1)) == false {
                x -= 1;
                y += 1;
            } else if rocks.contains(&(x+1, y+1)) == false {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        if !rocks.insert((x, y)) {
            break;
        }
    }

    println!("Day14b: {}", rocks.len()-initial);
}
