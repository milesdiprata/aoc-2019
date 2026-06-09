use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::time::Instant;

use aoc_2019::intcode::IntcodeCpu;
use aoc_2019::intcode::Run;

use anyhow::Result;
use anyhow::anyhow;
use strum::FromRepr;

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i64)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Debug)]
enum Output {
    Tile { x: i64, y: i64, ty: TileType },
    Score(i64),
}

#[derive(Debug)]
struct Game {
    outputs: Vec<Output>,
}

impl std::fmt::Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => ' ',
            Self::Wall => '#',
            Self::Block => '*',
            Self::Paddle => '—',
            Self::Ball => 'o',
        };

        f.write_char(c)
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tiles = self.tiles().collect::<HashMap<_, _>>();
        let x_min = tiles.keys().map(|&(x, _)| x).min().unwrap();
        let x_max = tiles.keys().map(|&(x, _)| x).max().unwrap();
        let y_min = tiles.keys().map(|&(_, y)| y).min().unwrap();
        let y_max = tiles.keys().map(|&(_, y)| y).max().unwrap();

        for y in y_min..=y_max {
            if y > y_min {
                f.write_char('\n')?;
            }

            for x in x_min..=x_max {
                let tile = tiles.get(&(x, y)).copied().unwrap_or(TileType::Empty);
                f.write_fmt(format_args!("{tile}"))?;
            }
        }

        Ok(())
    }
}

impl Output {
    fn from_outputs(outputs: [i64; 3]) -> Result<Self> {
        let x = outputs[0];
        let y = outputs[1];

        if (x, y) == (-1, 0) {
            Ok(Self::Score(outputs[2]))
        } else {
            Ok(Self::Tile {
                x: outputs[0],
                y: outputs[1],
                ty: TileType::from_repr(outputs[2])
                    .ok_or_else(|| anyhow!("invalid tile type '{}'", outputs[2]))?,
            })
        }
    }
}

impl Game {
    fn from_outputs(outputs: &[i64]) -> Result<Self> {
        Ok(Self {
            outputs: outputs
                .chunks(3)
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(Output::from_outputs)
                .collect::<Result<_>>()?,
        })
    }

    fn tiles(&self) -> impl Iterator<Item = ((i64, i64), TileType)> {
        self.outputs.iter().filter_map(|output| match output {
            &Output::Tile { x, y, ty } => Some(((x, y), ty)),
            Output::Score(_) => None,
        })
    }
}

fn part1(program: &[i64]) -> usize {
    let mut cpu = IntcodeCpu::from(program.iter().copied());
    let mut outputs = Vec::new();

    loop {
        match cpu.run() {
            Run::Output(output) => outputs.push(output),
            Run::Halted => break,
            Run::NeedsInput => unreachable!(),
        }
    }

    let game = Game::from_outputs(&outputs).unwrap();
    println!("{game}");

    game.tiles()
        .filter(|&(_, ty)| matches!(ty, TileType::Block))
        .count()
}

fn part2(program: &[i64]) -> i64 {
    let mut cpu = IntcodeCpu::from(program.iter().copied());
    let mut score = 0;
    let mut x_ball = 0;
    let mut x_paddle = 0;
    let mut buf = Vec::with_capacity(3);

    cpu.set(0, 2);

    loop {
        match cpu.run() {
            Run::Output(val) => {
                buf.push(val);
                if let Ok(triple) = <[_; 3]>::try_from(buf.as_slice()) {
                    match Output::from_outputs(triple).unwrap() {
                        Output::Tile {
                            x,
                            ty: TileType::Ball,
                            ..
                        } => x_ball = x,
                        Output::Tile {
                            x,
                            ty: TileType::Paddle,
                            ..
                        } => x_paddle = x,
                        Output::Tile { .. } => {}
                        Output::Score(s) => score = s,
                    }
                    buf.clear();
                }
            }
            Run::NeedsInput => cpu.push((x_ball - x_paddle).signum()),
            Run::Halted => break,
        }
    }

    score
}

fn main() -> Result<()> {
    let program = fs::read_to_string("in/day13.txt")?
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&program);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 348);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&program);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 16_999);
    };

    Ok(())
}
