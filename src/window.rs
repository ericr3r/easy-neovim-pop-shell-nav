use dbus::blocking::Connection;
use human_regex::{
    any, beginning, end, named_capture, non_whitespace, one_or_more, text, whitespace, zero_or_more,
};
use jwilm_xdo::Xdo;
use regex::Regex;
use std::time::Duration;

pub fn get_window_title() -> Option<String> {
    if let Ok(title) = get_window_title_from_gnome() {
        return Some(title);
    }

    if let Some(title) = get_window_title_from_x() {
        return Some(title);
    }

    return None;
}

fn get_window_title_from_x() -> Option<String> {
    let xdo = Xdo::new().expect("create xdo");
    let window = xdo.get_active_window();

    if let Ok(window) = window {
        if let Ok(name) = window.get_name() {
            return Some(name);
        }
    }

    return None;
}

fn get_window_title_from_gnome() -> Result<String, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(
        "org.gnome.Shell",
        "/org/gnome/Shell/Extensions/Windows",
        Duration::from_millis(5000),
    );

    let (title,): (String,) =
        proxy.method_call("org.gnome.Shell.Extensions.Windows", "GetFocusedTitle", ())?;
    return Ok(title);
}

fn nvim_regex() -> Regex {
    let regex_string = beginning()
        + one_or_more(any())
        + text("nvim")
        + zero_or_more(whitespace())
        + named_capture(one_or_more(non_whitespace()), "directory")
        + zero_or_more(any())
        + zero_or_more(whitespace())
        + zero_or_more(any())
        + text("[")
        + named_capture(zero_or_more(any()), "server_name")
        + text("]")
        + zero_or_more(any())
        + end();

    regex_string.to_regex()
}

pub fn nvim_server<'a>(window_name: &'a str) -> Option<&'a str> {
    let caps = nvim_regex().captures(window_name)?;
    let server_name = caps.name("server_name")?.as_str();
    Some(server_name)
}

pub fn directory<'a>(window_name: &'a str) -> Option<&'a str> {
    if let Some(directory) = directory_from_nvim_title(window_name) {
        return Some(directory);
    }

    None
}

fn directory_from_nvim_title<'a>(window_name: &'a str) -> Option<&'a str> {
    let caps = nvim_regex().captures(window_name)?;
    let directory = caps.name("directory")?.as_str();
    Some(directory)
}
