use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../input/day07.txt");
    let mut path: Vec<&'static str> = Vec::new();

    #[derive(Debug)]
    enum Entry<'a> {
        Dir(&'a str, usize),
        File(&'a str, usize),
    }

    let mut folders = HashMap::new();
    let mut ordered = Vec::new();
    for ls in input.split("$ ls") {
        let (dir, commands) = if let Some((a, b)) = ls.split_once('$') {
            (a, b)
        } else {
            (ls, "")
        };

        let cur = *path.last().unwrap_or(&"/");

        let entries: Vec<_> = dir
            .lines()
            .filter_map(|x| {
                let (sz, name) = x.split_once(' ')?;

                Some(if let Ok(size) = sz.parse() {
                    Entry::File(name, size)
                } else {
                    let p = [cur, name].join("/").leak();
                    Entry::Dir(p, 0)
                })
            })
            .collect();

        if folders.insert(cur, entries).is_none() {
            ordered.push((path.len(), cur));
        }

        for line in commands.lines() {
            let line = line.trim_start_matches(&['$', ' ']);
            let Some((cmd, arg)) = line.split_once(' ') else {
                continue;
            };

            if cmd == "cd" {
                if arg == "/" {
                    path.clear();
                } else if arg == ".." {
                    path.pop();
                } else {
                    let current = *path.last().unwrap_or(&"/");
                    path.push([current, arg].join("/").leak());
                }
            }
        }
    }

    ordered.sort();
    let mut total_sums = HashMap::new();
    let mut sums = 0;
    for &(_, dir) in ordered.iter().rev() {
        let entries = folders.get_mut(dir).unwrap();
        let sum = entries
            .iter_mut()
            .map(|entry| match entry {
                Entry::Dir(child, sz) => {
                    *sz = *total_sums
                        .get(child)
                        .expect(format!("Child folder to exists {child}").as_str());
                    *sz
                }
                Entry::File(_, sz) => *sz,
            })
            .sum::<usize>();

        total_sums.insert(dir, sum);

        if sum <= 100000 {
            sums += sum;
        }
    }

    println!("Day07a: {sums}");

    let remove = total_sums["/"].saturating_sub(70000000 - 30000000);

    let folder_size = total_sums
        .values()
        .cloned()
        .filter(|x| *x >= remove)
        .min()
        .unwrap_or_default();

    println!("Day07b: {folder_size}");
}
