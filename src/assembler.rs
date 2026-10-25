use std::{fmt::Display};

use crate::{error::Error::{self, *}, instruction::Instruction::{self, *}};

use Operand::*;
use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub enum Operand {
    Register(u8),
    Number(u16),
    RegAdress(u8),
    Adress(u16),
    Label(String),
}

#[derive(Debug, Clone, Copy)]
pub enum JumpKind {
    JMP,
    JEQ,
    JNE,
    JLT,
    JLE,
    JGT,
    JGE,
    JC,
    JNC,
}

#[derive(Debug)]
pub enum Item {
    Instruction(Instruction),
    Label(String),
    UnresolvedJump {
        kind: JumpKind,
        label: String,
    },
    UnresolvedCall(String),
}

macro_rules! mnemonics {
    ($($name:ident),* $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        #[allow(clippy::upper_case_acronyms)]
        enum Mnemonic { $($name),* }

        impl Mnemonic {
            fn from_word(w: &str) -> Option<Self> {
                match w {
                    $(stringify!($name) => Some(Self::$name),)*
                    _ => None,
                }
            }
        }
    };
}

mnemonics!(    
    HLT, LD, ST, ADD, SUB, MUL, DIV, NOT, AND, OR, XOR, LSH, RSH,
    JMP, CMP, JEQ, JNE, JLT, JLE, JGT, JGE, OUT, IN, PUSH, POP,
    CALL, RET, LDS, STS, INC, DEC, MOD, JC, JNC,
);

pub struct Assembler {
    src: String,
    pos: usize,
}

macro_rules! args {
    ($self:expr, $n:expr) => {
        match $self.parse_args($n) {
            Ok(a) => a,
            Err(e) => return throw(e),
        }
    };
}

/**
    Macro for ensuring an argument is valid, will throw the appropiate error if not.
    Possible inputs:
    - reg: throws `ExpectedReg`
    - num: throws `ExpectedNum`
    - regaddr: throws `ExpectedRegAddr`
    - label: throws `ExpectedLabel`
*/
macro_rules! ensure {
    ($arg:expr, reg) => {
        match $arg {
            Register(x) => x,
            _ => return throw(ExpectedReg),
        }
    };

    ($arg:expr, num) => {
        match $arg {
            Number(x) => x,
            _ => return throw(ExpectedNum),
        }
    };
    
    ($arg:expr, regaddr) => {
        match $arg {
            RegAdress(x) => x,
            _ => return throw(ExpectedRegAddr),
        }
    };

    ($arg:expr, label) => {
        match $arg {
            Label(x) => x,
            _ => return throw(ExpectedLabel),
        }
    };
}

impl Assembler {
    pub fn new(src: String) -> Assembler {
        Assembler {
            src,
            pos: 0,
        }
    }

    pub fn process(&mut self) -> Result<Vec<Item>, Error> {
        let mut instr = vec![];

        self.skip_whitespace();

        loop {
            if let Some(i) = self.process_instruction() {
                instr.push(i?);
            } else {
                break;
            }

            self.skip_whitespace();
        }

        Ok(instr)
    }

