use std::env::args;

mod commands;
use commands::{Commands, workshop::*, project::*};
mod error_handler;
mod data_file_parsing;
mod pip_frontend;
mod data;


fn _main() {
	let args: Vec<String> = args().collect();

	let commands = Commands::_new();
	match args[1].as_str() {
		"new"		 => commands.new(args[2..args.len()].to_vec()),
        
		"run"		 => commands.run(),
		"list"		 => commands.list(),
        "reload"     => commands.reload(),

		"help" | "h" => commands.help(),
		_ => {}
	}
}


const WEBSITE: &str = 
r#"
<body>
    <ul>
        <li class="meh">hey</li>
        <li>hello</li>
    </ul>
</body>
"#;

use pip_frontend::PipFrontend;
use std::fs;
use scraper::Html;
use scraper::Selector;
fn main() {
    //let td_selector = Selector::parse("table.table tbody tr td").unwrap();
    //let doc = Html::parse_document(&fs::read_to_string("./output.log").unwrap());
    //let tds: Vec<_> = doc.select(&td_selector).collect();
    //for (i, td) in tds.iter().enumerate() {
    //    println!("{}", td.inner_html());
    //    if (i + 1) % 4 == 0 { println!("END\n") };
    //}
    let pf = PipFrontend::new();
    for package in pf.search("") {
        println!("{:?}", package);
    }
}
