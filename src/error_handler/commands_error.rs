use crate::commands::commands_error::CommandsError;

use std::process;


pub trait CommandsErrorHandler<T> {
    fn handle(self) -> T; 

} impl<T> CommandsErrorHandler<T> for Result<T, CommandsError> {
    fn handle(self) -> T { match self {
        Ok(val)  => { return val }
        Err(err) => { match err {
            CommandsError::NotInProjectDirectory  => { println!("You are not in a project diretory") }
            CommandsError::ImproperCharactersUsed => { println!("Improper characters were used") }
        }; process::exit(1)}
    }}
}
