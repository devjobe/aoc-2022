pub fn run() {
    let input = include_str!("../input/day08.txt");

    let map: Vec<Vec<i8>> = input
        .lines()
        .map(|line| line.chars().map(|ch| (ch as i8) - '0' as i8).collect())
        .collect();

    fn fold_acc_tree(
        (scenic_max, vis_count): (usize, usize),
        (scenic, vis): (usize, usize),
    ) -> (usize, usize) {
        (scenic_max.max(scenic), vis_count + vis)
    }

    let (scenic_max, visible) = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, height)| {
                    let left = row
                        .iter()
                        .take(x)
                        .rev()
                        .position(|n| n >= height)
                        .map(|idx| (idx + 1, false))
                        .unwrap_or((x, true));

                    let right = row
                        .iter()
                        .skip(x + 1)
                        .position(|n| n >= height)
                        .map(|idx| (idx + 1, false))
                        .unwrap_or((row.len() - (x + 1), true));

                    let column = map.iter().map(|row| row[x]);
                    let up = column
                        .clone()
                        .take(y)
                        .rev()
                        .position(|n| n >= *height)
                        .map(|idx| (idx + 1, false))
                        .unwrap_or((y, true));

                    let down = column
                        .skip(y + 1)
                        .position(|n| n >= *height)
                        .map(|idx| (idx + 1, false))
                        .unwrap_or((map.len() - (y + 1), true));

                    let (scenic, visible) = [left, right, up, down]
                        .iter()
                        .fold((1usize, false), |acc, x| (acc.0 * x.0, acc.1 | x.1));
                    (scenic, visible as usize)
                })
                .fold((0usize, 0usize), fold_acc_tree)
        })
        .fold((0usize, 0usize), fold_acc_tree);

    println!("Day08a: {visible}");
    println!("Day08b: {scenic_max}");
}
