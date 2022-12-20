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
        production: [usize; 3],
    }

    impl Sim {
        pub fn build(&mut self, bp: &Blueprint, index: usize, time: usize) {
            self.res[0] -= bp.ore_costs[index];
            if index == 2 {
                self.res[1] -= bp.tier2;
            } else if index == 3 {
                self.res[2] -= bp.tier3;
                self.res[3] += time;
                return;
            }
            self.production[index] += 1;
        }

        pub fn best_case(&self, _bp: &Blueprint, time: usize) -> usize {
            if time > 1 {
                (time * (time - 1)) / 2 + self.res[3]
            } else {
                self.res[3]
            }
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
        fn is_buildable(&self, sim: &Sim, i: usize) -> bool {
            match i  {
                0 => sim.production[0] < self.ore_costs[1]
                    .max(self.ore_costs[2])
                    .max(self.ore_costs[3]),
                1 => sim.production[1] < self.tier2,
                2 => sim.production[2] < self.tier3,
                _ => true,
            }
        }

        fn has_resources(&self, sim: &Sim, i: usize) -> bool {
            if sim.res[0] < self.ore_costs[i] {
                return false;     
            }

            if i == 2 && sim.res[1] < self.tier2 {
                return false;
            }

            if i == 3 && sim.res[2] < self.tier3 {
                return false;
            }
            true
        }
        
        fn build_time(&self, sim: &Sim, i: usize) -> Option<usize> {
            if !self.is_buildable(sim, i) {
                return None;
            }

            if self.has_resources(sim, i) {
                return Some(1);
            }

            let second = if i == 2 {
                if sim.res[1] < self.tier2 {
                    if sim.production[1] == 0 {
                        return None;
                    }
                    ((self.tier2 - sim.res[1]) + sim.production[1] - 1) / sim.production[1]    
                } else {
                    0
                }
            } else if i == 3 {
                if sim.res[2] < self.tier3 {
                    if sim.production[2] == 0 {
                        return None;
                    }
                    ((self.tier3 - sim.res[2]) + sim.production[2] - 1) / sim.production[2]    
                } else {
                    0
                }
            } else {
                0
            };

            let ore_time = if sim.res[0] >= self.ore_costs[i] {
                0
            } else {
                ((self.ore_costs[i] - sim.res[0]) + sim.production[0] - 1) / sim.production[0]
            };
            
            Some(ore_time.max(second) + 1)
        }
    }

    fn max_production(
        sim: Sim,
        time: usize,
        bp: &Blueprint,
        cache: &mut HashMap<(Sim, usize), usize>,
    ) -> usize {
        let key = (sim.clone(), time);
        if let Some(other_best) = cache.get(&key) {
            return *other_best;
        }

        let mut best = sim.res[3];
        for i in (0..4).rev() {
            if let Some(dur) = bp.build_time(&sim, i) {
                if dur > time {
                    continue;
                }
                let mut choice = sim.clone();
                for i in 0..3 {
                    choice.res[i] += sim.production[i] * dur;
                }
                let time_point = time - dur;
                choice.build(bp, i, time_point);
                if choice.best_case(bp, time_point) <= best {
                    continue;
                }
                best = best.max(max_production(choice, time_point, bp, cache));

                if i == 3 && dur <= 1 {
                    break;
                }
            }
        }
        cache.insert(key, best);

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

            let mut cache = HashMap::new();
            let geodes = max_production(sim, 32, &bp, &mut cache);

            geodes
        })
        .product::<usize>();

    println!("Day19b: {}", answer);
}
