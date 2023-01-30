mod direction;
mod nvim;
mod pop_shell;

use clap::Parser;
use direction::Cli;
use jwilm_xdo::Xdo;
use nvim::{nvim_server, vim_navigate};
use pop_shell::pop_shell_navigate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let xdo = Xdo::new().expect("create xdo");
    let window = xdo.get_active_window();

    if let Ok(window) = window {
        let window_name = window.get_name();

        if let Ok(window_name) = window_name {
            if let Some(server_name) = nvim_server(&window_name) {
                match vim_navigate(server_name, cli.direction.to_vim_direction()) {
                    Err(_) => return pop_shell_navigate(cli.direction.to_pop_os_method_call()),
                    _ => return Ok(()),
                }
            } else {
                pop_shell_navigate(cli.direction.to_pop_os_method_call())?;
            }
        }
    }

    Ok(())
}
