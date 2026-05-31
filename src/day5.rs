use std::fs;
use std::time::Instant;

use anyhow::Result;

use aoc_2019::intcode::IntcodeCpu;
use aoc_2019::intcode::Run;

fn diagnostic(program: &[i64], input: i64) -> i64 {
    let mut cpu = IntcodeCpu::from(program.iter().copied());
    cpu.push(input);

    let mut last = 0;
    while let Run::Output(out) = cpu.run() {
        assert!(last == 0, "non-zero diagnostic code before final output");
        last = out;
    }
    last
}

fn main() -> Result<()> {
    let program = fs::read_to_string("in/day5.txt")?
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::diagnostic(&program, 1);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 15_314_507);
    };

    {
        let start = Instant::now();
        let part2 = self::diagnostic(&program, 5);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 652_726);
    };

    Ok(())
}
