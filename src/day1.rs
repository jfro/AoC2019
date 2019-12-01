#[aoc_generator(day1)]
fn gen_part1(input: &str) -> Vec<u32> {
    input.lines().map(|d| d.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    input.iter().map(|d| d / 3 - 2).sum()
}

#[aoc(day1, part2, Recursion)]
fn solve_part2(input: &[u32]) -> u32 {
    input.iter().map(|d| calculate_fuel(*d)).sum()
}

#[aoc(day1, part2, Loop)]
fn solve_part2_loop(input: &[u32]) -> u32 {
    input.iter().map(|d| calculate_fuel_loop(*d)).sum()
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
fn calculate_fuel_loop(mass: u32) -> u32 {
    let mut fuel = mass / 3 - 2;
    let mut last_fuel = fuel;
    loop {
        if last_fuel / 3 <= 2 {
            break fuel;
        }
        last_fuel = last_fuel / 3 - 2;
        fuel += last_fuel;

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
    #[test]
    fn test_fuel_loop() {
        assert_eq!(calculate_fuel_loop(14), 2);
        assert_eq!(calculate_fuel_loop(1969), 966);
        assert_eq!(calculate_fuel_loop(100756), 50346);
    }
}
