use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Chemical {
    quantity: usize,
    name: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

struct Recipe<'a> {
    batch: usize,
    ingredients: &'a [Chemical],
}

impl FromStr for Chemical {
    type Err = Error;

    fn from_str(chemical: &str) -> Result<Self> {
        let (quantity, name) = chemical
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid chemical '{chemical}'"))?;

        Ok(Self {
            quantity: quantity.parse()?,
            name: name.to_string(),
        })
    }
}

impl FromStr for Reaction {
    type Err = Error;

    fn from_str(reaction: &str) -> Result<Self> {
        let (inputs, output) = reaction
            .split_once(" => ")
            .ok_or_else(|| anyhow!("invalid reaction '{reaction}'"))?;

        Ok(Self {
            inputs: inputs.split(", ").map(str::parse).collect::<Result<_>>()?,
            output: output.parse()?,
        })
    }
}

impl<'a> From<&'a Reaction> for Recipe<'a> {
    fn from(reaction: &'a Reaction) -> Self {
        Self {
            batch: reaction.output.quantity,
            ingredients: reaction.inputs.as_slice(),
        }
    }
}

fn make_recipes(reactions: &'_ [Reaction]) -> HashMap<&'_ str, Recipe<'_>> {
    reactions
        .iter()
        .map(|reaction| (reaction.output.name.as_str(), Recipe::from(reaction)))
        .collect()
}

fn ore_for_fuel(recipes: &HashMap<&str, Recipe>, fuel: usize) -> usize {
    let mut needed = HashMap::from([("FUEL", fuel)]);
    let mut surplus = HashMap::<&str, usize>::new();
    let mut ore = 0;

    while let Some((&name, &quantity)) = needed.iter().next() {
        needed.remove(name);

        // Consume whatever surplus exists, up to what we need
        let from_surplus = surplus.get(name).copied().unwrap_or_default().min(quantity);
        *surplus.entry(name).or_default() -= from_surplus;
        let quantity = quantity - from_surplus;
        if quantity == 0 {
            continue;
        }

        // Run the reaction enough times to cover the shortfall, banking the leftover
        let &Recipe { batch, ingredients } = &recipes[name];
        let runs = quantity.div_ceil(batch);
        *surplus.entry(name).or_default() += (runs * batch) - quantity;

        for Chemical { quantity, name } in ingredients {
            if name == "ORE" {
                ore += runs * quantity;
            } else {
                *needed.entry(name.as_str()).or_default() += runs * quantity;
            }
        }
    }

    ore
}

fn part1(reactions: &[Reaction]) -> usize {
    self::ore_for_fuel(&self::make_recipes(reactions), 1)
}

fn part2(reactions: &[Reaction]) -> usize {
    const ORE: usize = 1_000_000_000_000;

    let recipes = self::make_recipes(reactions);

    let mut lo = ORE / self::ore_for_fuel(&recipes, 1);
    let mut hi = 2 * lo;
    while self::ore_for_fuel(&recipes, hi) <= ORE {
        hi *= 2;
    }

    // Invariant: ore_for_fuel(lo) <= ORE < ore_for_fuel(hi)
    while lo + 1 < hi {
        let mid = (lo + (hi - lo)) / 2;
        if self::ore_for_fuel(&recipes, mid) <= ORE {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    lo
}

fn main() -> Result<()> {
    let reactions = fs::read_to_string("in/day14.txt")?
        .lines()
        .map(Reaction::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&reactions);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 443_537);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&reactions);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 2_910_558);
    };

    Ok(())
}
