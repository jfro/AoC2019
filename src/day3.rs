use std::collections::HashMap;
use std::num::ParseIntError;

fn manhattan_distance(p1: &Point, p2: &Point) -> u32 {
    ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()) as u32
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
    fn mdist_from_origin(&self) -> u32 {
        manhattan_distance(&Point::new(0, 0), self)
    }
}

#[derive(Debug, PartialEq)]
enum WireDirection {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}
impl WireDirection {
    fn points_from(&self, start: &Point) -> Vec<Point> {
        match self {
            WireDirection::Up(y) => {
                let end = start.y + i64::from(*y);
                (start.y + 1..=end)
                    .map(|y| Point { x: start.x, y })
                    .collect()
            }
            WireDirection::Down(y) => {
                let end = start.y - i64::from(*y);
                (end..=start.y - 1)
                    .map(|y| Point { x: start.x, y })
                    .rev()
                    .collect()
            }
            WireDirection::Left(x) => {
                let end = start.x - i64::from(*x);
                (end..=start.x - 1)
                    .map(|x| Point { x, y: start.y })
                    .rev()
                    .collect()
            }
            WireDirection::Right(x) => {
                let end = start.x + i64::from(*x);
                (start.x + 1..=end)
                    .map(|x| Point { x, y: start.y })
                    .collect()
            }
        }
    }
}
#[derive(Debug)]
struct Wire {
    id: u8,
    directions: Vec<WireDirection>,
}
impl Wire {
    fn points(&self) -> Vec<Point> {
        let mut dir_points: Vec<Point> = Vec::new();
        let mut point = Point::new(0, 0); // port is 0,0
        for dir in &self.directions {
            let mut points = dir.points_from(&point);
            point = points.last().unwrap().clone();
            dir_points.append(&mut points);
        }
        dir_points
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
        Ok(Wire { id: 0, directions })
    }
}

#[aoc_generator(day3)]
fn gen_part1(input: &str) -> Vec<Wire> {
    input
        .lines()
        .enumerate()
        .map(|(i, d)| {
            let mut wire: Wire = d.parse().unwrap();
            wire.id = i as u8;
            wire
        })
        .collect()
}

#[aoc(day3, part1)]
fn solve_part1(input: &[Wire]) -> u32 {
    closest_intersection(input)
}

fn closest_intersection(wires: &[Wire]) -> u32 {
    let mut map: HashMap<Point, Vec<u8>> = HashMap::new();
    for wire in wires {
        let points = wire.points();
        for point in points {
            let entry = map.entry(point).or_insert(vec![]);
            if !entry.contains(&wire.id) {
                entry.push(wire.id);
            }
        }
    }
    let result = map
        .iter()
        .filter(|(_k, v)| v.len() > 1)
        .map(|(k, _v)| (k, manhattan_distance(&Point::new(0, 0), k)))
        .min_by(|(_p, dist1), (_p2, dist2)| dist1.cmp(dist2));
    result.unwrap().1
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
        let wire: Wire = "R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83"
            .parse()
            .unwrap();
        assert_eq!(
            wire.directions,
            vec![
                WireDirection::Right(75),
                WireDirection::Down(30),
                WireDirection::Right(83),
                WireDirection::Up(83),
                WireDirection::Left(12),
                WireDirection::Down(49),
                WireDirection::Right(71),
                WireDirection::Up(7),
                WireDirection::Left(72),
                WireDirection::Up(62),
                WireDirection::Right(66),
                WireDirection::Up(55),
                WireDirection::Right(34),
                WireDirection::Down(71),
                WireDirection::Right(55),
                WireDirection::Down(58),
                WireDirection::Right(83)
            ]
        );
    }
    #[test]
    fn test_dir_points() {
        let start = Point::new(0, 0);
        let dir: WireDirection = "U2".parse().unwrap();
        let points = dir.points_from(&start);
        println!("Points: {:?}", points);
        assert_eq!(
            points,
            vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2)]
        );

        let start = Point::new(0, 2);
        let dir: WireDirection = "D4".parse().unwrap();
        let points = dir.points_from(&start);
        println!("Points: {:?}", points);
        assert_eq!(
            points,
            vec![
                Point::new(0, 2),
                Point::new(0, 1),
                Point::new(0, 0),
                Point::new(0, -1),
                Point::new(0, -2)
            ]
        );
    }
    #[test]
    fn test_wire_points() {
        let mut wire: Wire = "R5,D4,R1,U6".parse().unwrap();
        wire.id = 1;
        let mut wire2: Wire = "U2,R1,D4,L6".parse().unwrap();
        wire2.id = 2;
        // let points = wire.points();
        // println!("Points: {:?}", points);
        closest_intersection(&vec![wire, wire2]);
    }
}
