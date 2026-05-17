use std::fs;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::bail;

#[derive(Clone, Copy)]
enum Opcode {
    Add,
    Mul,
    Halt,
}

impl TryFrom<i64> for Opcode {
    type Error = Error;

    fn try_from(opcode: i64) -> Result<Self> {
        match opcode {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            99 => Ok(Self::Halt),
            _ => bail!("invalid opcode '{opcode}'"),
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn run(mem: &mut [i64]) -> i64 {
    let mut idx = 0;

    loop {
        let opcode = Opcode::try_from(mem[idx]).unwrap();
        if matches!(opcode, Opcode::Halt) {
            break;
        }

        let a = mem[mem[idx + 1] as usize];
        let b = mem[mem[idx + 2] as usize];
        let dst = mem[idx + 3] as usize;

        mem[dst] = match opcode {
            Opcode::Add => a + b,
            Opcode::Mul => a * b,
            Opcode::Halt => unreachable!(),
        };

        idx += 4;
    }

    mem[0]
}

fn part1(program: &[i64]) -> i64 {
    let mut mem = program.to_vec();
    mem[1] = 12;
    mem[2] = 2;
    self::run(&mut mem)
}

fn part2(program: &[i64]) -> i64 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut mem = program.to_vec();
            mem[1] = noun;
            mem[2] = verb;
            if self::run(&mut mem) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

fn main() -> Result<()> {
    let program = fs::read_to_string("in/day2.txt")?
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&program);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 4_570_637);
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
