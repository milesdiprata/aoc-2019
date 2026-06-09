use std::fs;
use std::ops::Add;
use std::ops::AddAssign;
use std::str::FromStr;
use std::time::Instant;

use aoc_2019::math;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Clone, Copy, Debug)]
struct Vec3<T>(T, T, T);

#[derive(Clone, Debug)]
struct Moon {
    pos: Vec3<i64>,
    vel: Vec3<i64>,
}

#[derive(Clone, Debug)]
struct Galaxy {
    moons: Vec<Moon>,
}

impl FromStr for Moon {
    type Err = Error;

    fn from_str(moon: &str) -> Result<Self> {
        let [x, y, z] = moon
            .strip_prefix('<')
            .ok_or_else(|| anyhow!("missing '<'"))?
            .strip_suffix('>')
            .ok_or_else(|| anyhow!("missing '>'"))?
            .split(',')
            .map(str::trim)
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("invalid list of coordinates"))?;

        Ok(Self {
            pos: Vec3(
                x.strip_prefix("x=")
                    .ok_or_else(|| anyhow!("missing 'x='"))?
                    .parse()?,
                y.strip_prefix("y=")
                    .ok_or_else(|| anyhow!("missing 'y='"))?
                    .parse()?,
                z.strip_prefix("z=")
                    .ok_or_else(|| anyhow!("missing 'z='"))?
                    .parse()?,
            ),
            vel: Vec3(0, 0, 0),
        })
    }
}

impl FromStr for Galaxy {
    type Err = Error;

    fn from_str(moons: &str) -> Result<Self> {
        Ok(Self {
            moons: moons.lines().map(Moon::from_str).collect::<Result<_>>()?,
        })
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T> Vec3<T> {
    const fn get(&self, axis: usize) -> Option<&T> {
        match axis {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            _ => None,
        }
    }
}

impl Vec3<i64> {
    const fn magnitude(self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl Moon {
    const fn gravity(&self, other: &Self) -> Vec3<i64> {
        const fn delta(axis_i: i64, axis_j: i64) -> i64 {
            (axis_j - axis_i).signum()
        }

        Vec3(
            delta(self.pos.0, other.pos.0),
            delta(self.pos.1, other.pos.1),
            delta(self.pos.2, other.pos.2),
        )
    }

    const fn energy(&self) -> i64 {
        self.pos.magnitude() * self.vel.magnitude()
    }
}

impl Galaxy {
    fn step(&mut self) {
        for i in 0..self.moons.len() {
            for j in (i + 1)..self.moons.len() {
                let di = self.moons[i].gravity(&self.moons[j]);
                let dj = self.moons[j].gravity(&self.moons[i]);

                self.moons[i].vel += di;
                self.moons[j].vel += dj;
            }
        }

        for moon in &mut self.moons {
            moon.pos += moon.vel;
        }
    }

    fn energy(&self) -> i64 {
        self.moons.iter().map(Moon::energy).sum()
    }
}

fn part1(mut galaxy: Galaxy) -> i64 {
    for _ in 0..1_000 {
        galaxy.step();
    }

    galaxy.energy()
}

fn part2(mut galaxy: Galaxy) -> u64 {
    let init = galaxy.clone();

    let mut periods = [None; 3];
    let mut steps = 0;

    while periods.iter().any(Option::is_none) {
        galaxy.step();
        steps += 1;

        for (axis, period) in periods.iter_mut().enumerate() {
            if period.is_none()
                && galaxy.moons.iter().zip(&init.moons).all(|(moon, init)| {
                    moon.pos.get(axis) == init.pos.get(axis)
                        && moon.vel.get(axis) == init.vel.get(axis)
                })
            {
                *period = Some(steps);
            }
        }
    }

    periods.into_iter().map(Option::unwrap).fold(1, math::lcm)
}

fn main() -> Result<()> {
    let galaxy = Galaxy::from_str(&fs::read_to_string("in/day12.txt")?)?;

    {
        let galaxy = galaxy.clone();
        let start = Instant::now();
        let part1 = self::part1(galaxy);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 10_198);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(galaxy);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 271_442_326_847_376);
    };

    Ok(())
}
