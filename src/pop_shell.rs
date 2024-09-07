use crate::command::Command;
use crate::server::Server;
use dbus::blocking::Connection;
use jwilm_xdo::Xdo;
use std::time::Duration;

pub struct PopShell {
    conn: Connection,
}

impl PopShell {
    pub fn new() -> Option<PopShell> {
        let conn = Connection::new_session().ok()?;
        Some(PopShell { conn: conn })
    }
}

pub fn get_window_title_from_x() -> Option<String> {
    let xdo = Xdo::new().expect("create xdo");
    let window = xdo.get_active_window();

    if let Ok(window) = window {
        if let Ok(name) = window.get_name() {
            return Some(name);
        }
    }

    return None;
}

pub fn get_window_title_from_gnome() -> Result<String, Box<dyn std::error::Error>> {
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

impl<'a> Server<'a> for PopShell {
    fn navigate(&self, command: Command) -> Result<(), Box<dyn std::error::Error>> {
        let method_call = command.to_pop_os_method_call();

        let proxy = self.conn.with_proxy(
            "org.gnome.Shell",
            "/com/System76/PopShell",
            Duration::from_millis(5000),
        );

        println!("navigating {}", method_call);

        proxy.method_call("com.System76.PopShell", method_call, ())?;

        Ok(())
    }

    fn get_window_title(&self) -> Option<String> {
        if let Ok(title) = get_window_title_from_gnome() {
            return Some(title);
        }

        if let Some(title) = get_window_title_from_x() {
            return Some(title);
        }

        return None;
    }
}
