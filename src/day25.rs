pub fn run() {
    fn snafu(mut n : i64) -> String {
        if n == 0 {
            return "0".into();
        }

        let mut r = String::new();
        let mut rem = 0;
        while n > 0 || rem != 0 {
            let k = (n % 5) + rem;
            n /= 5;
            const TBL: &[u8; 6] = b"012=-1";
            r.push(TBL[k as usize] as char);
            rem = k / 3;
        }

        r.chars().rev().collect::<String>()
    }

    let input = include_str!("../input/day25.txt");
    let fuels = input.lines().map(|line| {
        line.as_bytes().iter().fold(0i64, |acc, ch| {
            let n = match ch {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => unreachable!()
            };
            acc * 5 + n
        })
    }).collect::<Vec<i64>>();

    let answer = snafu(fuels.iter().sum::<i64>());

    println!("Day25: {}", answer);
}

