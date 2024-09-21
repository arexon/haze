mod cli;
mod com_mojang;
mod config;
mod error;
mod term;
mod world;

use std::process;

use clap::Parser;
use miette::Result;

use crate::{
    cli::{Cli, Commands},
    config::Config,
    world::WorldManager,
};

fn main() {
    let cli = Cli::parse();
    term::init_logger();
    term::init_miette();

    let run = || -> Result<()> {
        let config = Config::load(cli.config)?;

        #[cfg(unix)]
        let com_mojang = com_mojang::get_and_check()?;
        #[cfg(windows)]
        let com_mojang = com_mojang::get_and_check(&cli.minecraft_version.0)?;

        let haze = WorldManager::new(config.worlds, com_mojang)?;
        match cli.commands {
            Commands::Export { names, overwrite } => haze.export(names, overwrite)?,
            Commands::Import { names } => haze.import(names)?,
            Commands::List => haze.list()?,
        }

        Ok(())
    };

    match run() {
        Ok(_) => process::exit(0),
        Err(error) => {
            let error = format!("{error:?}");
            // Trim the initial whitespace.
            log::error!("{}", &error[13..]);
            process::exit(1);
        }
    }
}
