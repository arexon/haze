use anstyle::{AnsiColor, Color, Style};
#[cfg(windows)]
use clap::ValueEnum;
use clap::{builder, Parser, Subcommand};

#[cfg(windows)]
use crate::com_mojang::MinecraftVersion;

#[derive(Parser)]
#[command(author, version, about, long_about = None, styles=get_styles())]
pub struct Cli {
    /// Set a path to the config file
    #[arg(short, long, value_name = "PATH", default_value = "config.json")]
    pub config: String,

    /// The Minecraft version to get the `com.mojang` directory from. To define
    /// an arbitrary path, set the `COM_MOJANG` environment variable instead
    #[cfg(windows)]
    #[arg(short = 'm', long, value_enum, default_value = "stable")]
    pub minecraft_version: MinecraftVersion,

    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Copy local worlds to `com.mojang`
    #[clap(visible_alias("ex"))]
    Export {
        /// The name of one or more worlds to export
        #[arg(required = true)]
        names: Vec<String>,
        /// Overwrite any already existing worlds in `com.mojang`
        #[arg(short, long)]
        overwrite: bool,
    },

    /// Copy `com.mojang` worlds to local worlds
    #[clap(visible_alias("im"))]
    Import {
        /// The name of one or more worlds to import
        #[arg(required = true)]
        names: Vec<String>,
    },

    /// List all worlds stored locally and in `com.mojang`.
    #[clap(visible_alias("ls"))]
    List,
}

#[cfg(windows)]
impl ValueEnum for MinecraftVersion {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Stable, Self::Preview, Self::Education]
    }

    fn to_possible_value(&self) -> Option<builder::PossibleValue> {
        Some(builder::PossibleValue::new(self.as_str()))
    }
}

fn get_styles() -> builder::Styles {
    let error_style = Style::new()
        .bold()
        .fg_color(Some(Color::Ansi(AnsiColor::Red)));

    let heading_style = Style::new()
        .bold()
        .fg_color(Some(Color::Ansi(AnsiColor::BrightMagenta)));
    let literal_style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan)));

    builder::Styles::styled()
        .usage(heading_style)
        .header(heading_style)
        .literal(literal_style.bold())
        .placeholder(literal_style)
        .invalid(error_style)
        .error(error_style)
        .valid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
}
