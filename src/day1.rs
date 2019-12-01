#[aoc_generator(day1)]
fn gen_part1(input: &str) -> Vec<u32> {
    input.lines().map(|d| d.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    input.iter().map(|d| d / 3 - 2).sum()
}

#[aoc(day1, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    input.iter().map(|d| calculate_fuel(*d)).sum()
}

fn calculate_fuel(mass: u32) -> u32 {
    let fuel: i64 = mass as i64 / 3 - 2;
    if fuel > 0 {
        fuel as u32 + calculate_fuel(fuel as u32)
    }
    else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fuel() {
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 966);
        assert_eq!(calculate_fuel(100756), 50346);
    }
}
