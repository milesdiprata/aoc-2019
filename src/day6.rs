use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
struct OrbitalMap {
    parents: Vec<Option<usize>>,
    orbits: Vec<Vec<usize>>,
    you: usize,
    san: usize,
}

impl FromStr for OrbitalMap {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let mut idxs = HashMap::<&str, usize>::new();
        let mut parents = Vec::new();
        let mut orbits = Vec::new();

        let mut you = None;
        let mut san = None;

        for line in map.lines() {
            let (n1, n2) = line
                .split_once(')')
                .ok_or_else(|| anyhow!("invalid orbit '{line}'"))?;

            for name in [n1, n2] {
                if !idxs.contains_key(name) {
                    let idx = orbits.len();
                    idxs.insert(name, idx);
                    parents.push(None);
                    orbits.push(Vec::new());

                    if name == "YOU" {
                        you = Some(idx);
                    }

                    if name == "SAN" {
                        san = Some(idx);
                    }
                }
            }

            let (a, b) = (idxs[n1], idxs[n2]);
            parents[b] = Some(a);
            orbits[a].push(b);
            orbits[b].push(a);
        }

        Ok(Self {
            parents,
            orbits,
            you: you.ok_or_else(|| anyhow!("missing 'YOU' in map"))?,
            san: san.ok_or_else(|| anyhow!("missing 'SAN' in map"))?,
        })
    }
}

impl OrbitalMap {
    fn find_num_orbits(&self) -> usize {
        let mut orbits = 0;

        for mut node in 0..self.parents.len() {
            while let Some(parent) = self.parents[node] {
                node = parent;
                orbits += 1;
            }
        }

        orbits
    }

    fn find_min_steps(&self) -> usize {
        let mut queue = VecDeque::from([(self.you, 0)]);
        let mut visited = vec![false; self.orbits.len()];

        visited[self.you] = true;

        while let Some((node, dist)) = queue.pop_front() {
            if node == self.san {
                return dist - 2;
            }

            for &next in &self.orbits[node] {
                if !visited[next] {
                    queue.push_back((next, dist + 1));
                    visited[next] = true;
                }
            }
        }

        unreachable!()
    }
}

fn main() -> Result<()> {
    let map = OrbitalMap::from_str(&fs::read_to_string("in/day6.txt")?)?;

    {
        let start = Instant::now();
        let part1 = map.find_num_orbits();
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 254_447);
    };

    {
        let start = Instant::now();
        let part2 = map.find_min_steps();
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 445);
    };

    Ok(())
}
