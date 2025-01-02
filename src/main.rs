use std::env::args;

mod commands;
use commands::{Commands, workshop::*, project::*};
mod config_file;
mod error_handler;
mod utils;


fn main() {
	let args: Vec<String> = args().collect();

	let commands = Commands::_new();
	match args[1].as_str() {
		"new"		 => commands.new(args[2..args.len()].to_vec()),
        
		"run"		 => commands.run(),
		"list"		 => commands.list(),
		"install"	 => commands.install(args[2..args.len()].to_vec()),
		"remove"	 => commands.remove(args[2..args.len()].to_vec()),

		"help" | "h" => commands.help(),
		_ => {}
	}
}
