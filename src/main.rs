mod command;
mod nvim;
mod pop_shell;
mod window;

use clap::Parser;
use command::Cli;
use nvim::vim_navigate;
use pop_shell::pop_shell_navigate;
use std::process;
use window::{get_window_title, nvim_server};

use crate::{command::is_navigation, window::directory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Cli::parse().command;

    if let Some(window_name) = get_window_title() {
        println!("window title: {}", window_name);

        if is_navigation(command) {
            return navigate(window_name, command);
        }

        if let Some(directory) = directory(&window_name) {
            return open_terminal(directory);
        }
    }

    Ok(())
}

fn navigate(
    window_name: String,
    command: command::Command,
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    if let Some(server_name) = nvim_server(&window_name) {
        match vim_navigate(server_name, command.to_vim_direction()) {
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
