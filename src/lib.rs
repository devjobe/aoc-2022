#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]
#![feature(array_windows)]
#![feature(string_leak)]
#![feature(map_many_mut)]
#![feature(drain_filter)]
#![feature(map_try_insert)]

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;

struct SplitParagraphs<'a> {
    input: &'a str,
}

impl<'a> Iterator for SplitParagraphs<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let bytes = self.input.as_bytes();
        let offset = if bytes.len() <= 4 {
            match bytes {
                [_, b'\n', b'\n', ..] => 1,
                [_, b'\r', b'\n', b'\n', ..] => 1,
                [_, b'\n', b'\r', b'\n', ..] => 1,
                [_, _, b'\n', b'\n', ..] => 2,
                _ => {
                    return Some(std::mem::replace(&mut self.input, ""));
                }
            }
        } else {
            let Some(offset) = bytes.array_windows().position(|[a, b, c, d]| {
                match (*a, *b, *c, *d) {
                    (b'\n', b'\n', _, _) => true,
                    (b'\n', b'\r', b'\n', _) => true,
                    (b'\r', b'\n', b'\n', _) => true,
                    (b'\r', b'\n', b'\r', b'\n') => true,
                    _ => false
                }
            }) else {
                return Some(std::mem::replace(&mut self.input, ""));
            };
            offset
        };

        let result = unsafe { std::str::from_utf8_unchecked(&bytes[..offset]) };
        self.input = unsafe {
            std::str::from_utf8_unchecked(&bytes[offset..]).trim_start_matches(&['\r', '\n'])
        };
        Some(result)
    }
}

fn split_paragraphs(input: &str) -> SplitParagraphs<'_> {
    SplitParagraphs {
        input: input.trim_matches(|c| c == '\r' || c == '\n'),
    }
}

fn get_many_mut<const N: usize, T>(slice: &mut [T], indices: [usize; N]) -> Option<[&mut T; N]> {
    let len = slice.len();
    for (n, x) in indices.iter().enumerate() {
        if indices[..n].contains(x) || *x >= len {
            return None;
        }
    }

    unsafe {
        let ptrs = indices.map(|i| slice.get_unchecked_mut(i) as *mut T);
        Some(std::mem::transmute_copy(&ptrs))
    }
}

fn top_descending<const N: usize, T: Ord>(slice: &mut [T]) -> &mut [T; N] {
    slice.select_nth_unstable_by(N, |a, b| b.cmp(a));
    unsafe {
        let ptrs: [*mut T; N] = std::array::from_fn(|i| slice.get_unchecked_mut(i) as *mut T);
        std::mem::transmute_copy(&ptrs)
    }
}
