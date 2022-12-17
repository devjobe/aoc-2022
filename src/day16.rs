pub fn run() {
    let input = include_str!("../input/day16.txt");

    struct Valve<'a> {
        valve: &'a str,
        flow: usize,
        tunnels: Vec<&'a str>,
    }

    let valves = input
        .lines()
        .map(|line| {
            let (valve_flow, tunnels) = line.split_once(';').unwrap();

            let valve = &valve_flow[6..8];
            let flow = (&valve_flow[23..]).parse::<usize>().unwrap();
            let tunnels = (&tunnels[23..])
                .trim_start()
                .split(", ")
                .collect::<Vec<_>>();
            Valve {
                valve,
                flow,
                tunnels,
            }
        })
        .collect::<Vec<_>>();

    fn floyd_warshall(valves: &Vec<Valve>) -> Vec<Vec<usize>> {
        let mut distances = vec![vec![usize::MAX; valves.len()]; valves.len()];
        for (idx, valve) in valves.iter().enumerate() {
            distances[idx][idx] = 0;
            for &dest in valve.tunnels.iter() {
                let d = valves.iter().position(|x| x.valve == dest).unwrap();
                distances[idx][d] = 1;
            }
        }

        let n = valves.len();
        for (k, i, j) in (0..n)
            .map(move |k| {
                (0..n)
                    .map(move |i| (0..n).map(move |j| (k, i, j)))
                    .flatten()
            })
            .flatten()
        {
            let current = distances[i][j];
            if let Some(intermediate) = distances[i][k].checked_add(distances[k][j]) {
                if current > intermediate {
                    distances[i][j] = intermediate;
                }
            }
        }
        distances
    }

    struct Edge {
        cost: usize,
        flow: usize,
    }

    let (start, costs) = {
        let distances = floyd_warshall(&valves);
        let filtered = valves
            .iter()
            .enumerate()
            .filter(|(_, valve)| valve.flow != 0 || valve.valve == "AA");

        let start = filtered
            .clone()
            .position(|(_, valve)| valve.valve == "AA")
            .unwrap();
        let mut costs = filtered
            .clone()
            .map(move |(idx, _)| {
                filtered
                    .clone()
                    .map(|(idx2, &Valve { flow, .. })| {
                        distances[idx][idx2]
                            .checked_add(1)
                            .map(|cost| Edge { cost, flow })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let last = costs.len()-1;
        for idx in 0..costs.len() {
            costs[idx].swap(start, last);
        }
        costs.swap(start, last);

        (last, costs)
    };

    fn dfs(idx: usize, time: usize, visited: usize, costs: &Vec<Vec<Option<Edge>>>) -> usize {
        let visited = visited | 1 << idx;
        let mut best = 0;
        for (current_idx, edge) in costs[idx].iter().enumerate() {
            if (visited & (1 << current_idx)) != 0 {
                continue;
            }
            if let Some(Edge { cost, flow, .. }) = edge {
                if *cost >= time {
                    continue;
                }
                let remaining_time = time - *cost;
                let pressure = *flow * remaining_time;
                best = best.max(dfs(current_idx, remaining_time, visited, costs) + pressure);
            }
        }
        best
    }

    assert!(costs.len() <= 64);
    
    let max_release = dfs(start, 30, 0, &costs);
    println!("Day16a: {}", max_release);

    let n = 1 << costs.len();
    let releases = (0..n)
        .map(|visited| dfs(start, 26, visited, &costs))
        .collect::<Vec<usize>>();

    let max_release2 = releases
        .iter()
        .zip(releases.iter().rev())
        .map(|(a, b)| a + b)
        .max()
        .unwrap_or_default();
    println!("Day16a: {}", max_release2);
}

