use crate::command::Command;

pub trait Server<'a> {
    fn navigate(&self, command: Command) -> Result<(), Box<dyn std::error::Error>>;
}