    fn process_instruction(&mut self) -> Option<Result<Item, Error>> {
        let word = self.read_word()?;

        if word.ends_with(':') && word.len() > 1 {
            return Some(Ok(Item::Label(
                word[..word.len() - 1].to_string()
            )));
        }

        let Some(mnemonic) = Mnemonic::from_word(word)
            else { return Some(Err(Error::InvalidMnemonic(word.to_string()))); };

        match mnemonic {
            Mnemonic::LD => {
                let args = args!(self, 2);
                let rx = ensure!(args[0], reg);

                match args[1] {
                    Register(ry) => instr(LDrr(rx, ry)),
                    Number(n) => instr(LDrn(rx, n)),
                    RegAdress(ry) => instr(LDrar(rx, ry)),
                    Adress(n) => instr(LDran(rx, n)),
                    _ => Some(Err(Error::InvalidArg)),
                }
            },
            Mnemonic::HLT => {
                instr(HLT)
            },
            Mnemonic::ST => {
                let args = args!(self, 2);
                let rx = ensure!(args[0], regaddr);
                let ry = ensure!(args[1], reg);

                instr(ST(rx, ry))
            },
            Mnemonic::MUL | Mnemonic::DIV | Mnemonic::MOD => {
                let args = args!(self, 2);
                let rx = ensure!(args[0], reg);
                let ry = ensure!(args[1], reg);

                match mnemonic {
                    Mnemonic::MUL => instr(MUL(rx, ry)),
                    Mnemonic::DIV => instr(DIV(rx, ry)),
                    Mnemonic::MOD => instr(MOD(rx, ry)),
                    _ => unreachable!()
                }
            },
            Mnemonic::NOT | Mnemonic::IN | Mnemonic::POP | Mnemonic::INC | Mnemonic::DEC => {
                let rx = ensure!(args!(self, 1)[0], reg);

                match mnemonic {
                    Mnemonic::NOT => instr(NOT(rx)),
                    Mnemonic::IN => instr(IN(rx)),
                    Mnemonic::POP => instr(POP(rx)),
                    Mnemonic::INC => instr(INC(rx)),
                    Mnemonic::DEC => instr(DEC(rx)),
                    _ => unreachable!(),
                }
            },
            Mnemonic::AND | Mnemonic::OR | Mnemonic::XOR | Mnemonic::LSH | Mnemonic::RSH | Mnemonic::SUB | Mnemonic::ADD | Mnemonic::CMP => {
                let args = args!(self, 2);
                let rx = ensure!(args[0], reg);

                match args[1] {
                    Register(ry) => match mnemonic {
                        Mnemonic::AND => instr(ANDrr(rx, ry)),
                        Mnemonic::OR => instr(ORrr(rx, ry)),
                        Mnemonic::XOR => instr(XORrr(rx, ry)),
                        Mnemonic::LSH => instr(LSHrr(rx, ry)),
                        Mnemonic::RSH => instr(RSHrr(rx, ry)),
                        Mnemonic::ADD => instr(ADDrr(rx, ry)),
                        Mnemonic::SUB => instr(SUBrr(rx, ry)),
                        Mnemonic::CMP => instr(CMPrr(rx, ry)),
                        _ => unreachable!(),
                    },
                    Number(n) => match mnemonic {
                        Mnemonic::AND => instr(ANDrn(rx, n)),
                        Mnemonic::OR => instr(ORrn(rx, n)),
                        Mnemonic::XOR => instr(XORrn(rx, n)),
                        Mnemonic::LSH => instr(LSHrn(rx, n)),
                        Mnemonic::RSH => instr(RSHrn(rx, n)),
                        Mnemonic::ADD => instr(ADDrn(rx, n)),
                        Mnemonic::SUB => instr(SUBrn(rx, n)),
                        Mnemonic::CMP => instr(CMPrn(rx, n)),
                        _ => unreachable!(),
                    },
                    _ => throw(InvalidArg),
                }
            },
            Mnemonic::JMP | Mnemonic::JEQ | Mnemonic::JNE | Mnemonic::JLT | Mnemonic::JLE | Mnemonic::JGT | Mnemonic::JGE | Mnemonic::JC | Mnemonic::JNC => {
                let args = args!(self, 1);
                let l = ensure!(args[0].clone(), label);

                let kind = match mnemonic {
                    Mnemonic::JMP => JumpKind::JMP,
                    Mnemonic::JEQ => JumpKind::JEQ,
                    Mnemonic::JNE => JumpKind::JNE,
                    Mnemonic::JLT => JumpKind::JLT,
                    Mnemonic::JLE => JumpKind::JLE,
                    Mnemonic::JGT => JumpKind::JGT,
                    Mnemonic::JGE => JumpKind::JGE,
                    Mnemonic::JC  => JumpKind::JC,
                    Mnemonic::JNC => JumpKind::JNC,
                    _ => unreachable!(),
                };

                Some(Ok(Item::UnresolvedJump {
                    kind,
                    label: l,
                }))
            },
            Mnemonic::OUT | Mnemonic::PUSH => {
                let args = args!(self, 1);
                match args[0] {
                    Register(rx) => match mnemonic {
                        Mnemonic::OUT => instr(OUTr(rx)),
                        Mnemonic::PUSH => instr(PUSHr(rx)),
                        _ => unreachable!(),
                    },
                    Number(n) => match mnemonic {
                        Mnemonic::OUT => instr(OUTn(n)),
                        Mnemonic::PUSH => instr(PUSHn(n)),
                        _ => unreachable!(),
                    },
                    _ => Some(Err(Error::InvalidArg)),
                }
            },
            Mnemonic::CALL => {
                let args = args!(self, 1);
                let l = ensure!(args[0].clone(), label);
                Some(Ok(Item::UnresolvedCall(l)))
            },
            Mnemonic::RET => {
                instr(RET)
            },
            Mnemonic::LDS => {
                let args = args!(self, 2);
                let rx = ensure!(args[0], reg);
                let n = ensure!(args[1], num);
                instr(LDS(rx, n))
            },
            Mnemonic::STS => {
                let args = args!(self, 2);
                let n = ensure!(args[0], num);
                let rx = ensure!(args[1], reg);
                instr(STS(n, rx))
            },
        }
    }

