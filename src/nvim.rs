use human_regex::{any, beginning, end, named_capture, one_or_more, text, zero_or_more};
use neovim_lib::{Neovim, NeovimApi, Session};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NvimError {
    details: String,
}

impl NvimError {
    pub fn new(msg: &str) -> NvimError {
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

pub fn nvim_server<'a>(window_name: &'a str) -> Option<&'a str> {
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

pub fn vim_navigate(server_name: &str, direction: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut session = Session::new_unix_socket(server_name)?;
    session.start_event_loop();

    let mut nvim = Neovim::new(session);

    let old_window = nvim.get_current_win()?;
    let cmd = format!("wincmd {}", direction);
    nvim.command(&cmd)?;

    let window = nvim.get_current_win()?;

    if old_window == window {
        return Err(Box::new(NvimError::new("no window movement")));
    } else {
        return Ok(());
    }
}
