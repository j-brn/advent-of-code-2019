use aoc_runner_derive::{aoc, aoc_generator};

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum IntCodeError {
    InvalidInstruction { instruction: u32, pos: usize },
    InvalidParameterMode { mode: u32, pos: usize },
    UnexpectedEndOfInput { pos: usize },
}

impl Display for IntCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IntCodeError::InvalidInstruction { instruction, pos } => write!(
                f,
                "attempted  to run invalid unknown instruction {} at position {}",
                instruction, pos
            )?,
            IntCodeError::InvalidParameterMode { mode, pos } => {
                write!(f, "invalid parameter mode {} at position {}", mode, pos)?
            }
            IntCodeError::UnexpectedEndOfInput { pos } => {
                write!(f, "unexpected end of input at position {}", pos)?
            }
        };

        Ok(())
    }
}

impl Error for IntCodeError {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IntCodeVM {
    memory: Vec<i32>,
    ptr: usize,
}

impl FromStr for IntCodeVM {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory = s.split(',').map(|s| s.parse::<i32>().unwrap()).collect();

        Ok(Self { memory, ptr: 0 })
    }
}

impl IntCodeVM {
    /// Run the next instruction and increment the instruction pointer.
    /// Returns false if an exit instruction (code 99) was reached.
    pub fn run_instruction(&mut self) -> Result<bool, Box<dyn Error>> {
        let instruction = Instruction::from_int(self.read_int(self.ptr)? as u32);

        match instruction.code {
            OpCode::Add => {
                let lhs = self.read_parameter(self.ptr + 1, instruction.get_param_mode(0))?;
                let rhs = self.read_parameter(self.ptr + 2, instruction.get_param_mode(1))?;
                let res = self.read_parameter(self.ptr + 3, ParamMode::Immediate)? as usize;

                self.set_int(res, lhs + rhs);
                self.jmp(self.ptr + 4);

                Ok(true)
            }
            OpCode::Multiply => {
                let lhs = self.read_parameter(self.ptr + 1, instruction.get_param_mode(0))?;
                let rhs = self.read_parameter(self.ptr + 2, instruction.get_param_mode(1))?;
                let res = self.read_parameter(self.ptr + 3, ParamMode::Immediate)? as usize;

                self.set_int(res, lhs * rhs);
                self.jmp(self.ptr + 4);

                Ok(true)
            }
            OpCode::Input => {
                let dst = self.read_parameter(self.ptr + 1, ParamMode::Immediate)? as usize;
                println!("[IntCodeVM] input required:");
                let line = std::io::stdin().lock().lines().next().unwrap()?;
                self.set_int(dst, line.parse::<i32>()?);
                self.jmp(self.ptr + 2);

                Ok(true)
            }
            OpCode::Output => {
                let value = self.read_parameter(self.ptr + 1, instruction.get_param_mode(0))?;
                println!("IntCodeVM: {}", value);
                self.jmp(self.ptr + 2);

                Ok(true)
            }
            OpCode::JumpIfTrue => {
                let value = self.read_parameter(self.ptr + 1, instruction.get_param_mode(0))?;
                let dst =
                    self.read_parameter(self.ptr + 2, instruction.get_param_mode(1))? as usize;

                if value != 0 {
                    self.jmp(dst);
                } else {
                    self.jmp(self.ptr + 3);
                }

                Ok(true)
            }
            OpCode::JumpIfFalse => {
                let value = self.read_parameter(self.ptr + 1, instruction.get_param_mode(0))?;
                let dst =
                    self.read_parameter(self.ptr + 2, instruction.get_param_mode(1))? as usize;

                if value == 0 {
                    self.jmp(dst);
                } else {
                    self.jmp(self.ptr + 3);
                }

                Ok(true)
            }
            OpCode::LessThan => {
                let lhs = self.read_parameter(self.ptr + 1, instruction.get_param_mode(0))?;
                let rhs = self.read_parameter(self.ptr + 2, instruction.get_param_mode(1))?;
                let dst = self.read_parameter(self.ptr + 3, ParamMode::Immediate)? as usize;

                self.set_int(dst, if lhs < rhs { 1 } else { 0 });

                self.jmp(self.ptr + 4);

                Ok(true)
            }
            OpCode::Equals => {
                let lhs = self.read_parameter(self.ptr + 1, instruction.get_param_mode(0))?;
                let rhs = self.read_parameter(self.ptr + 2, instruction.get_param_mode(1))?;
                let dst = self.read_parameter(self.ptr + 3, ParamMode::Immediate)? as usize;

                self.set_int(dst, if lhs == rhs { 1 } else { 0 });

                self.jmp(self.ptr + 4);

                Ok(true)
            }
            OpCode::Exit => Ok(false),
            OpCode::Invalid(code) => Err(Box::new(IntCodeError::InvalidInstruction {
                instruction: code,
                pos: self.ptr,
            })),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while self.run_instruction()? {}

        Ok(())
    }

    // read the value at the given position in memory
    pub fn read_int(&self, position: usize) -> Result<i32, IntCodeError> {
        match self.memory.get(position) {
            Some(value) => Ok(*value),
            None => Err(IntCodeError::UnexpectedEndOfInput { pos: position }),
        }
    }

    // store the value at the given position in memory
    pub fn set_int(&mut self, position: usize, value: i32) {
        self.memory[position] = value
    }

    pub fn read_parameter(&self, position: usize, mode: ParamMode) -> Result<i32, IntCodeError> {
        match mode {
            ParamMode::Positional => Ok(self.read_int(self.read_int(position)? as usize)?),
            ParamMode::Immediate => Ok(self.read_int(position)?),
            ParamMode::Invalid(n) => Err(IntCodeError::InvalidParameterMode {
                mode: n,
                pos: position,
            }),
        }
    }

    /// set the instruction pointer to the given position
    pub fn jmp(&mut self, destination: usize) {
        self.ptr = destination
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    code: OpCode,
    parameter_modes: Vec<ParamMode>,
}

impl Instruction {
    pub fn from_int(i: u32) -> Self {
        let digits = format!("{:0>2}", i)
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .rev()
            .collect::<Vec<u32>>();

        let code = OpCode::from(digits[0] + (10 * digits[1]));
        let parameter_modes = digits[2..].iter().map(|&m| m.into()).collect();

        Instruction {
            code,
            parameter_modes,
        }
    }

    /// Get the mode for the given parameter. Defaults to zero
    pub fn get_param_mode(&self, param: usize) -> ParamMode {
        *self
            .parameter_modes
            .get(param)
            .unwrap_or(&ParamMode::Positional)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ParamMode {
    Positional,
    Immediate,
    Invalid(u32),
}

impl From<u32> for ParamMode {
    fn from(value: u32) -> Self {
        match value {
            0 => ParamMode::Positional,
            1 => ParamMode::Immediate,
            n => ParamMode::Invalid(n),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Exit,
    Invalid(u32),
}

impl From<u32> for OpCode {
    fn from(code: u32) -> Self {
        match code {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Exit,
            n => OpCode::Invalid(n),
        }
    }
}

#[aoc_generator(day5)]
fn day2_generator(input: &str) -> IntCodeVM {
    input.parse().unwrap()
}

#[aoc(day5, part1)]
#[aoc(day5, part2)]
fn solve_d2_part1(vm: &IntCodeVM) -> &'static str {
    let mut vm = (*vm).clone();
    vm.run().unwrap();
    "see output"
}

#[cfg(test)]
pub mod tests {
    use crate::day5::*;

    #[test]
    fn test_vm_from_str() {
        let input = "1,2,3,11,1337,99";
        let vm = IntCodeVM {
            memory: vec![1, 2, 3, 11, 1337, 99],
            ptr: 0,
        };

        assert_eq!(vm, input.parse().unwrap())
    }

    #[test]
    fn test_vm_get_int() {
        let vm = IntCodeVM {
            memory: vec![1, 2, 3, 11, 1337, 99],
            ptr: 0,
        };

        assert_eq!(Ok(1337), vm.read_int(4));
    }

    #[test]
    fn test_vm_set_int() {
        let mut vm = IntCodeVM {
            memory: vec![1, 2, 3, 11, 1337, 99],
            ptr: 0,
        };
        vm.set_int(4, 11);

        assert_eq!(Ok(11), vm.read_int(4));
    }

    #[test]
    fn test_vm_jump() {
        let mut vm = IntCodeVM {
            memory: Vec::new(),
            ptr: 0,
        };
        vm.jmp(5);

        assert_eq!(vm.ptr, 5);
    }

    #[test]
    fn test_vm_read_parameter() {
        let mut vm = IntCodeVM {
            memory: vec![1, 4, 3, 11, 1337, 99],
            ptr: 0,
        };
        assert_eq!(vm.read_parameter(1, 0).unwrap(), 1337);
        assert_eq!(vm.read_parameter(1, 1).unwrap(), 4);
    }

    #[test]
    fn test_instruction_from_int() {
        let instruction = Instruction {
            code: OpCode::Mul,
            parameter_modes: vec![ParamMode::Immediate, ParamMode::Immediate],
        };

        assert_eq!(instruction, Instruction::from_int(01102));
        assert_eq!(0, instruction.get_param_mode(2));
    }
}
