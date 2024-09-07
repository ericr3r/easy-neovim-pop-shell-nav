use crate::command::Command;
use crate::server::Server;
use std::cell::RefCell;
use swayipc::Connection;
use swayipc::Node;

#[derive(Debug)]
pub struct Sway {
    conn: RefCell<Connection>,
}

impl Sway {
    pub fn new() -> Option<Sway> {
        let conn = Connection::new().ok()?;
        Some(Sway {
            conn: RefCell::new(conn),
        })
    }
}

// Recursively find the focused node in the tree
fn find_focused(node: &Node) -> Option<&Node> {
    if node.focused {
        return Some(node);
    }

    for child in &node.nodes {
        if let Some(focused) = find_focused(child) {
            return Some(focused);
        }
    }

    None
}

impl<'a> Server<'a> for Sway {
    fn navigate(&self, command: Command) -> Result<(), Box<dyn std::error::Error>> {
        let msg = match command {
            Command::Up => "focus up",
            Command::Down => "focus down",
            Command::Left => "focup left",
            _ => "focus right",
        };

        self.conn.borrow_mut().run_command(msg)?;

        Ok(())
    }

    fn get_window_title(&self) -> Option<String> {
        let tree = self.conn.borrow_mut().get_tree().ok()?;

        find_focused(&tree)?.name.clone()
    }
}
