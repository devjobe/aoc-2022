pub fn run1(input: &'static str) {
    let (map_input, path_input) = {
        let mut it = crate::split_paragraphs(input);
        (it.next().unwrap(), it.next().unwrap())
    };

    let map = map_input
        .lines()
        .map(|line| {
            let offset = line
                .as_bytes()
                .iter()
                .position(|x| *x != b' ')
                .unwrap_or_default();
            (offset, &line.as_bytes()[offset..])
        })
        .collect::<Vec<_>>();
    let width = map
        .iter()
        .map(|(start, row)| start + row.len())
        .max()
        .unwrap_or_default();
    let v_map = (0..width)
        .map(|x| {
            let start = map
                .iter()
                .position(|(start, row)| *start <= x && x < start + row.len())
                .unwrap_or_default();

            let col: Vec<u8> = map
                .iter()
                .skip(start)
                .filter_map(|(start, row)| {
                    if *start <= x && x < start + row.len() {
                        Some(row[x - *start])
                    } else {
                        None
                    }
                })
                .collect();
            (start, col)
        })
        .collect::<Vec<_>>();

    const OPEN: u8 = b'.';
    const WALL: u8 = b'#';

    let path = {
        let mut it = path_input.as_bytes().iter();
        let mut path = Vec::new();
        let mut offset = 0;
        let mut turn = 1;
        loop {
            match it.next() {
                Some(ch) if ch.is_ascii_digit() => offset = offset * 10 + (ch - b'0') as usize,
                Some(&b'L') => {
                    path.push((offset as isize) * turn);
                    offset = 0;
                    turn = -1;
                }
                Some(&b'R') => {
                    path.push(offset as isize * turn);
                    offset = 0;
                    turn = 1;
                }
                _ => {
                    path.push(offset as isize * turn);
                    break;
                }
            }
        }
        path
    };

    let start = (
        map[0].1.iter().position(|x| *x == OPEN).unwrap() + map[0].0,
        0usize,
    );

    const RIGHT: i32 = 0;
    const DOWN: i32 = 1;
    const LEFT: i32 = 2;
    const UP: i32 = 3;

    // let mut facing = RIGHT;
    let mut facing = UP;
    let mut position = start;

    fn forward_walk(origin: usize, start: usize, row: &[u8], offset: usize) -> usize {
        let index = origin - start;
        let steps = row
            .iter()
            .cycle()
            .skip(index + 1)
            .take(offset)
            .position(|x| *x == WALL)
            .unwrap_or(offset);

        start + (index + steps) % row.len()
    }

    fn reverse_walk(origin: usize, start: usize, row: &[u8], offset: usize) -> usize {
        let index = origin - start;
        let steps = row[..index]
            .iter()
            .rev()
            .chain(row.iter().rev().cycle())
            .take(offset)
            .position(|x| *x == WALL)
            .unwrap_or(offset)
            % row.len();

        start + (row.len() - steps + index) % row.len()
    }

    for walk in path.iter() {
        let turn = walk.signum() as i32;
        facing = (facing + turn).rem_euclid(4);

        let offset = walk.abs() as usize;

        match facing {
            RIGHT => {
                let (start, row) = map[position.1];
                position.0 = forward_walk(position.0, start, row, offset);
            }
            LEFT => {
                let (start, row) = map[position.1];
                position.0 = reverse_walk(position.0, start, row, offset);
            }
            DOWN => {
                let (start, row) = &v_map[position.0];
                position.1 = forward_walk(position.1, *start, row, offset);
            }
            UP => {
                let (start, row) = &v_map[position.0];
                position.1 = reverse_walk(position.1, *start, row, offset);
            }
            _ => unreachable!(),
        }
    }

    println!(
        "Day22a: {}",
        (position.1 + 1) * 1000 + (position.0 + 1) * 4 + facing as usize
    );
}

