use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Password([u8; 6]);

impl Password {
    pub fn digits_increment(&self) -> bool {
        self.0[..].windows(2).all(|w| w[0] <= w[1])
    }

    pub fn pairs(&self) -> Vec<[u8; 2]> {
        self.0[..]
            .windows(2)
            .filter(|w| w[0] == w[1])
            .map(|p| [p[0], p[1]])
            .collect()
    }

    pub fn contains_pair(&self) -> bool {
        !self.pairs().is_empty()
    }

    pub fn contains_pair_not_in_larger_group(&self) -> bool {
        self.pairs()
            .iter()
            .any(|pair| self.0.iter().filter(|&digit| digit == &pair[0]).count() == 2)
    }
}

impl From<u32> for Password {
    fn from(num: u32) -> Self {
        let mut password = [0u8; 6];

        num.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .take(6)
            .enumerate()
            .for_each(|(index, digit)| password[index] = digit);

        Password(password)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PasswordRange {
    start: u32,
    end: u32,
}

impl FromStr for PasswordRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let delim_pos = s.find('-').unwrap();

        Ok(Self {
            start: s[0..delim_pos].parse().unwrap(),
            end: s[delim_pos + 1..].parse().unwrap(),
        })
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Password> {
    let range = input.parse::<PasswordRange>().unwrap();
    (range.start..=range.end).map(Password::from).collect()
}

#[aoc(day4, part1)]
fn solve_part_1(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|pass| pass.digits_increment())
        .filter(|pass| pass.contains_pair())
        .count()
}

#[aoc(day4, part2)]
fn solve_part_2(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|pass| {
            pass.contains_pair() && pass.digits_increment() && pass.contains_pair_not_in_larger_group()
        })
        .count()
}

#[cfg(test)]
pub mod tests {
    use crate::day4::*;

    #[test]
    fn test_password_range_from_str() {
        assert_eq!(
            Ok(PasswordRange {
                start: 402328,
                end: 864247
            }),
            "402328-864247".parse()
        );
    }

    #[test]
    fn test_password_from_u32() {
        assert_eq!(Password([1, 2, 3, 4, 5, 6]), 123456.into());
    }

    #[test]
    fn test_password_digits_increment() {
        assert_eq!(false, Password([1, 2, 3, 4, 3, 6]).digits_increment());
        assert_eq!(true, Password([1, 2, 3, 4, 5, 6]).digits_increment());
    }

    #[test]
    fn test_password_contains_pair() {
        assert_eq!(false, Password([1, 2, 3, 4, 5, 6]).contains_pair());
        assert_eq!(true, Password([1, 2, 2, 4, 5, 6]).contains_pair());
    }

    #[test]
    fn test_password_contains_pair_not_in_larger_group() {
        assert_eq!(true, Password([1,1,2,2,3,3]).contains_pair_not_in_larger_group());
        assert_eq!(false, Password([1,2,3,4,4,4]).contains_pair_not_in_larger_group());
        assert_eq!(true, Password([1,1,1,1,2,2]).contains_pair_not_in_larger_group());
    }
}
