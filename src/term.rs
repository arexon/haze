use std::io::Write;

use color_print::cstr;
use env_logger::Env;
use log::Level;
use miette::{GraphicalTheme, MietteHandlerOpts, ThemeCharacters};

pub fn init_miette() {
    miette::set_hook(Box::new(|_| {
        Box::new(
            MietteHandlerOpts::new()
                .graphical_theme(GraphicalTheme {
                    characters: ThemeCharacters::ascii(),
                    ..Default::default()
                })
                .build(),
        )
    }))
    .expect("should set miette hook")
}

pub fn init_logger() {
    env_logger::Builder::from_env(Env::new().filter_or("HAZE_LOG", "info"))
        .format(|buf, record| match record.level() {
            Level::Error => write!(buf, cstr!("<r,s>error:</> {}"), record.args()),
            Level::Warn => todo!(),
            Level::Info => {
                writeln!(buf, cstr!("<s,c>info:</> {}"), record.args())
            }
            Level::Debug => writeln!(buf, cstr!("<dim,s>debug:</> {}"), record.args()),
            Level::Trace => unimplemented!(),
        })
        .init();
}
