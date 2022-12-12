use std::{collections::HashMap, cmp::Ordering};

pub fn run() {
    let input = include_str!("../input/day12.txt");
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.into()).collect();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(row, heights)| {
            let column = heights.iter().position(|&x| x == b'S')?;
            Some((column, row))
        })
        .unwrap();

    grid[start.1][start.0] = b'a';

    let end = grid
        .iter()
        .enumerate()
        .find_map(|(row, heights)| {
            let column = heights.iter().position(|&x| x == b'E')?;
            Some((column, row))
        })
        .unwrap();

    grid[end.1][end.0] = b'z';

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Vertex {
        vertex: (usize, usize),
        cost: usize,
    }

    impl Vertex {
        pub fn height(&self, grid: &Vec<Vec<u8>>) -> u8 {
            grid[self.vertex.1][self.vertex.0]
        }

        pub fn adjacent(
            &self,
            offset: (i32, i32),
            grid: &Vec<Vec<u8>>,
        ) -> Option<((usize, usize), u8)> {
            let x = self.vertex.0 as isize + offset.0 as isize;
            let y = self.vertex.1 as isize + offset.1 as isize;

            if x < 0 || y < 0 {
                return None;
            }

            let x = x as usize;
            let y = y as usize;

            grid.get(y).and_then(|row| row.get(x)).map(|&h| ((x, y), h))
        }
    }

    impl Ord for Vertex {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.vertex.cmp(&other.vertex))
        }
    }

    impl PartialOrd for Vertex {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut open = std::collections::BinaryHeap::new();

    let start_edge = Vertex {
        vertex: end,
        cost: 0,
    };
    open.push(start_edge.clone());

    let mut distance: HashMap<(usize, usize), usize> = HashMap::new();
    distance.insert(end, 0);

    while let Some(current) = open.pop() {
        match distance.get(&current.vertex) {
            Some(&cost) if cost < current.cost => continue,
            _ => (),
        }

        let to = current.height(&grid);
        let step_cost = current.cost + 1;
        for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let Some((adjacent, from)) = current.adjacent(offset, &grid) else {
                continue;
            };

            if to > from + 1 {
                continue;
            }

            match distance.get(&adjacent) {
                Some(&cost) if cost <= step_cost => continue,
                _ => (),
            }

            distance.insert(adjacent, step_cost);

            open.push(Vertex {
                vertex: adjacent,
                cost: step_cost,
            });
        }
    }

    let cost = *distance.get(&start).unwrap();
    println!("Day12a: {cost}");

    let min_cost = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, h)| {
                    if *h == b'a' {
                        distance.get(&(x, y)).cloned()
                    } else {
                        None
                    }
                })
                .min()
                .unwrap_or(usize::MAX)
        })
        .min()
        .unwrap_or(usize::MAX);

    println!("Day12b: {min_cost}");
}
