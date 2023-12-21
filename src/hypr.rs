use crate::command::Command;
use crate::server::Server;
use hyprland::dispatch::{Direction, Dispatch, DispatchType};

#[derive(Debug)]
pub struct Hypr {}

impl Hypr {
    pub fn new() -> Hypr {
        Hypr {}
    }
}

impl<'a> Server<'a> for Hypr {
    fn navigate(&self, command: Command) -> Result<(), Box<dyn std::error::Error>> {
        let direction = match command {
            Command::Up => Direction::Up,
            Command::Down => Direction::Down,
            Command::Left => Direction::Left,
            _ => Direction::Right,
        };
        Ok(Dispatch::call(DispatchType::MoveFocus(direction))?)
    }
}
