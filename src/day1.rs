use std::fs;
use std::iter;
use std::time::Instant;

use anyhow::Result;

const fn fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn part1(masses: &[i64]) -> i64 {
    masses.iter().map(|&mass| self::fuel(mass)).sum()
}

fn part2(masses: &[i64]) -> i64 {
    masses
        .iter()
        .flat_map(|&mass| {
            iter::successors(Some(fuel(mass).max(0)), |&mass| {
                let fuel = fuel(mass);
                (fuel > 0).then_some(fuel)
            })
        })
        .sum()
}

fn main() -> Result<()> {
    let masses = fs::read_to_string("in/day1.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&masses);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 3_239_890);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&masses);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 4_856_963);
    };

    Ok(())
}
