use dbus::blocking::Connection;
use jwilm_xdo::Xdo;
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
