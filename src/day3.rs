use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Debug)]
struct Wire {
    coords: Vec<(i32, i32)>,
}

impl FromStr for Wire {
    type Err = Error;

    fn from_str(path: &str) -> Result<Self> {
        let mut coords = Vec::new();
        let (mut x, mut y) = (0, 0);

        for step in path.split(',') {
            if let Some(units) = step.strip_prefix('U') {
                let units = units.parse::<i32>()?;
                coords.extend((1..=units).map(|dy| (x, y + dy)));
                y += units;
            } else if let Some(units) = step.strip_prefix('D') {
                let units = units.parse::<i32>()?;
                coords.extend((1..=units).map(|dy| (x, y - dy)));
                y -= units;
            } else if let Some(units) = step.strip_prefix('L') {
                let units = units.parse::<i32>()?;
                coords.extend((1..=units).map(|dx| (x - dx, y)));
                x -= units;
            } else if let Some(units) = step.strip_prefix('R') {
                let units = units.parse::<i32>()?;
                coords.extend((1..=units).map(|dx| (x + dx, y)));
                x += units;
            } else {
                bail!("invalid step '{step}'");
            }
        }

        Ok(Self { coords })
    }
}

impl Wire {
    fn steps(&self) -> HashMap<(i32, i32), usize> {
        let mut steps = HashMap::new();
        for (i, &coord) in self.coords.iter().enumerate() {
            match steps.entry(coord) {
                Entry::Occupied(_) => {}
                Entry::Vacant(vacant) => {
                    vacant.insert(i + 1);
                }
            }
        }
        steps
    }
}

fn parse() -> Result<(Wire, Wire)> {
    let input = fs::read_to_string("in/day3.txt")?;
    let (w1, w2) = input
        .split_once('\n')
        .ok_or_else(|| anyhow!("invalid input"))?;

    Ok((w1.parse()?, w2.parse()?))
}

fn part1(w1: &Wire, w2: &Wire) -> i32 {
    let mut dist = i32::MAX;
    let w1_coords = w1.coords.iter().copied().collect::<HashSet<_>>();

    for &(x, y) in &w2.coords {
        if w1_coords.contains(&(x, y)) {
            dist = dist.min(x.abs() + y.abs());
        }
    }

    dist
}

fn part2(w1: &Wire, w2: &Wire) -> usize {
    let mut steps = usize::MAX;

    let w1 = w1.steps();
    let w2 = w2.steps();

    for ((x, y), w2_steps) in w2 {
        if let Some(&w1_steps) = w1.get(&(x, y)) {
            steps = steps.min(w1_steps + w2_steps);
        }
    }

    steps
}

fn main() -> Result<()> {
    let (w1, w2) = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&w1, &w2);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 2_193);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&w1, &w2);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 63_526);
    };

    Ok(())
}
