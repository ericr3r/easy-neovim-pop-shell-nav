use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(value_enum)]
    pub command: Command,
    #[clap(short, long, value_enum, default_value_t = Backend::PopOS)]
    pub backend: Backend,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Command {
    Up,
    Down,
    Left,
    Right,
    Open,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Backend {
    Hyprland,
    PopOS,
    Sway
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Command {
    pub fn to_vim_direction(&self) -> &'static str {
        match *self {
            Command::Up => "k",
            Command::Down => "j",
            Command::Left => "h",
            Command::Right => "l",
            Command::Open => "s",
        }
    }

    pub fn to_pop_os_method_call(&self) -> &'static str {
        match *self {
            Command::Up => "FocusUp",
            Command::Down => "FocusDown",
            Command::Left => "FocusLeft",
            Command::Right => "FocusRight",
            Command::Open => "Launcher",
        }
    }
}

pub fn is_navigation(command: Command) -> bool {
    match command {
        Command::Open => false,
        _ => true,
    }
}
