use aoc_runner_derive::{aoc, aoc_generator};

type IntCResult<T> = Result<T, String>;

struct IntComputer {
    code: Vec<u32>,
    pos: usize,
}

impl IntComputer {
    fn new(code: Vec<u32>) -> Self {
        Self { code, pos: 0 }
    }

    fn get_integer_at(&self, index: usize) -> IntCResult<u32> {
        match self.code.get(index) {
            Some(c) => Ok(*c),
            None => Err(format!("no integer found at position {}", index)),
        }
    }

    fn set_integer_at(&mut self, index: usize, value: u32) -> IntCResult<()> {
        match self.code.get_mut(index) {
            Some(val) => {
                *val = value;

                Ok(())
            }
            None => Err(format!("no integer found at position {}", index)),
        }
    }

    fn run(&mut self) -> IntCResult<()> {
        while let Some(c) = self.code.get(self.pos) {
            let lhs = self.get_integer_at(self.get_integer_at(self.pos + 1)? as usize);
            let rhs = self.get_integer_at(self.get_integer_at(self.pos + 2)? as usize);
            let result_pos = self.get_integer_at(self.pos + 3);

            match *c {
                // Add
                1 => self.set_integer_at(result_pos? as usize, lhs? + rhs?)?,
                // Mul
                2 => self.set_integer_at(result_pos? as usize, lhs? * rhs?)?,
                // Exit
                99 => return Ok(()),
                // invalid
                _ => {
                    return Err(format!(
                        "Not a valid instruction code: {}, pos: {}",
                        c, self.pos
                    ))
                }
            }

            self.pos += 4;
        }

        Ok(())
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part_1(code: &[u32]) -> u32 {
    let mut computer = IntComputer::new(code.to_vec());
    computer.set_integer_at(1, 12).unwrap();
    computer.set_integer_at(2, 2).unwrap();
    computer.run().unwrap();

    computer.get_integer_at(0).unwrap()
}

#[aoc(day2, part2)]
fn solve_part_2(code: &[u32]) -> u32 {
    for x in 0..100 {
        for y in 0..100 {
            let mut computer = IntComputer::new(code.to_vec());
            computer.set_integer_at(1, x).unwrap();
            computer.set_integer_at(2, y).unwrap();
            computer.run().unwrap();
            let res = computer.get_integer_at(0).unwrap();

            if res == 19_690_720 {
                return 100 * x + y;
            }
        }
    }

    unreachable!()
}