pub fn run2(input: &'static str) {
    let (map_input, path_input) = {
        let mut it = crate::split_paragraphs(input);
        (it.next().unwrap(), it.next().unwrap())
    };

    let map = map_input
        .lines()
        .map(|line| {
            let offset = line
                .as_bytes()
                .iter()
                .position(|x| *x != b' ')
                .unwrap_or_default();
            (offset, &line.as_bytes()[offset..])
        })
        .collect::<Vec<_>>();
    const OPEN: u8 = b'.';
    const WALL: u8 = b'#';

    let path = {
        let mut it = path_input.as_bytes().iter();
        let mut path = Vec::new();
        let mut offset = 0;
        let mut turn = 1;
        loop {
            match it.next() {
                Some(ch) if ch.is_ascii_digit() => offset = offset * 10 + (ch - b'0') as usize,
                Some(&b'L') => {
                    path.push((offset as isize) * turn);
                    offset = 0;
                    turn = -1;
                }
                Some(&b'R') => {
                    path.push(offset as isize * turn);
                    offset = 0;
                    turn = 1;
                }
                _ => {
                    path.push(offset as isize * turn);
                    break;
                }
            }
        }
        path
    };

    let width = map
        .iter()
        .map(|(start, row)| start + row.len())
        .max()
        .unwrap_or_default();

    let side = width.max(map.len()) / 4;

    let v_map = (0..width)
        .map(|x| {
            let start = map
                .iter()
                .position(|(start, row)| *start <= x && x < start + row.len())
                .unwrap_or_default();

            let col: Vec<u8> = map
                .iter()
                .skip(start)
                .filter_map(|(start, row)| {
                    if *start <= x && x < start + row.len() {
                        Some(row[x - *start])
                    } else {
                        None
                    }
                })
                .collect();
            (start, col)
        })
        .collect::<Vec<_>>();

    struct Chunk {
        position: (usize, usize),
        data: Vec<Vec<u8>>,
    }

    let grid = map
        .chunks(side)
        .enumerate()
        .map(|(grid_y, chunk)| {
            let start = chunk[0].0;
            let end = chunk[0].1.len() + start;
            assert!(chunk.iter().all(|x| x.0 == start));

            let y = grid_y * side;
            (0..width)
                .step_by(side)
                .map(|x| {
                    if x < start || x >= end {
                        None
                    } else {
                        Some(Chunk {
                            position: (x, y),
                            data: chunk
                                .iter()
                                .map(|(_s, row)| Vec::from(&row[x - start..][..side]))
                                .collect(),
                        })
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = (
        map[0].1.iter().position(|x| *x == OPEN).unwrap() + map[0].0,
        0usize,
    );

    const RIGHT: i32 = 0;
    const DOWN: i32 = 1;
    const LEFT: i32 = 2;
    const UP: i32 = 3;

    let mut facing = UP;
    let mut position = start;

    fn forward_walk(origin: usize, start: usize, row: &[u8], offset: usize) -> (usize, usize) {
        let index = origin - start;
        for (pos, x) in row.iter().enumerate().skip(index + 1).take(offset) {
            if *x == WALL {
                let pos = pos - 1;
                return (start + pos, offset);
            }
        }

        let end = (row.len() - 1).min(index + offset);
        (start + end, (end - index))
    }

    fn reverse_walk(origin: usize, start: usize, row: &[u8], offset: usize) -> (usize, usize) {
        let index = origin - start;

        for (pos, x) in row[..index].iter().enumerate().rev().take(offset) {
            if *x == WALL {
                let pos = pos + 1;
                return (start + pos, offset);
            }
        }

        let n = index.min(offset);
        (start + index - n, n)
    }

    let map_open =
        |(grid_x, grid_y): (usize, usize), (tile_x, tile_y): (usize, usize), facing: i32| {
            let chunk: &Chunk = &grid[grid_y][grid_x].as_ref().unwrap();
            let cell = chunk.data[tile_y][tile_x];
            if cell != WALL {
                Some((
                    (chunk.position.0 + tile_x, chunk.position.1 + tile_y),
                    facing,
                ))
            } else {
                None
            }
        };

    for walk in path.iter() {
        let turn = walk.signum() as i32;
        facing = (facing + turn).rem_euclid(4);

        let mut offset = walk.abs() as usize;

        while offset > 0 {
            match facing {
                RIGHT => {
                    let (start, row) = map[position.1];
                    position.0 = {
                        let (x, n) = forward_walk(position.0, start, row, offset);
                        offset -= n;
                        x
                    };
                }
                LEFT => {
                    let (start, row) = map[position.1];
                    position.0 = {
                        let (x, n) = reverse_walk(position.0, start, row, offset);
                        offset -= n;
                        x
                    };
                }
                DOWN => {
                    let (start, row) = &v_map[position.0];
                    position.1 = {
                        let (x, n) = forward_walk(position.1, *start, row, offset);
                        offset -= n;
                        x
                    };
                }
                UP => {
                    let (start, row) = &v_map[position.0];
                    position.1 = {
                        let (x, n) = reverse_walk(position.1, *start, row, offset);
                        offset -= n;
                        x
                    };
                }
                _ => unreachable!(),
            }

            if offset > 0 {
                let grid_pos = (position.0 / side, position.1 / side);
                let (x, y) = (position.0 % side, position.1 % side);

                /*
                let res = match facing {
                    RIGHT => match grid_pos {
                        (2, 0) => map_open((3, 2), (side - 1, side - 1 - y), LEFT),
                        (2, 1) => map_open((3, 2), (side - 1 - y, 0), DOWN),
                        (3, 2) => map_open((2, 0), (side - 1, side - 1 - y), LEFT),
                        _ => unreachable!(),
                    },
                    LEFT => match grid_pos {
                        (2, 0) => map_open((1, 1), (y, 0), DOWN),
                        (0, 1) => map_open((3, 2), (side - 1 - y, side - 1), UP),
                        (2, 2) => map_open((1, 1), (side - 1 - y, side - 1), UP),
                        _ => unreachable!("left on {grid_pos:?}"),
                    },
                    DOWN => match grid_pos {
                        (0, 1) => map_open((2, 2), (side - 1 - x, side - 1), UP),
                        (1, 1) => map_open((2, 2), (0, side - 1 - x), RIGHT),
                        (2, 2) => map_open((0, 1), (side - 1 - x, side - 1), UP),
                        (3, 2) => map_open((3, 2), (0, side - 1 - x), RIGHT),
                        _ => unreachable!(),
                    },
                    UP => match grid_pos {
                        (0, 1) => map_open((2, 0), (side - 1 - x, 0), DOWN),
                        (1, 1) => map_open((2, 0), (0, x), RIGHT),
                        (2, 0) => map_open((0, 1), (side - 1 - x, 0), DOWN),
                        (3, 2) => map_open((1, 2), (side - 1, side - 1 - x), LEFT),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };
                */

                let res = match (facing, grid_pos) {
                    (LEFT, (1, 0)) => map_open((0, 2), (0, side - 1 - y), RIGHT),
                    (UP, (1, 0)) => map_open((0, 3), (0, x), RIGHT),

                    (UP, (2, 0)) => map_open((0, 3), (x, side - 1), UP),
                    (RIGHT, (2, 0)) => map_open((1, 2), (side - 1, side - 1 - y), LEFT),
                    (DOWN, (2, 0)) => map_open((1, 1), (side - 1, x), LEFT),

                    (LEFT, (1, 1)) => map_open((0, 2), (y, 0), DOWN),
                    (RIGHT, (1, 1)) => map_open((2, 0), (y, side - 1), UP),

                    (LEFT, (0, 2)) => map_open((1, 0), (0, side - 1 - y), RIGHT),
                    (UP, (0, 2)) => map_open((1, 1), (0, x), RIGHT),

                    (RIGHT, (1, 2)) => map_open((2, 0), (side - 1, side - 1 - y), LEFT),
                    (DOWN, (1, 2)) => map_open((0, 3), (side - 1, x), LEFT),

                    (RIGHT, (0, 3)) => map_open((1, 2), (y, side - 1), UP),
                    (DOWN, (0, 3)) => map_open((2, 0), (x, 0), DOWN),
                    (LEFT, (0, 3)) => map_open((1, 0), (y, 0), DOWN),
                    _ => unreachable!(),
                };

                if let Some((pos, face)) = res {
                    offset -= 1;
                    position = pos;
                    facing = face;
                } else {
                    break;
                }
            }
        }
    }

    println!(
        "Day22b: {}",
        (position.1 + 1) * 1000 + (position.0 + 1) * 4 + facing as usize
    );
}

pub fn run() {
    let input = include_str!("../input/day22.txt");
    run1(input);
    run2(input);
}
