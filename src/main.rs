mod command;
mod nvim;
mod pop_shell;
mod server;
mod window;

use clap::Parser;
use command::Cli;
use nvim::Nvim;
use std::process;
use window::get_window_title;

use crate::pop_shell::PopShell;
use crate::server::{Server, ServerError};
use crate::{command::is_navigation, window::directory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Cli::parse().command;

    let title = get_window_title();

    if is_navigation(command) {
        if let Some(window_name) = title {
            println!("window title: {}", window_name);

            return navigate(window_name, command);
        }
    }

    let default_path = "/home/eric/projects";

    match title {
        Some(window_name) => match directory(&window_name) {
            Some(directory) => open_terminal(directory),
            None => open_terminal(default_path),
        },
        None => open_terminal(default_path),
    }
}

fn navigate(
    window_name: String,
    command: command::Command,
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let shell = PopShell::new();
    if let Some(nvim_server) = Nvim::new(&window_name) {
        match nvim_server.navigate(command) {
            Err(_) => {
                return shell
                    .ok_or(ServerError::new("no window movement"))?
                    .navigate(command)
            }
            _ => return Ok(()),
        }
    } else {
        return shell
            .ok_or(ServerError::new("no window movement"))?
            .navigate(command);
    }
}

fn open_terminal(directory: &str) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    process::Command::new("tilix")
        .args(["-w", directory])
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .spawn()?;

    Ok(())
}
