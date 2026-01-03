pub fn fetch_puzzle_input(puzzle_number: u8) -> anyhow::Result<String> {
    let aoc_token = std::env::var("AOC_2025_TOKEN")?;
    let url = format!("https://adventofcode.com/2025/day/{puzzle_number}/input");
    let mut resp = ureq::get(url).header("Cookie", aoc_token).call()?;

    Ok(resp.body_mut().read_to_string()?)
}
