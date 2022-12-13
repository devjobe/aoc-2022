pub fn run() {
    let input = include_str!("../input/day13.txt");

    #[derive(Clone, PartialEq, Eq)]
    enum Packet {
        Value(i32),
        List(Vec<Packet>),
    }

    impl PartialOrd for Packet {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Packet {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            use std::cmp::Ordering::*;

            match (self, other) {
                (Packet::Value(a), Packet::Value(b)) => a.cmp(b),
                (Packet::List(a), Packet::List(b)) => {
                    match a
                        .iter()
                        .zip(b.iter())
                        .find_map(|(left, right)| match left.cmp(right) {
                            Equal => None,
                            ord => Some(ord),
                        })
                        .unwrap_or(Equal)
                    {
                        Equal => a.len().cmp(&b.len()),
                        ord => ord,
                    }
                }
                (Packet::Value(_), Packet::List(b)) => match b.first() {
                    Some(right) => match self.cmp(right) {
                        Equal => 1.cmp(&b.len()),
                        ord => ord,
                    },
                    _ => Greater,
                },
                (Packet::List(a), Packet::Value(_)) => match a.first() {
                    Some(left) => match left.cmp(other) {
                        Equal => a.len().cmp(&1),
                        ord => ord,
                    },
                    _ => Less,
                },
            }
        }
    }

    impl Packet {
        fn new(value: &str) -> Self {
            let bytes = value.as_bytes();

            let mut stack = vec![];
            let mut current = Vec::new();

            let mut iter = bytes.iter().cloned().peekable();

            while let Some(ch) = iter.next() {
                match ch {
                    b'[' => {
                        stack.push(std::mem::replace(&mut current, Vec::new()));
                    }
                    b']' => {
                        let previous = std::mem::replace(&mut current, stack.pop().unwrap());
                        current.push(Packet::List(previous));
                    }
                    b'0'..=b'9' => {
                        let mut value = ch as i32;
                        while let Some(ch) = iter.next_if(|x| (b'0'..=b'9').contains(x)) {
                            value *= 10;
                            value += ch as i32;
                        }
                        current.push(Packet::Value(value));
                    }
                    b',' => continue,
                    _ => continue,
                }
            }

            Packet::List(current)
        }
    }

    let packets = crate::split_paragraphs(input)
        .map(|paragraph| {
            let (a, b) = {
                let mut lines = paragraph.lines();
                (lines.next().unwrap(), lines.next().unwrap())
            };

            (Packet::new(a), Packet::new(b))
        })
        .collect::<Vec<_>>();

    let sum =
        packets.iter().enumerate().fold(
            0usize,
            |sum, (index, (left, right))| {
                if left > right {
                    sum
                } else {
                    sum + index + 1
                }
            },
        );

    println!("Day13a: {}", sum);

    let mut list = packets
        .iter()
        .cloned()
        .map(|v| [v.0, v.1].into_iter())
        .flatten()
        .collect::<Vec<Packet>>();

    list.sort();

    let first = match list.binary_search(&Packet::new("[[2]]")) {
        Ok(n) => n,
        Err(n) => n,
    } + 1;

    let second = match list.binary_search(&Packet::new("[[6]]")) {
        Ok(n) => n,
        Err(n) => n,
    } + 2;

    println!("Day13b: {}", first * second);
}
