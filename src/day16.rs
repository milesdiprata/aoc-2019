use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

const BASE_PATTERN: &[i32] = &[0, 1, 0, -1];

#[derive(Clone, Debug)]
struct Signal {
    nums: Vec<i32>,
}

impl FromStr for Signal {
    type Err = Error;

    fn from_str(signal: &str) -> Result<Self> {
        Ok(Self {
            nums: signal
                .chars()
                .map(|c| c.to_digit(10).ok_or_else(|| anyhow!("invalid digit '{c}'")))
                .map(|digit| digit.map(u32::cast_signed))
                .collect::<Result<_>>()?,
        })
    }
}

impl Signal {
    fn apply_phase(&self) -> Self {
        let mut out = Vec::with_capacity(self.nums.len());

        for i in 0..self.nums.len() {
            let repeats = i + 1;
            let mut pattern = BASE_PATTERN
                .iter()
                .flat_map(|&p| (0..repeats).map(move |_| p))
                .cycle()
                .skip(1);

            let mut new = 0;
            for &n in &self.nums {
                let p = pattern.next().unwrap();
                new += n * p;
            }

            out.push(new.abs() % 10);
        }

        Self { nums: out }
    }

    fn read_digits(&self, len: usize) -> u64 {
        #[allow(clippy::cast_sign_loss)]
        self.nums
            .iter()
            .take(len)
            .fold(0, |acc, &n| (10 * acc) + n as u64)
    }
}

fn part1(signal: &Signal) -> u64 {
    let mut signal = signal.clone();
    for _ in 0..100 {
        signal = signal.apply_phase();
    }

    signal.read_digits(8)
}

fn part2(signal: &Signal) -> u64 {
    #[allow(clippy::cast_possible_truncation)]
    let offset = signal.read_digits(7) as usize;
    let len = 10_000 * signal.nums.len();

    assert!(offset >= len / 2, "Offset must be in the second half");

    let mut signal = Signal {
        nums: signal
            .nums
            .iter()
            .copied()
            .cycle()
            .skip(offset)
            .take(len - offset)
            .collect(),
    };

    for _ in 0..100 {
        let mut sum = 0;
        for n in signal.nums.iter_mut().rev() {
            sum = (sum + *n) % 10;
            *n = sum;
        }
    }

    signal.read_digits(8)
}

fn main() -> Result<()> {
    let signal = Signal::from_str(&fs::read_to_string("in/day16.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&signal);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 44_098_263);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&signal);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 12_482_168);
    };

    Ok(())
}
