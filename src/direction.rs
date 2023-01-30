use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(arg_enum, value_parser)]
    pub direction: Direction,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Direction {
    pub fn to_vim_direction(&self) -> &'static str {
        match *self {
            Direction::Up => "k",
            Direction::Down => "j",
            Direction::Left => "h",
            Direction::Right => "l",
        }
    }

    pub fn to_pop_os_method_call(&self) -> &'static str {
        match *self {
            Direction::Up => "FocusUp",
            Direction::Down => "FocusDown",
            Direction::Left => "FocusLeft",
            Direction::Right => "FocusRight",
        }
    }
}