    fn parse_args(&mut self, amount: u8) -> Result<SmallVec<[Operand; 2]>, Error> {
        let mut args: SmallVec<[Operand; 2]> = SmallVec::new();

        self.skip_whitespace();

        for i in 0..amount {
            let current = match self.current() {
                Some(c) => c,
                None => return Err(NotEnoughArgs)
            };

            match current {
                'r' => {
                    let reg = match self.parse_reg() {
                        Ok(r) => r,
                        Err(e) => return Err(e),
                    };
                    args.push(Register(reg));
                },
                '[' => {
                    self.advance();
                    let addr = self.parse_args(1)?[0].clone();

                    match self.current() {
                        Some(']') => self.advance(),
                        Some(c) => return Err(BracketCloseExpected(c)),
                        None => return Err(BracketCloseEOF),
                    }

                    match addr {
                        Register(r) => args.push(RegAdress(r)),
                        Number(n) => args.push(Adress(n)),
                        _ => return Err(InvalidAddr(addr))
                    }
                },
                c if c.is_numeric() => {
                    let mut start = self.pos;

                    let (base, prefix) = if c == '0' {
                        match self.peek() {
                            Some('x' | 'X') => (16, true),
                            Some('b' | 'B') => (2, true),
                            _ => {
                                (10, false)
                            },
                        }
                    } else {
                        (10, false)
                    };

                    if prefix {
                        self.advance();
                        start = self.pos + 1;
                    }

                    loop {
                        self.advance();
                        let Some(current) = self.current() else { break; };
                        let valid = match base {
                            16 => current.is_ascii_hexdigit(),
                            2 => current == '0' || current == '1',
                            _ => current.is_ascii_digit(),
                        };
                        if !valid { break };
                    }

                    let end = self.pos;

                    let num = match u16::from_str_radix(&self.src[start..end], base) {
                        Ok(n) => n,
                        Err(_) => return Err(NumAboveCap(self.src[start..end].to_string())),
                    };
                    args.push(Number(num));
                },
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let mut s = String::from(c);
                    loop {
                        self.advance();
                        let current = match self.current() {
                            Some(c) => c,
                            None => break,
                        };
                        if current.is_ascii_alphanumeric() || current == '_' {
                            s.push(current);
                        } else {
                            break;
                        }
                    }

                    args.push(Label(s));
                },
                _ => return Err(InvalidArg),
            }

            if i == amount - 1 {
                break;
            }

            self.skip_whitespace();

            if !matches!(self.current(), Some(',')) {
                return Err(MissingArgSeperator);
            }
            self.advance();
            self.skip_whitespace();
        }

        Ok(args)
    }

    fn read_word(&mut self) -> Option<&str> {
        let start = self.pos;
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                break;
            }
            self.advance();
        }
        let end = self.pos;
        self.skip_whitespace();

        if start == end {
            None
        } else {
            Some(&self.src[start..end])
        }
    }

    fn parse_reg(&mut self) -> Result<u8, Error> {
        let next = match self.next() {
            Some(c) => c,
            None => return Err(MissingRegIndex),
        };
        if matches!(next, '0'..='7') {
            self.advance();
            Ok(next as u8 - b'0')
        } else {
            Err(InvalidReg(next))
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<char> {
        self.src.as_bytes().get(self.pos).map(|&b| b as char)
    }

    fn next(&mut self) -> Option<char> {
        self.advance();
        self.current()
    }

    fn peek(&self) -> Option<char> {
        self.src.as_bytes().get(self.pos + 1).map(|&b| b as char)
    }

    fn skip_whitespace(&mut self) {
        loop {
            if let Some(c) = self.current() && c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
}

fn throw(err: Error) -> Option<Result<Item, Error>> {
    Some(Err(err))
}

fn instr(i: Instruction) -> Option<Result<Item, Error>> {
    Some(Ok(Item::Instruction(i)))
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register(r) => write!(f, "r{r}"),
            Number(n) => write!(f, "{n}"),
            RegAdress(r) => write!(f, "[r{r}]"),
            Adress(n) => write!(f, "[{n}]"),
            Label(s) => write!(f, "{s}"),
        }
    }
}