#[derive(Debug)]
struct Ingredient(u64);

impl Ingredient {
    fn is_fresh(&self, ranges: &[std::ops::RangeInclusive<u64>]) -> bool {
        for range in ranges.iter() {
            if range.contains(&self.0) {
                return true;
            }
        }
        false
    }
}

fn main() -> anyhow::Result<()> {
    let input = aoc::fetch_puzzle_input(5)?;
    println!("part1 ans -> {}", part1(&input)?);
    println!("part2 ans -> {}", part2(&input)?);
    Ok(())
}

fn parse_input(
    input: &str,
) -> anyhow::Result<(Vec<std::ops::RangeInclusive<u64>>, Vec<Ingredient>)> {
    let mut input = input.split('\n');

    let ranges = input
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut ss = line.split('-');
            let start = ss.next().unwrap().parse::<u64>().unwrap();
            let end = ss.next().unwrap().parse::<u64>().unwrap();
            std::ops::RangeInclusive::new(start, end)
        })
        .collect::<Vec<_>>();

    // The rest are ingredients
    let ingredients: Vec<Ingredient> = input
        .filter(|line| !line.is_empty())
        .map(|line| Ingredient(line.parse::<u64>().unwrap()))
        .collect();

    Ok((ranges, ingredients))
}

fn part1(input: &str) -> anyhow::Result<String> {
    let (ranges, ingredients) = parse_input(input)?;
    println!("{ranges:?}");
    let ans = ingredients
        .iter()
        .filter(|ingredient| ingredient.is_fresh(&ranges))
        .inspect(|ing| {
            println!("Spoiled ingredient: {:?}", ing);
        })
        .count();
    Ok(ans.to_string())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let (ranges, _) = parse_input(input)?;
    let out_ranges = merge_ranges(ranges);

    let ans = out_ranges
        .into_iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<u64>();

    Ok(ans.to_string())
}

fn merge_ranges(
    mut ranges: Vec<std::ops::RangeInclusive<u64>>,
) -> Vec<std::ops::RangeInclusive<u64>> {
    // If we sort first by start, then only the end can get bigger which simplifies merging
    ranges.sort_by_key(|r| *r.start());

    let mut merged: Vec<std::ops::RangeInclusive<u64>> = vec![];
    for range in ranges {
        if let Some(last) = merged.last_mut() {
            // If overlap or adjacent, merge
            if overlaps(last, &range) {
                let new_end = std::cmp::max(*last.end(), *range.end());
                // replace the end
                *last = std::ops::RangeInclusive::new(*last.start(), new_end);
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }
    merged
}

fn overlaps<T: Ord>(lhs: &std::ops::RangeInclusive<T>, rhs: &std::ops::RangeInclusive<T>) -> bool {
    lhs.end() >= rhs.start()
}
