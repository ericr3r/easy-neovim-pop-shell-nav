use crate::command::Command;
use crate::server::Server;
use dbus::blocking::Connection;
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
}
