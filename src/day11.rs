pub fn run() {
    let input = include_str!("../input/day11.txt");

    struct Monkey {
        pub items: Vec<i64>,
        pub multiply: bool,
        pub operand: Option<i64>,
        pub divisor: i64,
        pub branch_true: usize,
        pub branch_false: usize,
    }

    let monkeys: Vec<Monkey> = crate::split_paragraphs(input)
        .filter_map(|para| {
            let mut iter = para.lines();
            iter.next()?;

            let args = [
                "  Starting items: ",
                "  Operation: new = old ",
                "  Test: divisible by ",
                "    If true: throw to monkey ",
                "    If false: throw to monkey ",
            ]
            .map(|prefix| iter.next().unwrap().strip_prefix(prefix).unwrap_or(""));

            let items = args[0]
                .split(", ")
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<i64>>();

            let (op, arg) = args[1].split_once(' ')?;

            let operand = arg.parse::<i64>().ok();
            assert!(operand != None || arg == "old");

            let multiply = match op {
                "*" => true,
                _ => false,
            };

            let divisor = args[2].parse::<i64>().ok()?;
            let branch_true = args[3].parse::<usize>().ok()?;
            let branch_false = args[4].parse::<usize>().ok()?;
            Some(Monkey {
                items,
                multiply,
                operand,
                divisor,
                branch_true,
                branch_false,
            })
        })
        .collect();

    let eval = |rounds, method: &dyn Fn(i64) -> i64| {
        let mut inspections = vec![0usize; monkeys.len()];
        let mut items: Vec<_> = monkeys.iter().map(|x| x.items.clone()).collect();
        for _round in 0..rounds {
            for index in 0..monkeys.len() {
                let [current, branch_true, branch_false] = crate::get_many_mut(
                    &mut items,
                    [
                        index,
                        monkeys[index].branch_true,
                        monkeys[index].branch_false,
                    ],
                )
                .expect("Valid indices");

                let Monkey {
                    multiply,
                    operand,
                    divisor,
                    ..
                } = monkeys[index];

                inspections[index] += current.len();

                for worry in current.drain(..) {
                    let x = method(if multiply {
                        worry * operand.unwrap_or(worry)
                    } else {
                        worry + operand.unwrap_or(worry)
                    });

                    if x % divisor == 0 {
                        branch_true.push(x);
                    } else {
                        branch_false.push(x);
                    }
                }
            }
        }

        let [a, b] = crate::top_descending(&mut inspections);
        *a * *b
    };

    println!("Day11a: {}", eval(20, &|x: i64| x / 3));
    let product = monkeys.iter().map(|x| x.divisor).product::<i64>();
    println!("Day11b: {}", eval(10000, &|x: i64| x % product));
}
