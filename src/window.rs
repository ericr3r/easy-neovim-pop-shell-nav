use crate::nvim::nvim_regex;

use human_regex::{any, beginning, end, named_capture, non_whitespace, one_or_more, text};

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
