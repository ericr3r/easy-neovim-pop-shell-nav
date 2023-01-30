use dbus::blocking::Connection;
use std::time::Duration;

pub fn pop_shell_navigate(method_call: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(
        "org.gnome.Shell",
        "/com/System76/PopShell",
        Duration::from_millis(5000),
    );

    proxy.method_call("com.System76.PopShell", method_call, ())?;

    Ok(())
}
