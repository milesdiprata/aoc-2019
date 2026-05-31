use std::fs;
use std::time::Instant;

use anyhow::Result;

use aoc_2019::intcode::IntcodeCpu;
use aoc_2019::intcode::Run;

pub fn permutations<T: Clone>(nums: &mut [T]) -> Vec<Vec<T>> {
    fn dfs<T: Clone>(nums: &mut [T], start: usize, permutations: &mut Vec<Vec<T>>) {
        if start == nums.len() {
            permutations.push(nums.to_vec());
            return;
        }

        for end in start..nums.len() {
            nums.swap(start, end);
            dfs(nums, start + 1, permutations);
            nums.swap(start, end);
        }
    }

    let mut permutations = Vec::new();
    dfs(nums, 0, &mut permutations);
    permutations
}

fn amplify(program: &[i64], phases: &[i64]) -> i64 {
    let mut signal = 0;

    for &phase in phases {
        let mut cpu = IntcodeCpu::from(program.iter().copied());
        cpu.push(phase);
        cpu.push(signal);

        if let Run::Output(out) = cpu.run() {
            signal = out;
        }
    }

    signal
}

fn feedback(program: &[i64], phases: &[i64]) -> i64 {
    let mut amps = phases
        .iter()
        .map(|&phase| {
            let mut cpu = IntcodeCpu::from(program.iter().copied());
            cpu.push(phase);
            cpu
        })
        .collect::<Vec<_>>();

    let mut signal = 0;
    'feedback: loop {
        for amp in &mut amps {
            amp.push(signal);
            match amp.run() {
                Run::Output(out) => signal = out,
                Run::Halted => break 'feedback,
                Run::NeedsInput => unreachable!("input is pushed before every run"),
            }
        }
    }
    signal
}

fn main() -> Result<()> {
    let program = fs::read_to_string("in/day7.txt")?
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::permutations(&mut [0, 1, 2, 3, 4])
            .into_iter()
            .map(|phases| self::amplify(&program, &phases))
            .max()
            .unwrap();
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 17_440);
    };

    {
        let start = Instant::now();
        let part2 = self::permutations(&mut [5, 6, 7, 8, 9])
            .into_iter()
            .map(|phases| self::feedback(&program, &phases))
            .max()
            .unwrap();
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 27_561_242);
    };

    Ok(())
}
