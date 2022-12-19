use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input/day18.txt");

    let cubes = input
        .lines()
        .map(|line| {
            let mut iter = line.split(',');

            let (x, y, z) = (
                iter.next().unwrap().parse::<i32>().ok().unwrap(),
                iter.next().unwrap().parse::<i32>().ok().unwrap(),
                iter.next().unwrap().parse::<i32>().ok().unwrap(),
            );
            (x, y, z)
        })
        .collect::<HashSet<_>>();

    let get_neighbours = |c: (i32, i32, i32)| {
        [
            (c.0 + 1, c.1, c.2),
            (c.0 - 1, c.1, c.2),
            (c.0, c.1 + 1, c.2),
            (c.0, c.1 - 1, c.2),
            (c.0, c.1, c.2 + 1),
            (c.0, c.1, c.2 - 1),
        ]
    };

    let area = cubes
        .iter()
        .map(|c| {
            get_neighbours(*c)
                .iter()
                .filter(|x| cubes.contains(*x) == false)
                .count()
        })
        .sum::<usize>();

    println!("Day18a: {area}");

    let max_c = cubes.iter().fold((i32::MIN, i32::MIN, i32::MIN), |acc, c| {
        (acc.0.max(c.0), acc.1.max(c.1), acc.2.max(c.2))
    });

    let min_c = cubes.iter().fold((i32::MAX, i32::MAX, i32::MAX), |acc, c| {
        (acc.0.min(c.0), acc.1.min(c.1), acc.2.min(c.2))
    });

    let is_outside = |(x, y, z): (i32, i32, i32)| {
        x < min_c.0 || x > max_c.0 || y < min_c.1 || y > max_c.1 || z < min_c.2 || z > max_c.2
    };

    let mut filled_cube = cubes.clone();
    let mut queue = Vec::new();
    for x in min_c.0..=max_c.0 {
        for y in min_c.1..=max_c.1 {
            if filled_cube.insert((x, y, min_c.2)) {
                queue.push((x, y, min_c.2));
            }
            if filled_cube.insert((x, y, max_c.2)) {
                queue.push((x, y, max_c.2));
            }
        }
    }
    for z in min_c.2..=max_c.2 {
        for y in min_c.1..=max_c.1 {
            if filled_cube.insert((min_c.0, y, z)) {
                queue.push((min_c.0, y, z));
            }
            if filled_cube.insert((max_c.0, y, z)) {
                queue.push((max_c.0, y, z));
            }
        }
    }

    for x in min_c.0..=max_c.0 {
        for z in min_c.2..=max_c.2 {
            if filled_cube.insert((x, min_c.1, z)) {
                queue.push((x, min_c.1, z));
            }
            if filled_cube.insert((x, max_c.1, z)) {
                queue.push((x, max_c.1, z));
            }
        }
    }

    while let Some(c) = queue.pop() {
        for c in get_neighbours(c)
            .iter()
            .filter(|c| is_outside(**c) == false)
        {
            if filled_cube.insert(*c) {
                queue.push(*c);
            }
        }
    }

    let filled_area = filled_cube
        .iter()
        .map(|c| {
            get_neighbours(*c)
                .iter()
                .filter(|x| filled_cube.contains(*x) == false)
                .count()
        })
        .sum::<usize>();

    let cube_surface = (2 * (max_c.0 - min_c.0 + 1) * (max_c.1 - min_c.1 + 1)
        + 2 * (max_c.2 - min_c.2 + 1) * (max_c.1 - min_c.1 + 1)
        + 2 * (max_c.0 - min_c.0 + 1) * (max_c.2 - min_c.2 + 1)) as usize;
    let exterior = area - (filled_area - cube_surface);
    println!("Day18b: {exterior}");

    /* Slow solution:
    let mut outside = HashSet::new();
    let mut find_outside = |s: (i32, i32, i32)| -> bool {
        if is_outside(s) || outside.contains(&s) {
            return true;
        }
        let mut state = outside.clone();
        let mut queue = Vec::new();

        queue.push(s);
        state.insert(s);

        while let Some(c) = queue.pop() {
            for n in get_neighbours(c) {
                if is_outside(n) || outside.contains(&n) {
                    outside = state;
                    return true;
                }

                if cubes.contains(&n) {
                    continue;
                }

                if state.insert(n) {
                    queue.push(n);
                }
            }
        }

        false
    };


    let exterior = cubes
        .iter()
        .map(|c| {
            get_neighbours(*c)
                .iter()
                .filter(|x| {
                    if cubes.contains(*x) == false {
                        find_outside(**x)
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum::<usize>();
    println!("Day18b: {exterior}");
    */
}
