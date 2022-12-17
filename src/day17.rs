use std::collections::VecDeque;

pub fn run() {
    let input = include_str!("../input/day17.txt");
    
    struct Rock<'a> {
        shape: &'a [u8],
        width: usize,
    }

    const ROCKS: [Rock; 5] = [
        Rock {
            shape: &[0b1111],
            width: 4,
        },
        Rock {
            shape: &[0b010, 0b111, 0b010],
            width: 3,
        },
        Rock {
            shape: &[0b111, 0b100, 0b100],
            width: 3,
        },
        Rock {
            shape: &[1, 1, 1, 1],
            width: 1,
        },
        Rock {
            shape: &[3, 3],
            width: 2,
        },
    ];

    struct Simulation<'a> {
        tower: VecDeque<u8>,
        push: &'a [u8],
        step: usize,
        cycle: usize,
        rock: u64,
        removed: u64,
    }

    impl<'a> Simulation<'a> {
        pub fn step(&mut self) {
            let Simulation {
                tower,
                push,
                rock,
                step,
                cycle,
                removed,
                ..
            } = self;

            let n_rock = ((*rock) % ROCKS.len() as u64) as usize;
            *rock += 1;
            let &Rock { shape, width } = &ROCKS[n_rock];

            let mut offset = 2;
            let mut y = tower.len() + 3;

            'stopped: loop {
                let new_offset = if push[*step] == b'>' {
                    (offset + 1).min(7 - width)
                } else {
                    offset.saturating_sub(1)
                };

                *step += 1;

                if *step >= push.len() {
                    *step = 0;
                    *cycle += 1;
                }

                'blocked: {
                    for (b, y) in shape.iter().zip(y..tower.len()) {
                        let mask = *b << new_offset;
                        if (tower[y] & mask) != 0 {
                            break 'blocked;
                        }
                    }
                    offset = new_offset;
                }

                if y == 0 {
                    break;
                }

                for (b, y) in shape.iter().zip((y - 1)..tower.len()) {
                    let mask = *b << offset;
                    if (tower[y] & mask) != 0 {
                        break 'stopped;
                    }
                }
                y = y - 1;
            }

            let new_len = y + shape.len();
            if new_len > tower.len() {
                tower.resize(new_len, 0);
            }

            for (row, b) in tower.iter_mut().skip(y).zip(shape.iter()) {
                let mask = b << offset;
                assert!((*row & mask) == 0);
                *row |= mask;
            }

            while tower.len() > 40 {
                tower.pop_front();
                *removed += 1;
            }
        }

        fn height(&self) -> u64 {
            self.tower.len() as u64 + self.removed
        }

        fn cycle(&mut self) -> (u64, u64) {
            let cycle = self.cycle;
            while self.cycle < 2 || cycle == self.cycle {
                self.step();
            }

            let h0 = self.height();
            let r0 = self.rock;
            let s0 = self.step;

            loop {
                self.step();
                if s0 == self.step {
                    break;
                }
            }
            let h1 = self.height();
            let r1 = self.rock;

            (h1 - h0, r1 - r0)
        }

        fn seek(&mut self, n: u64) -> u64 {
            if n > 2022 {
                let (steps, rocks) = self.cycle();
                let k = (n - self.rock) / rocks;
                self.removed += steps * k;
                self.rock += rocks * k;
            }

            while self.rock < n {
                self.step();
            }

            self.height()
        }
    }

    let mut sim = Simulation {
        tower: Default::default(),
        push: input.as_bytes(),
        step: Default::default(),
        rock: Default::default(),
        cycle: Default::default(),
        removed: Default::default(),
    };

    println!("Day17a: {}", sim.seek(2022));
    println!("Day17b: {}", sim.seek(1_000_000_000_000));
}
