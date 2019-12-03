use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;
use std::error::Error;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(coordinates: (i32, i32)) -> Self {
        Self {
            x: coordinates.0,
            y: coordinates.1,
        }
    }

    pub fn distance(self, other: Position) -> i32 {
        (self.x.abs() - other.x.abs()) + (self.y.abs() - other.y.abs())
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, len) = s.split_at(1);
        let len = len
            .parse::<i32>()
            .map_err(|e| e.description().to_string())?;

        match dir {
            "R" => Ok(Direction::Right(len)),
            "L" => Ok(Direction::Left(len)),
            "U" => Ok(Direction::Up(len)),
            "D" => Ok(Direction::Down(len)),
            _ => Err(format!("unknown direction {}", dir)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Wire(Vec<Direction>);

impl FromIterator<Direction> for Wire {
    fn from_iter<T: IntoIterator<Item = Direction>>(iter: T) -> Self {
        Wire(iter.into_iter().collect())
    }
}

impl Wire {
    pub fn positions(&self) -> Vec<Position> {
        let mut positions = Vec::<Position>::new();
        let mut current_pos = Position::new((0, 0));

        for direction in &self.0 {
            match direction {
                Direction::Left(len) => {
                    ((current_pos.x - *len)..=current_pos.x)
                        .rev()
                        .skip(1)
                        .for_each(|x| positions.push(Position::new((x, current_pos.y))));

                    current_pos.x -= *len
                }
                Direction::Right(len) => {
                    (current_pos.x..=current_pos.x + *len)
                        .skip(1)
                        .for_each(|x| positions.push(Position::new((x, current_pos.y))));

                    current_pos.x += *len
                }
                Direction::Up(len) => {
                    (current_pos.y..=current_pos.y + *len)
                        .skip(1)
                        .for_each(|y| positions.push(Position::new((current_pos.x, y))));

                    current_pos.y += *len
                }
                Direction::Down(len) => {
                    ((current_pos.y - *len)..=current_pos.y)
                        .rev()
                        .skip(1)
                        .for_each(|y| positions.push(Position::new((current_pos.x, y))));

                    current_pos.y -= *len
                }
            }
        }

        positions
    }

    pub fn intersections(&self, wire: &Wire) -> Vec<Position> {
        let positions_a = HashSet::<Position>::from_iter(self.positions());
        let positions_b = HashSet::<Position>::from_iter(wire.positions());

        positions_a.intersection(&positions_b).copied().collect()
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Wire> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|part| part.parse::<Direction>())
                .filter_map(|res| res.ok())
                .collect::<Wire>()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part_1(wires: &[Wire]) -> i32 {
    let center = Position::new((0, 0));

    wires[0]
        .intersections(&wires[1])
        .iter()
        .map(|pos| pos.distance(center))
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn solve_part_2(wires: &[Wire]) -> usize {
    let intersections = wires[0].intersections(&wires[1]);
    let positions_a = wires[0].positions();
    let positions_b = wires[1].positions();

    intersections
        .iter()
        .map(|intersection| {
            let steps_a = positions_a
                .iter()
                .position(|pos| pos == intersection)
                .unwrap()
                + 1;
            let steps_b = positions_b
                .iter()
                .position(|pos| pos == intersection)
                .unwrap()
                + 1;

            steps_a + steps_b
        })
        .min()
        .unwrap()
}
