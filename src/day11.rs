use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;

use anyhow::bail;
use aoc_2019::intcode::IntcodeCpu;
use aoc_2019::intcode::Run;

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Cw,
    Ccw,
}

#[derive(Clone, Copy, Debug, Default)]
enum Dir {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
}

#[derive(Debug, Default)]
struct Robot {
    pos: (i64, i64),
    dir: Dir,
}

struct Hull {
    panels: HashMap<(i64, i64), Color>,
}

impl From<Color> for i64 {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl TryFrom<i64> for Color {
    type Error = Error;

    fn try_from(color: i64) -> Result<Self> {
        match color {
            0 => Ok(Self::Black),
            1 => Ok(Self::White),
            _ => bail!("unknown color '{color}'"),
        }
    }
}

impl TryFrom<i64> for Rotation {
    type Error = Error;

    fn try_from(rotation: i64) -> Result<Self> {
        match rotation {
            0 => Ok(Self::Ccw),
            1 => Ok(Self::Cw),
            _ => bail!("invalid rotation '{rotation}'"),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => f.write_char(' '),
            Self::White => f.write_char('#'),
        }
    }
}

impl std::fmt::Display for Hull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.panels.keys().map(|&(x, _)| x).min().unwrap();
        let x_max = self.panels.keys().map(|&(x, _)| x).max().unwrap();
        let y_min = self.panels.keys().map(|&(_, y)| y).min().unwrap();
        let y_max = self.panels.keys().map(|&(_, y)| y).max().unwrap();

        for y in y_min..=y_max {
            if y > y_min {
                f.write_char('\n')?;
            }

            for x in x_min..x_max {
                let color = self.panels.get(&(x, y)).copied().unwrap_or(Color::Black);
                f.write_fmt(format_args!("{color}"))?;
            }
        }

        Ok(())
    }
}

impl Dir {
    const fn rotate(self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Cw => self.rotate_cw(),
            Rotation::Ccw => self.rotate_ccw(),
        }
    }

    const fn rotate_cw(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    const fn rotate_ccw(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    const fn delta(self) -> (i64, i64) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

impl Robot {
    const fn step(&mut self, rotation: Rotation) {
        let dir = self.dir.rotate(rotation);
        let (x, y) = self.pos;
        let (dx, dy) = dir.delta();

        self.pos = (x + dx, y + dy);
        self.dir = dir;
    }
}

fn run(cpu: &mut IntcodeCpu, panel: Color) -> Option<(Color, Rotation)> {
    cpu.push(i64::from(panel));

    let paint = match cpu.run() {
        Run::Output(paint) => Some(Color::try_from(paint).unwrap()),
        Run::Halted => None,
        Run::NeedsInput => unreachable!(),
    }?;

    let rotation = match cpu.run() {
        Run::Output(rotation) => Some(Rotation::try_from(rotation).unwrap()),
        Run::Halted => None,
        Run::NeedsInput => todo!(),
    }?;

    Some((paint, rotation))
}

fn paint(program: &[i64], initial: Color) -> Hull {
    let mut cpu = IntcodeCpu::from(program.iter().copied());
    let mut panels = HashMap::new();
    let mut robot = Robot::default();

    while let Some((paint, rotation)) =
        self::run(&mut cpu, panels.get(&robot.pos).copied().unwrap_or(initial))
    {
        panels.insert(robot.pos, paint);
        robot.step(rotation);
    }

    Hull { panels }
}

fn main() -> Result<()> {
    let program = fs::read_to_string("in/day11.txt")?
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::paint(&program, Color::Black).panels.len();
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_876);
    };

    {
        let start = Instant::now();
        let part2 = self::paint(&program, Color::White);
        let elapsed = start.elapsed();
        println!("Part 2 ({elapsed:?}):\n{part2}");
    };

    Ok(())
}
