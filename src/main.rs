mod direction;
mod nvim;
mod pop_shell;
mod window;

use clap::Parser;
use direction::Cli;
use nvim::{nvim_server, vim_navigate};
use pop_shell::pop_shell_navigate;
use window::get_window_title;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(window_name) = get_window_title() {
        if let Some(server_name) = nvim_server(&window_name) {
            match vim_navigate(server_name, cli.direction.to_vim_direction()) {
                Err(_) => return pop_shell_navigate(cli.direction.to_pop_os_method_call()),
                _ => return Ok(()),
            }
        } else {
            pop_shell_navigate(cli.direction.to_pop_os_method_call())?;
        }
    }

    Ok(())
}
