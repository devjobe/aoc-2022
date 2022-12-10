pub fn run() {
    let input = include_str!("../input/day10.txt");

    #[derive(Clone, Copy)]
    enum Ins {
        Noop,
        Add(i32),
    }

    let program = input
        .lines()
        .map(|line| {
            let (cmd, arg) = line.split_once(' ').unwrap_or((line, ""));

            match cmd {
                "noop" => Ins::Noop,
                "addx" => Ins::Add(arg.parse().unwrap()),
                _ => panic!("Unknown instruction {}", cmd),
            }
        })
        .collect::<Vec<_>>();

    {
        let mut x = 1;
        let mut pc = program.iter();    
        let mut ins = None;
        let mut signal_strength = 0;
        let mut cycle = 1;
        let execute = |n| {
            let mut last_signal = 0;
            for c in cycle..cycle+n {
                last_signal = c * x;
                match ins {
                    Some(Ins::Add(n)) => {           
                        x += n;
                        ins = None;
                        continue;
                    }
                    _ => (),
                }
                ins = pc.next().cloned();
            }
            signal_strength += last_signal;
            cycle += n;
        };

        [20, 40, 40, 40, 40, 40].into_iter().for_each(execute);
        println!("Day10a: {signal_strength}");
    }

    {
        let mut x: i32 = 1;
        let mut pc = program.iter();    
        let mut ins = None;       
        
        println!("Day10b: ");
        for cycle in 0..240 {
            
            let column = cycle % 40;
            if (column - x).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            if column == 39 {
                println!("");
            }

            if match ins {
                Some(Ins::Add(n)) => {           
                    x += n;
                    ins = None;
                    false
                }
                _ => true,
            } {
                ins = pc.next().cloned();
            }
                        
        }


    }

}
