use std::io;

use std::process;


pub trait IOLibHandler<T> {
    fn handle(self, subject_name: &str) -> T;

} impl<T> IOLibHandler<T> for Result<T, io::Error> {
    fn handle(self, subject_name: &str) -> T { match self {
        Ok(val)  => { return val }
        Err(err) => { match err.kind() {
            io::ErrorKind::NotFound         => { println!("{subject_name} could not be found") }
            io::ErrorKind::IsADirectory     => { println!("{subject_name} is a directory, not a file") }
            io::ErrorKind::AlreadyExists    => { println!("{subject_name} already exists") }
            io::ErrorKind::NotADirectory    => { println!("{subject_name} is a file, not a directory") }
            io::ErrorKind::PermissionDenied => { println!("You do not have permission to access {subject_name}") }
                                          _ => { println!("Unknown/Unspecified error occurred while operating on {subject_name}") }
        }; process::exit(1);}
    }}
}
