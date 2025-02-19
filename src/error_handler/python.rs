use std::io;

use std::process::{self, ExitStatus};


pub trait PythonHandler {
    fn python_handle(self);

} impl PythonHandler for Result<ExitStatus, io::Error> {
    fn python_handle(self) { if let Ok(status) = self { match status.code() {
        Some(0)   => return,
        Some(128) => {println!("python: invalid exit argument was used"); process::exit(1)}
        Some(130) => {println!("python: program was force terminated");   process::exit(1)}
        Some(137) => {println!("python: program was killed");             process::exit(1)}
        Some(149) => {println!("python: program was killed");             process::exit(1)}
        Some(139) => {println!("python: found a segmentation fault");     process::exit(1)}
        None      => process::exit(1),
                _ => process::exit(1),
    }} else { process::exit(1) }}
}
