use std::fs;
use std::time::Instant;

use aoc_2019::intcode::IntcodeCpu;
use aoc_2019::intcode::Run;

use anyhow::Result;

fn boost_code(program: &[i64], input: i64) -> i64 {
    let mut cpu = IntcodeCpu::from(program.iter().copied());
    cpu.push(input);

    let Run::Output(boost) = cpu.run() else {
        unreachable!();
    };

    boost
}

fn main() -> Result<()> {
    let program = fs::read_to_string("in/day9.txt")?
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::boost_code(&program, 1);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 3_241_900_951);
    };

    {
        let start = Instant::now();
        let part2 = self::boost_code(&program, 2);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 83_089);
    };

    Ok(())
}
