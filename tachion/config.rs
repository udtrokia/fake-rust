use clap;

pub struct Config {
    pub quiet: bool,
}

pub fn parse(matches: &clap::ArgMatches) -> Result<Config, String> {
    let quiet = matches.is_present("quiet");

    let config = Config {
        quiet: quiet,
    };

    Ok(config)
}
