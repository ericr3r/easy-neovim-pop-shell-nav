use crate::command::Command;
use crate::server::Server;
use hyprland::data::Client;
use hyprland::dispatch::{Direction, Dispatch, DispatchType};
use hyprland::prelude::*;
use std::panic;

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

    fn get_window_title(&self) -> Option<String> {
        let result = panic::catch_unwind(|| {
            let client = Client::get_active().ok()?;
            let title = client?.title;
            Some(title)
        });

        result.ok()?
    }
}
