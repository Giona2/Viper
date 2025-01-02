use colored::Colorize;
use utils::advanced_error_handling::*;


pub mod workshop;
pub mod project;
pub mod content;

pub struct Commands {
	pub error_handler: ErrorHandler,
} impl Commands {
	pub fn _new() -> Commands { Commands {
		error_handler: ErrorHandler::new(),
	}}

	pub fn help(&self, ) {
        println!("--<{}>   = tags",      "blue".blue());
		println!("  <{}> = arguments", "yellow".yellow());
        println!();
		println!("viper new <{}> --<{}>", "project name".yellow(), "main.py layout".blue());
		println!("  creates a new python project with a virtual environment and a main.py file. You can pass in a few arguments to get different main.py layouts");
        println!("    -e | --entry-point: Creates the main.py in the entry point layout");
        println!("    -c | --class      : Creates the main.py in the object-oriented layout");
        println!("    -s | --simple     : Creates the main.py in the most simplistic layout as possible");
        println!();
		println!("viper run");
		println!("  if in a folder created by viper new, it will run the main.py file sourcing the local virtual environment");
        println!();
		println!("viper list");
		println!("  if in a folder created by viper new, it will return the list of installed packages in the local virtual environment");
        println!("  based on the pip3 freeze command");
        println!();
		println!("viper install <{}>", "package name".yellow());
		println!("  if in a folder created by viper new, it will install the specified package to the local virtual environment");
        println!("  based on the pip3 install command");
        println!();
		println!("viper remove <{}>", "package name".yellow());
		println!("  if in a folder created by viper new, it will remove the specified package from the local virtual environment");
        println!("  based on the pip3 uninstall command");

	}
}

#[derive(Debug, thiserror::Error)]
enum Error {
	#[error("Improper characters were used")]
	ImproperCharactersUsed,
	#[error("The user needs to run this command in a python project")]
	NotInProjectDirectory,
}
