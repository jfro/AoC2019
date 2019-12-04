use std::num::ParseIntError;
use std::collections::HashMap;

fn manhattan_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    let x1: i64 = x1 as i64;
    let y1: i64 = y1 as i64;
    let x2: i64 = x2 as i64;
    let y2: i64 = y2 as i64;
    ((x1 - x2).abs() + (y1 - y2).abs()) as u32
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}
#[derive(Debug, PartialEq)]
enum WireDirection {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}
#[derive(Debug)]
struct Wire {
    directions: Vec<WireDirection>,
}
impl Wire {
    fn points(&self) -> Vec<Point> {
        vec![]
    }
}
#[derive(Debug)]
enum WireError {
    InvalidDirectionString,
    ParseIntError(ParseIntError),
}
impl From<ParseIntError> for WireError {
    fn from(error: ParseIntError) -> WireError {
       WireError::ParseIntError(error) 
    }
}
impl std::str::FromStr for WireDirection {
    type Err = WireError;
    fn from_str(string: &str) -> Result<WireDirection, Self::Err> {
        let (dir, amount) = (&string[0..1], &string[1..]);
        let amount: u32 = amount.parse()?;
        match dir {
            "U" => Ok(WireDirection::Up(amount)),
            "D" => Ok(WireDirection::Down(amount)),
            "L" => Ok(WireDirection::Left(amount)),
            "R" => Ok(WireDirection::Right(amount)),
            _ => Err(WireError::InvalidDirectionString),
        }
    }
}
impl std::str::FromStr for Wire {
    type Err = WireError;
    fn from_str(string: &str) -> Result<Wire, Self::Err> {
        let directions = string.split(",").map(|d| d.parse().unwrap()).collect();
        Ok(Wire {
            directions
        })
    }
}

#[aoc_generator(day3)]
fn gen_part1(input: &str) -> Vec<Wire> {
    input.lines().map(|d| d.parse().unwrap()).collect()
}

#[aoc(day3, part1)]
fn solve_part1(_input: &[Wire]) -> u32 {
    let map: HashMap<Point, u8> = HashMap::new();
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_direction() {
        let dir: WireDirection = "U2340".parse().unwrap();
        assert_eq!(dir, WireDirection::Up(2340));
        let dir: WireDirection = "D2".parse().unwrap();
        assert_eq!(dir, WireDirection::Down(2));
        let dir: WireDirection = "L92".parse().unwrap();
        assert_eq!(dir, WireDirection::Left(92));
        let dir: WireDirection = "R203413".parse().unwrap();
        assert_eq!(dir, WireDirection::Right(203413));
    }
    #[test]
    fn test_wire() {
        let wire: Wire = "R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83".parse().unwrap();
        assert_eq!(wire.directions, vec![WireDirection::Right(75), WireDirection::Down(30), WireDirection::Right(83), WireDirection::Up(83), WireDirection::Left(12), WireDirection::Down(49), WireDirection::Right(71), WireDirection::Up(7), WireDirection::Left(72), WireDirection::Up(62), WireDirection::Right(66), WireDirection::Up(55), WireDirection::Right(34), WireDirection::Down(71), WireDirection::Right(55), WireDirection::Down(58), WireDirection::Right(83)]);
    }
}
