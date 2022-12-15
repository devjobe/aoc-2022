use std::{collections::HashSet, ops::RangeInclusive};

pub fn run() {
    let (input, row, search_space) = (include_str!("../input/day15.txt"), 2000000, 4000000);
    struct Report {
        sensor: (i64, i64),
        beacon: (i64, i64),
        dist: i64,
    }

    impl Report {
        fn row_range(&self, row: i64) -> Option<RangeInclusive<i64>> {
            let offset = (self.sensor.1 - row).abs();
            if offset <= self.dist {
                let coverage = self.dist - offset;
                let range = self.sensor.0 - coverage..=self.sensor.0 + coverage;
                Some(range)
            } else {
                None
            }
        }

        fn ysearch(&self, search_space: i64) -> Option<RangeInclusive<i64>> {
            let sense_min = self.sensor.1 - self.dist;
            let sense_max = self.sensor.1 + self.dist;

            if sense_max < 0 || sense_min > search_space {
                None
            } else {
                Some(sense_min.max(0)..=sense_max.min(search_space))
            }
        }

    }

    fn manhatten(a: (i64, i64), b: (i64, i64)) -> i64 {
        (b.0 - a.0).abs() + (b.1 - a.1).abs()
    }

    let reports: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let (s, b) = line.split_once(':')?;

            let (x, y) = s.strip_prefix("Sensor at x=")?.split_once(", y=")?;
            let sensor = (x.parse::<i64>().ok()?, y.parse::<i64>().ok()?);

            let (x, y) = b
                .strip_prefix(" closest beacon is at x=")?
                .split_once(", y=")?;
            let beacon = (x.parse::<i64>().ok()?, y.parse::<i64>().ok()?);
            Some(Report {
                sensor,
                beacon,
                dist: manhatten(sensor, beacon),
            })
        })
        .collect();


    let beacons = reports.iter().map(|r| r.beacon).collect::<HashSet<_>>();

    fn combine_ranges(iter: impl Iterator<Item = RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
        let mut ranges: Vec<RangeInclusive<i64>> = Vec::new();
        for mut range in iter {
            let new_range = range.clone();
            for r in ranges.drain_filter(|r| {
                r.contains(new_range.start())
                    || new_range.contains(r.start())
                    || new_range.contains(r.end())
            }) {
                let start = *range.start().min(r.start());
                let end = *range.end().max(r.end());
                range = start..=end;
            }
            ranges.push(range);
        }
        ranges
    }

    let positions = combine_ranges(reports.iter().filter_map(|r| r.row_range(row)))
        .into_iter()
        .map(|r| {
            r.size_hint().0
                - beacons
                    .iter()
                    .filter(|b| b.1 == row && r.contains(&b.0))
                    .count()
        })
        .sum::<usize>();
    println!("Day15a: {}", positions);

    'found: for (r1, r2) in reports
        .iter()
        .enumerate()
        .map(|(index, r1)| reports.iter().skip(index + 1).map(move |r2| (r1, r2)))
        .flatten()
    {
        let dist = manhatten(r1.sensor, r2.sensor);
        if dist > r1.dist + r2.dist + 1 {
            continue;
        }

        let Some(s1) = r1.ysearch(search_space) else {
            continue;
        };
        let Some(s2) = r2.ysearch(search_space) else {
            continue;
        };

        let overlap = *s1.start().max(s2.start())..=*s1.end().min(s2.end());
        for row in overlap {
            if let [a, b] =
                combine_ranges(reports.iter().filter_map(|r| r.row_range(row))).as_slice()
            {
                let x = if a.end() + 1 == b.start() - 1 {
                    a.end() + 1
                } else if b.end() + 1 == a.start() - 1 {
                    b.end() + 1
                } else {
                    continue;
                };

                println!("Day15b: {}", x * 4000000 + row);
                break 'found;
            }
        }
    }
}
