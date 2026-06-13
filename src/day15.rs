use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

use aoc_2019::intcode::IntcodeCpu;
use aoc_2019::intcode::Run;

use anyhow::Result;
use strum::FromRepr;

#[derive(Clone, Copy, Debug)]
#[repr(i64)]
enum Movement {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i64)]
enum Status {
    WallHit = 0,
    Moved = 1,
    FoundOxygen = 2,
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty,
    Wall,
    Oxygen,
}

#[derive(Debug)]
struct Droid {
    cpu: IntcodeCpu,
    x: i64,
    y: i64,
    cells: HashMap<(i64, i64), Cell>,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Oxygen => 'O',
        };

        write!(f, "{c}")
    }
}

impl std::fmt::Display for Droid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.cells.keys().map(|&(x, _)| x).min().unwrap_or_default() - 1;
        let x_max = self.cells.keys().map(|&(x, _)| x).max().unwrap_or_default() + 1;
        let y_min = self.cells.keys().map(|&(_, y)| y).min().unwrap_or_default() - 1;
        let y_max = self.cells.keys().map(|&(_, y)| y).max().unwrap_or_default() + 1;

        let width = (x_min..=x_max).count();
        let border = "-".repeat(width);

        writeln!(f, "+{border}+")?;

        for y in (y_min..=y_max).rev() {
            write!(f, "|")?;

            for x in x_min..=x_max {
                if (x, y) == (self.x, self.y) {
                    write!(f, "D")?;
                } else if (x, y) == (0, 0) {
                    write!(f, "S")?;
                } else if let Some(cell) = self.cells.get(&(x, y)) {
                    write!(f, "{cell}")?;
                } else {
                    write!(f, " ")?;
                }
            }

            writeln!(f, "|")?;
        }

        write!(f, "+{border}+")
    }
}

impl From<IntcodeCpu> for Droid {
    fn from(cpu: IntcodeCpu) -> Self {
        Self {
            cpu,
            x: 0,
            y: 0,
            cells: HashMap::from([((0, 0), Cell::Empty)]),
        }
    }
}

impl Movement {
    const ALL: [Self; 4] = [Self::North, Self::South, Self::West, Self::East];

    const fn delta(self) -> (i64, i64) {
        match self {
            Self::North => (0, 1),
            Self::South => (0, -1),
            Self::West => (-1, 0),
            Self::East => (1, 0),
        }
    }

    const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }
}

impl Droid {
    fn explore(&mut self) -> (i64, i64) {
        let mut oxygen = (self.x, self.y);
        let mut path = Vec::new();

        loop {
            let unexplored = Movement::ALL.iter().copied().find(|&dir| {
                let (dx, dy) = dir.delta();
                !self.cells.contains_key(&(self.x + dx, self.y + dy))
            });

            match unexplored {
                Some(dir) => match self.run(dir) {
                    Status::WallHit => {}
                    status => {
                        if matches!(status, Status::FoundOxygen) {
                            oxygen = (self.x, self.y);
                        }

                        path.push(dir);
                    }
                },
                None => match path.pop() {
                    Some(dir) => {
                        self.run(dir.opposite());
                    }
                    None => break,
                },
            }
        }

        oxygen
    }

    fn run(&mut self, dir: Movement) -> Status {
        let status = loop {
            match self.cpu.run() {
                Run::Output(status) => break Status::from_repr(status).unwrap(),
                Run::NeedsInput => self.cpu.push(dir as i64),
                Run::Halted => unreachable!(),
            }
        };

        let (dx, dy) = dir.delta();
        let adj = (self.x + dx, self.y + dy);

        match status {
            Status::WallHit => self.cells.insert(adj, Cell::Wall),
            Status::Moved => self.cells.insert(adj, Cell::Empty),
            Status::FoundOxygen => self.cells.insert(adj, Cell::Oxygen),
        };

        if matches!(status, Status::Moved | Status::FoundOxygen) {
            (self.x, self.y) = adj;
        }

        status
    }
}

fn bfs(cells: &HashMap<(i64, i64), Cell>, start: (i64, i64)) -> HashMap<(i64, i64), usize> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::from([start]);
    let mut dists = HashMap::new();

    while let Some(((x, y), dist)) = queue.pop_front() {
        dists.insert((x, y), dist);

        for dir in Movement::ALL {
            let (dx, dy) = dir.delta();
            let (x, y) = (x + dx, y + dy);

            if cells
                .get(&(x, y))
                .is_some_and(|&cell| matches!(cell, Cell::Empty | Cell::Oxygen))
                && visited.insert((x, y))
            {
                queue.push_back(((x, y), dist + 1));
            }
        }
    }

    dists
}

fn part1(program: &[i64]) -> usize {
    let mut droid = Droid::from(IntcodeCpu::from(program.iter().copied()));
    let oxygen = droid.explore();
    let dists = self::bfs(&droid.cells, (0, 0));
    dists[&oxygen]
}

fn part2(program: &[i64]) -> usize {
    let mut droid = Droid::from(IntcodeCpu::from(program.iter().copied()));
    let oxygen = droid.explore();
    let dists = self::bfs(&droid.cells, oxygen);
    dists.values().max().copied().unwrap()
}

fn main() -> Result<()> {
    let program = fs::read_to_string("in/day15.txt")?
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&program);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 226);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&program);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 0);
    };

    Ok(())
}
