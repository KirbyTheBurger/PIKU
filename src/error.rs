use std::fmt::Display;

use Error::*;

use crate::assembler::Operand;

#[derive(Debug)]
pub enum Error {
    ExpectedReg,
    ExpectedRegAddr,
    ExpectedNum,
    ExpectedLabel,

    InvalidReg(char),
    MissingRegIndex,
    MissingArgSeperator,
    NotEnoughArgs,
    InvalidAddr(Operand),
    InvalidArg,
    BracketCloseExpected(char),
    BracketCloseEOF,
    NumAboveCap(String),
    InvalidMnemonic(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedReg => write!(f, "Invalid argument, expected a register"),
            InvalidReg(c) => write!(f, "Invalid register: register `{c}` does not exist"),
            MissingRegIndex => write!(f, "Missing register index"),
            MissingArgSeperator => write!(f, "Missing argument seperator `,`"),
            NotEnoughArgs => write!(f, "Not enough arguments supplied"),
            InvalidAddr(o) => write!(f, "Invalid memory adress `{}`", *o),
            InvalidArg => write!(f, "Invalid argument"),
            BracketCloseExpected(c) => write!(f, "Expected `]`, got `{c}`"),
            BracketCloseEOF => write!(f, "Expected `]`, got `EOF`"),
            NumAboveCap(n) => write!(f, "Number `{n}` is greater than 65535"),
            ExpectedNum => write!(f, "Invalid argument, expected a number"),
            ExpectedRegAddr => write!(f, "Invalid argument, expected a register containing a memory adress"),
            ExpectedLabel => write!(f, "Invalid argument, expected a label"),
            InvalidMnemonic(s) => write!(f, "Invalid mnemonic `{s}`"),
        }
    }
}

impl std::error::Error for Error {}