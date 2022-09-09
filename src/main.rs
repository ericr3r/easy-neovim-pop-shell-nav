use clap::{Parser, ValueEnum};
use dbus::blocking::Connection;
use human_regex::{any, beginning, end, named_capture, one_or_more, text, zero_or_more};
use jwilm_xdo::Xdo;
use neovim_lib::{Neovim, NeovimApi, Session};
use std::error::Error;
use std::fmt;
use std::time::Duration;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(arg_enum, value_parser)]
    direction: Direction,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
enum Direction {
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

#[derive(Debug)]
struct NvimError {
    details: String,
}

impl NvimError {
    fn new(msg: &str) -> NvimError {
        NvimError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for NvimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for NvimError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let xdo = Xdo::new().expect("create xdo");
    let window = xdo.get_active_window();

    if let Ok(window) = window {
        let window_name = window.get_name();

        if let Ok(window_name) = window_name {
            if let Some(server_name) = nvim_server(&window_name) {
                match vim_navigate(server_name, cli.direction) {
                    Err(_) => return pop_shell_navigate(cli.direction),
                    _ => return Ok(()),
                }
            } else {
                pop_shell_navigate(cli.direction)?;
            }
        }
    }

    Ok(())
}

fn nvim_server<'a>(window_name: &'a str) -> Option<&'a str> {
    let regex_string = beginning()
        + one_or_more(any())
        + text("nvim")
        + one_or_more(any())
        + text("[")
        + named_capture(zero_or_more(any()), "server_name")
        + text("]")
        + zero_or_more(any())
        + end();

    let caps = regex_string.to_regex().captures(window_name)?;
    Some(caps.get(1).unwrap().as_str())
}

fn vim_navigate(server_name: &str, direction: Direction) -> Result<(), Box<dyn std::error::Error>> {
    let mut session = Session::new_unix_socket(server_name)?;
    session.start_event_loop();

    let mut nvim = Neovim::new(session);

    let old_window = nvim.get_current_win()?;
    let cmd = format!("wincmd {}", direction.to_vim_direction());
    nvim.command(&cmd)?;

    let window = nvim.get_current_win()?;

    if old_window == window {
        return Err(Box::new(NvimError::new("no window movement")));
    } else {
        return Ok(());
    }
}

fn pop_shell_navigate(direction: Direction) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(
        "org.gnome.Shell",
        "/com/System76/PopShell",
        Duration::from_millis(5000),
    );

    proxy.method_call(
        "com.System76.PopShell",
        direction.to_pop_os_method_call(),
        (),
    )?;

    Ok(())
}
