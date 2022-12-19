use rayon::prelude::*;
use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../input/day19.txt");
    let entries = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let (_, s) = line.split_once(": ").unwrap();

            let blueprint = s
                .split('.')
                .filter_map(|entry| {
                    if entry.is_empty() {
                        return None;
                    }

                    let (_name, costs) = entry
                        .trim()
                        .strip_prefix("Each ")
                        .unwrap()
                        .split_once(" ")
                        .unwrap();
                    let costs = costs.strip_prefix("robot costs ").unwrap();

                    fn get_cost(s: &str) -> usize {
                        s.split_once(" ").unwrap().0.parse::<usize>().unwrap()
                    }

                    let costs = if let Some((cost1, cost2)) = costs.split_once(" and ") {
                        (get_cost(cost1), get_cost(cost2))
                    } else {
                        (get_cost(costs), 0)
                    };

                    Some(costs)
                })
                .collect::<Vec<_>>();

            let (ore_ore, _a) = blueprint[0];
            let (clay_ore, _b) = blueprint[1];
            let (obsidian_ore, obsidian_clay) = blueprint[2];
            let (geode_ore, geode_obsidian) = blueprint[3];

            let bp = Blueprint {
                id: index + 1,
                ore_costs: [ore_ore, clay_ore, obsidian_ore, geode_ore],
                tier2: obsidian_clay,
                tier3: geode_obsidian,
            };

            bp
        })
        .collect::<Vec<_>>();

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
    struct Sim {
        res: [usize; 4],
        production: [usize; 4],
    }

    impl Sim {
        pub fn build(&mut self, bp: &Blueprint, index: usize) {
            self.res[0] -= bp.ore_costs[index];
            if index == 2 {
                self.res[1] -= bp.tier2;
            } else if index == 3 {
                self.res[2] -= bp.tier3;
            }
            self.production[index] += 1;
        }
    }

    #[derive(Debug)]
    struct Blueprint {
        id: usize,
        ore_costs: [usize; 4],
        tier2: usize,
        tier3: usize,
    }

    impl Blueprint {
        fn buildable(&self, sim: &Sim) -> [bool; 4] {
            std::array::from_fn(|i| {
                if i == 0
                    && sim.production[0]
                        >= self.ore_costs[1]
                            .max(self.ore_costs[2])
                            .max(self.ore_costs[3])
                {
                    false
                } else if i == 1 && sim.production[1] >= self.tier2 {
                    false
                } else if i == 2 && (sim.res[1] < self.tier2 || sim.production[2] >= self.tier3) {
                    false
                } else if i == 3 && sim.res[2] < self.tier3 {
                    false
                } else {
                    sim.res[0] >= self.ore_costs[i]
                }
            })
        }
    }

    fn max_production(
        mut sim: Sim,
        time: usize,
        bp: &Blueprint,
        cache: &mut HashMap<(Sim, usize), usize>,
    ) -> usize {
        let mut best = sim.res[3];
        for t in (0..time).rev() {
            let buildable = bp.buildable(&sim);
            for i in 0..4 {
                sim.res[i] += sim.production[i];
            }

            let key = (sim.clone(), t);
            if let Some(other_best) = cache.get(&key) {
                best = best.max(*other_best);
                continue;
            }

            for i in 0..4 {
                if buildable[i] {
                    let mut choice = sim.clone();
                    choice.build(bp, i);
                    best = best.max(max_production(choice, t, bp, cache));
                }
            }

            best = best.max(sim.res[3]);
            cache.insert(key, best);
        }

        best
    }

    let quality = entries
        .par_iter()
        .map(|bp| {
            let mut sim = Sim::default();
            sim.production[0] = 1;
            let mut cache = HashMap::new();
            let geodes = max_production(sim, 24, &bp, &mut cache);
            bp.id * geodes
        })
        .sum::<usize>();

    println!("Day19a: {}", quality);

    let answer = &entries[0..3]
        .par_iter()
        .map(|bp| {
            let mut sim = Sim::default();
            sim.production[0] = 1;

            println!("{bp:?}");

            let mut cache = HashMap::new();
            let geodes = max_production(sim, 32, &bp, &mut cache);

            println!("{}: {geodes}", bp.id);
            geodes
        })
        .product::<usize>();

    println!("Day19b: {}", answer);
}

#[test]
fn test_it() {
    run();
}
