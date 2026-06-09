use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::ops::Index;
use std::str::FromStr;
use std::time::Instant;

use aoc_2019::math;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
struct Map {
    height: usize,
    width: usize,
    asteroids: Vec<bool>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let height = map.lines().count();
        let width = map
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty map"))?
            .len();

        let asteroids = map
            .lines()
            .flat_map(|line| line.chars().map(|pos| pos == '#'))
            .collect();

        Ok(Self {
            height,
            width,
            asteroids,
        })
    }
}

impl Index<(usize, usize)> for Map {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &bool {
        &self.asteroids[(y * self.width) + x]
    }
}

impl Map {
    fn asteroids(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|&(x, y)| self[(x, y)])
    }
}

fn part1(map: &Map) -> ((usize, usize), usize) {
    let mut station = None;
    let mut best = 0;

    for (x, y) in map.asteroids() {
        let mut lines_of_sight = HashSet::new();

        for (x_target, y_target) in map.asteroids() {
            if (x, y) == (x_target, y_target) {
                continue;
            }

            let (dx, dy) = (
                x.cast_signed() - x_target.cast_signed(),
                y.cast_signed() - y_target.cast_signed(),
            );
            let gcd = math::gcd(dx, dy).abs();

            lines_of_sight.insert((dx / gcd, dy / gcd));
        }

        if lines_of_sight.len() > best {
            station = Some((x, y));
            best = best.max(lines_of_sight.len());
        }
    }

    (station.unwrap(), best)
}

fn part2(map: &Map, (x, y): (usize, usize)) -> usize {
    #[derive(Debug)]
    struct Asteroid {
        coord: (usize, usize),
        dist: usize,
    }

    // Group asteroids by line of sight (vector)
    let mut line_groups = HashMap::new();
    for (x_target, y_target) in map.asteroids() {
        if (x_target, y_target) == (x, y) {
            continue;
        }

        let (dx, dy) = (
            x.cast_signed() - x_target.cast_signed(),
            y.cast_signed() - y_target.cast_signed(),
        );
        let gcd = math::gcd(dx, dy).abs();

        let step = (dx / gcd, dy / gcd);
        let dist = ((dx * dx) + (dy * dy)).cast_unsigned();

        line_groups
            .entry(step)
            .or_insert_with(Vec::new)
            .push(Asteroid {
                coord: (x_target, y_target),
                dist,
            });
    }

    // Sort target asteroids so closest is last
    for group in line_groups.values_mut() {
        group.sort_by_key(|asteroid| Reverse(asteroid.dist));
    }

    // Sort vectors by clockwise angle starting from straight up
    let angle = |(sx, sy): (isize, isize)| {
        #[allow(clippy::cast_precision_loss)]
        let theta = (-sx as f64).atan2(sy as f64);
        if theta.is_sign_negative() {
            theta + std::f64::consts::TAU
        } else {
            theta
        }
    };
    let mut lines_sorted = line_groups.keys().copied().collect::<Vec<_>>();
    lines_sorted.sort_by(|&a, &b| angle(a).total_cmp(&angle(b)));

    // Vaporize in clockwise order, closest asteroids first
    let mut vaporized = 0;
    loop {
        for &line in &lines_sorted {
            if let Some(group) = line_groups.get_mut(&line)
                && !group.is_empty()
            {
                let (x, y) = group.pop().unwrap().coord;
                vaporized += 1;

                if vaporized == 200 {
                    return (100 * x) + y;
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let map = Map::from_str(&fs::read_to_string("in/day10.txt")?)?;

    let start = Instant::now();
    let (station, best) = self::part1(&map);
    let elapsed = start.elapsed();
    println!("Part 1: {best} ({elapsed:?})");
    assert_eq!(best, 267);

    let start = Instant::now();
    let part2 = self::part2(&map, station);
    let elapsed = start.elapsed();
    println!("Part 2: {part2} ({elapsed:?})");
    assert_eq!(part2, 1_309);

    Ok(())
}
