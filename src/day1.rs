#[aoc_generator(day1)]
fn gen_part1(input: &str) -> Vec<u32> {
    input.lines().map(|d| d.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    input.iter().map(|d| d / 3 - 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
}
