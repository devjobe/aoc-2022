use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../input/day21.txt");

    enum Monkey {
        Value(i64),
        Add(&'static str, &'static str),
        Sub(&'static str, &'static str),
        Mul(&'static str, &'static str),
        Div(&'static str, &'static str),
    }

    let monkeys = input
        .lines()
        .map(|line| {
            let (var, expr) = line.split_once(": ").unwrap();

            if let Ok(val) = expr.parse::<i64>() {
                (var, Monkey::Value(val))
            } else {
                let (a, op, b) = (&expr[0..4], &expr[5..6], &expr[7..]);
                (
                    var,
                    match op {
                        "+" => Monkey::Add(a, b),
                        "-" => Monkey::Sub(a, b),
                        "*" => Monkey::Mul(a, b),
                        "/" => Monkey::Div(a, b),
                        _ => panic!("unknown op: {op}"),
                    },
                )
            }
        })
        .collect::<HashMap<_, _>>();

    let mut parents = HashMap::new();
    let mut eval = Vec::new();
    let mut state = HashMap::new();
    let mut queue = Vec::new();
    queue.push("root");
    while let Some(root) = queue.pop() {
        eval.push(root);

        let Some(monkey) = monkeys.get(root) else {
            continue;
        };

        let (a, b) = match monkey {
            Monkey::Value(value) => {
                state.insert(root, *value);
                continue;
            }
            Monkey::Add(a, b) => (*a, *b),
            Monkey::Sub(a, b) => (*a, *b),
            Monkey::Mul(a, b) => (*a, *b),
            Monkey::Div(a, b) => (*a, *b),
        };

        parents.insert(a, root);
        parents.insert(b, root);

        queue.push(b);
        queue.push(a);
    }

    for &root in eval.iter().rev() {
        let Some(monkey) = monkeys.get(root) else {
            continue;
        };
        let (a, b) = match monkey {
            Monkey::Value(_value) => {
                continue;
            }
            Monkey::Add(a, b) => (a, b),
            Monkey::Sub(a, b) => (a, b),
            Monkey::Mul(a, b) => (a, b),
            Monkey::Div(a, b) => (a, b),
        };

        let Some(&x) = state.get(a) else {
            continue;
        };

        let Some(&y) = state.get(b) else {
            continue;
        };

        let r = match monkey {
            Monkey::Value(_) => continue,
            Monkey::Add(_, _) => x + y,
            Monkey::Sub(_, _) => x - y,
            Monkey::Mul(_, _) => x * y,
            Monkey::Div(_, _) => x / y,
        };

        state.insert(root, r);
    }

    println!("Day21a: {}", state["root"]);

    let mut current = "humn";
    while let Some(&root) = parents.get(current) {
        queue.push(current);
        current = root;
    }

    let mut x1 = 0;
    let mut root = "root";
    while let Some(current) = queue.pop() {
        let Some(monkey) = monkeys.get(root) else {
            continue;
        };
        let (a, b) = match monkey {
            Monkey::Value(_value) => {
                continue;
            }
            Monkey::Add(a, b) => (*a, *b),
            Monkey::Sub(a, b) => (*a, *b),
            Monkey::Mul(a, b) => (*a, *b),
            Monkey::Div(a, b) => (*a, *b),
        };

        if a != current {
            let y = *state.get(a).unwrap();
            if root == "root" {
                x1 = y;
            } else {
                let x0 = x1;
                x1 = match monkey {
                    Monkey::Value(_) => panic!("expected expr"),
                    Monkey::Add(_, _) => x0 - y,
                    Monkey::Sub(_, _) => y - x0,
                    Monkey::Mul(_, _) => x0 / y,
                    Monkey::Div(_, _) => y / x0,
                };
            }
        } else {
            let y = *state.get(b).unwrap();
            if root == "root" {
                x1 = y;
            } else {
                let x0 = x1;
                x1 = match monkey {
                    Monkey::Value(_) => panic!("expected expr"),
                    Monkey::Add(_, _) => x0 - y,
                    Monkey::Sub(_, _) => x0 + y,
                    Monkey::Mul(_, _) => x0 / y,
                    Monkey::Div(_, _) => x0 * y,
                };
            }
        };

        root = current;
    }

    println!("Day21b: {x1}");
}
