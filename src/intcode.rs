use std::collections::VecDeque;

use anyhow::Error;
use anyhow::Result;
use anyhow::bail;

#[derive(Clone, Copy, Debug)]
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

#[derive(Clone, Copy, Debug)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    modes: [Mode; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run {
    Output(i64),
    NeedsInput,
    Halted,
}

#[derive(Clone, Debug)]
pub struct IntcodeCpu {
    mem: Vec<i64>,
    ip: usize,
    inputs: VecDeque<i64>,
}

impl TryFrom<i64> for Opcode {
    type Error = Error;

    fn try_from(opcode: i64) -> Result<Self> {
        match opcode {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            5 => Ok(Self::JumpIfTrue),
            6 => Ok(Self::JumpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),
            99 => Ok(Self::Halt),
            _ => bail!("invalid opcode '{opcode}'"),
        }
    }
}

impl TryFrom<i64> for Mode {
    type Error = Error;

    fn try_from(mode: i64) -> Result<Self> {
        match mode {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => bail!("invalid mode '{mode}'"),
        }
    }
}

impl TryFrom<i64> for Instruction {
    type Error = Error;

    fn try_from(word: i64) -> Result<Self> {
        let mode = word / 100;
        Ok(Self {
            opcode: Opcode::try_from(word % 100)?,
            modes: [
                Mode::try_from(mode % 10)?,
                Mode::try_from((mode / 10) % 10)?,
                Mode::try_from((mode / 100) % 10)?,
            ],
        })
    }
}

impl<I: Iterator<Item = i64>> From<I> for IntcodeCpu {
    fn from(program: I) -> Self {
        Self {
            mem: program.collect(),
            ip: 0,
            inputs: VecDeque::new(),
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl IntcodeCpu {
    /// Queue an input value for the next `Input` opcode.
    pub fn push(&mut self, val: i64) {
        self.inputs.push_back(val);
    }

    fn read(&self, offset: usize, mode: Mode) -> i64 {
        let val = self.mem[self.ip + offset];
        match mode {
            Mode::Position => self.mem[val as usize],
            Mode::Immediate => val,
        }
    }

    fn write(&mut self, offset: usize, val: i64) {
        let dst = self.mem[self.ip + offset] as usize;
        self.mem[dst] = val;
    }

    /// Run until the program produces an output, needs an input it does not
    /// have, or halts. Resumable: call again after pushing more input.
    ///
    /// # Panics
    ///
    /// Panics if the program contains an invalid instruction word.
    pub fn run(&mut self) -> Run {
        loop {
            let instruction = Instruction::try_from(self.mem[self.ip]).unwrap();
            match instruction.opcode {
                Opcode::Add => {
                    let a = self.read(1, instruction.modes[0]);
                    let b = self.read(2, instruction.modes[1]);
                    self.write(3, a + b);
                    self.ip += 4;
                }
                Opcode::Mul => {
                    let a = self.read(1, instruction.modes[0]);
                    let b = self.read(2, instruction.modes[1]);
                    self.write(3, a * b);
                    self.ip += 4;
                }
                Opcode::Input => {
                    let Some(input) = self.inputs.pop_front() else {
                        return Run::NeedsInput;
                    };
                    self.write(1, input);
                    self.ip += 2;
                }
                Opcode::JumpIfTrue => {
                    let cond = self.read(1, instruction.modes[0]);
                    self.ip = if cond != 0 {
                        self.read(2, instruction.modes[1]) as usize
                    } else {
                        self.ip + 3
                    };
                }
                Opcode::JumpIfFalse => {
                    let cond = self.read(1, instruction.modes[0]);
                    self.ip = if cond == 0 {
                        self.read(2, instruction.modes[1]) as usize
                    } else {
                        self.ip + 3
                    };
                }
                Opcode::LessThan => {
                    let a = self.read(1, instruction.modes[0]);
                    let b = self.read(2, instruction.modes[1]);
                    self.write(3, i64::from(a < b));
                    self.ip += 4;
                }
                Opcode::Equals => {
                    let a = self.read(1, instruction.modes[0]);
                    let b = self.read(2, instruction.modes[1]);
                    self.write(3, i64::from(a == b));
                    self.ip += 4;
                }
                Opcode::Output => {
                    let output = self.read(1, instruction.modes[0]);
                    self.ip += 2;
                    return Run::Output(output);
                }
                Opcode::Halt => return Run::Halted,
            }
        }
    }
}
