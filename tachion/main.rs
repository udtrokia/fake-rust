#[macro_use]
extern crate clap;
extern crate env_logger;

extern crate logs;

mod config;

pub const LOG_INFO: &'static str = "sync=info";

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}

pub fn run() -> Result<(), String> {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    let cfg = try!(config::parse(&matches));

    if !cfg.quiet {
	if cfg!(windows) {
	    logs::init(LOG_INFO, logs::DateLogFormatter);
	} else {
	    logs::init(LOG_INFO, logs::DateAndColorLogFormatter);
	}
    } else {
	env_logger::init();
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
            Ok(())
        } else {
            println!("Printing normally...");
            Ok(())
        }
    } else {
        println!("USAGE: tachion [Flags] [SUBCOMMAND]");
        Ok(())
    }
}
