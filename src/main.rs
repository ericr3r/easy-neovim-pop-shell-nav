mod command;
mod hypr;
mod nvim;
mod pop_shell;
mod server;
mod sway;
mod window;

use clap::Parser;
use command::Cli;
use nvim::Nvim;
use std::process;

use crate::hypr::Hypr;
use crate::pop_shell::PopShell;
use crate::server::Server;
use crate::sway::Sway;
use crate::{command::is_navigation, command::Backend, window::directory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("{args:?}");

    let command = args.command;

    let server: Box<dyn Server> = match args.backend {
        Backend::Hyprland => Box::new(Hypr::new()) as Box<dyn Server>,
        Backend::Sway => Box::new(Sway::new().ok_or("sway backend failed")?) as Box<dyn Server>,
        _ => Box::new(PopShell::new().ok_or("pop shell backend failed")?) as Box<dyn Server>,
    };

    let title = server.get_window_title();

    if is_navigation(command) {
        if let Some(window_name) = title {
            println!("window title: {}", window_name);

            return navigate(server, window_name, command);
        }
    }

    let default_path = String::from("/home/eric/projects");

    match title {
        Some(window_name) => match directory(&window_name) {
            Some(directory) => open_terminal(directory),
            None => open_terminal(default_path),
        },
        None => open_terminal(default_path),
    }
}

fn navigate(
    server: Box<dyn Server>,
    window_name: String,
    command: command::Command,
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    if let Some(nvim_server) = Nvim::new(&window_name) {
        match nvim_server.navigate(command) {
            Err(_) => return server.navigate(command),
            _ => return Ok(()),
        }
    } else {
        return server.navigate(command);
    }
}

fn open_terminal(directory: String) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    process::Command::new("ghostty")
        .args([format!("--working-directory={}", directory)])
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .spawn()?;

    Ok(())
}
