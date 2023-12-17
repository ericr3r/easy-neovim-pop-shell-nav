use crate::nvim::nvim_regex;
use dbus::blocking::Connection;
use hyprland::data::Client;
use hyprland::prelude::*;
use jwilm_xdo::Xdo;
use std::panic;
use std::time::Duration;

use human_regex::{any, beginning, end, named_capture, non_whitespace, one_or_more, text};

pub fn get_window_title() -> Option<String> {
    if let Some(title) = get_window_title_from_hyprland() {
        return Some(title);
    }

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
        "/org/gnome/Shell/Extensions/WindowsExt",
        Duration::from_millis(5000),
    );

    let (title,): (String,) =
        proxy.method_call("org.gnome.Shell.Extensions.WindowsExt", "FocusTitle", ())?;
    return Ok(title);
}

fn get_window_title_from_hyprland() -> Option<String> {
    let result = panic::catch_unwind(|| {
        let client = Client::get_active().ok()?;
        let title = client?.title;
        Some(title)
    });

    result.ok()?
}

pub fn directory<'a>(window_name: &'a str) -> Option<&'a str> {
    if let Some(directory) = directory_from_nvim_title(window_name) {
        return Some(directory);
    }

    if let Some(directory) = directory_from_terminal_title(window_name) {
        return Some(directory);
    }

    None
}

fn directory_from_nvim_title<'a>(window_name: &'a str) -> Option<&'a str> {
    let caps = nvim_regex().captures(window_name)?;
    let directory = caps.name("directory")?.as_str();
    Some(directory)
}

fn directory_from_terminal_title<'a>(window_name: &'a str) -> Option<&'a str> {
    let regex_string = beginning()
        + one_or_more(any())
        + text("@")
        + one_or_more(any())
        + text(":")
        + named_capture(one_or_more(non_whitespace()), "directory")
        + end();

    let caps = regex_string.to_regex().captures(window_name)?;
    let directory = caps.name("directory")?.as_str();
    Some(directory)
}
