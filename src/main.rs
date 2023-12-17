mod command;
mod nvim;
mod pop_shell;
mod server;
mod window;

use clap::Parser;
use command::Cli;
use nvim::Nvim;
use pop_shell::pop_shell_navigate;
use std::process;
use window::get_window_title;

use crate::server::Server;
use crate::{command::is_navigation, window::directory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Cli::parse().command;

    if is_navigation(command) {
        if let Some(window_name) = get_window_title() {
            println!("window title: {}", window_name);

            return navigate(window_name, command);
        }
    }

    let default_path = "/home/eric/projects";

    match get_window_title() {
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
    if let Some(nvim_server) = Nvim::new(&window_name) {
        match nvim_server.navigate(command) {
            Err(_) => return pop_shell_navigate(command.to_pop_os_method_call()),
            _ => return Ok(()),
        }
    } else {
        pop_shell_navigate(command.to_pop_os_method_call())?;
    }

    Ok(())
}

fn open_terminal(directory: &str) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    process::Command::new("tilix")
        .args(["-w", directory])
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .spawn()?;

    Ok(())
}
