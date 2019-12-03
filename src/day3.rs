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

    pub fn distance(&self, other: &Position) -> i32 {
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
    pub fn coordinates(&self) -> HashSet<Position> {
        let coordinates = {
            let mut tmp_positions = HashSet::<Position>::new();
            let mut current_pos = Position::new((0, 0));

            for direction in &self.0 {
                let new_pos = Position::new(match direction {
                    Direction::Up(len) => (current_pos.x, current_pos.y + *len),
                    Direction::Down(len) => (current_pos.x, current_pos.y - *len),
                    Direction::Left(len) => (current_pos.x - *len, current_pos.y),
                    Direction::Right(len) => (current_pos.x + *len, current_pos.y),
                });

                for x in current_pos.x..=new_pos.x {
                    for y in current_pos.y..=new_pos.y {
                        tmp_positions.insert(Position::new((x, y)));
                    }
                }

                current_pos = new_pos;
            }

            tmp_positions
        };

        coordinates
    }

    pub fn intersections(&self, wire: &Wire) -> Vec<Position> {
        let wire2_coordinates = wire.coordinates();

        self.coordinates()
            .iter()
            .filter(|&c| wire2_coordinates.contains(c))
            .copied()
            .collect()
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
        .map(|pos| pos.distance(&center))
        .min()
        .unwrap()
}
