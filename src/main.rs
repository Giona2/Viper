use std::env::args;

mod commands;
use commands::{Commands, workshop::*, project::*};
mod error_handler;
mod data_file_parsing;
mod pip_frontend;
mod data;


fn main() {
	let args: Vec<String> = args().collect();

	let commands = Commands::_new();
	match args[1].as_str() {
		"new"		 => commands.new(args[2..args.len()].to_vec()),
        
		"run"		 => commands.run(),
        "reload"     => commands.reload(),
        "search"     => commands.search(&args[2]),

		"help" | "h" => commands.help(),
		_ => {}
	}
}
