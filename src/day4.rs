use std::fs;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::RangeInclusive;
use std::time::Instant;

use anyhow::Result;
use anyhow::anyhow;

#[derive(Clone, Copy)]
struct Num {
    digits: [u8; Self::LEN],
}

impl Index<usize> for Num {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.digits[index]
    }
}

impl IndexMut<usize> for Num {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.digits[index]
    }
}

impl From<Num> for u32 {
    fn from(num: Num) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        num.digits.iter().enumerate().fold(0, |num, (i, &digit)| {
            num + (10_u32.pow((Num::LEN - 1 - i) as Self) * Self::from(digit))
        })
    }
}

impl Num {
    const LEN: usize = 6;

    const fn new() -> Self {
        Self {
            digits: [1; Self::LEN],
        }
    }

    fn is_in_range(self, range: &RangeInclusive<u32>) -> bool {
        range.contains(&self.into())
    }

    fn has_adj_pair(self) -> bool {
        self.digits.windows(2).any(|window| window[0] == window[1])
    }

    fn has_run_of_exactly_two(self) -> bool {
        self.digits
            .chunk_by(|&a, &b| a == b)
            .any(|chunk| chunk.len() == 2)
    }
}

fn parse() -> Result<RangeInclusive<u32>> {
    let input = fs::read_to_string("in/day4.txt")?;
    let (lo, hi) = input
        .split_once('-')
        .ok_or_else(|| anyhow!("invalid input '{input}'"))?;
    Ok(lo.parse()?..=hi.parse()?)
}

fn solve(range: &RangeInclusive<u32>) -> (usize, usize) {
    fn generate(
        mut num: Num,
        idx: usize,
        min: u8,
        range: &RangeInclusive<u32>,
        part1: &mut usize,
        part2: &mut usize,
    ) {
        if idx == Num::LEN {
            if !num.is_in_range(range) {
                return;
            }

            if num.has_adj_pair() {
                *part1 += 1;
            }

            if num.has_run_of_exactly_two() {
                *part2 += 1;
            }
        } else {
            for digit in min..=9 {
                num[idx] = digit;
                generate(num, idx + 1, digit, range, part1, part2);
            }
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;
    generate(Num::new(), 0, 0, range, &mut part1, &mut part2);
    (part1, part2)
}

fn main() -> Result<()> {
    let range = self::parse()?;

    let start = Instant::now();
    let (part1, part2) = self::solve(&range);
    let elapsed = start.elapsed();

    println!("Part 1: {part1} ({elapsed:?})");
    assert_eq!(part1, 1_716);

    println!("Part 2: {part2} ({elapsed:?})");
    assert_eq!(part2, 1_163);

    Ok(())
}
