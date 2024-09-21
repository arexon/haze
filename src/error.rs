use std::{env, fmt, io, path::PathBuf};

use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("could not find `{path}` in `{}`", cwd.display())]
    ConfigNotFound {
        path: String,
        cwd: PathBuf,
        source: io::Error,
    },

    #[error("could not parse `{path}` in `{}`", cwd.display())]
    ConfigFormat {
        path: String,
        cwd: PathBuf,
        source: serde_json::Error,
    },

    #[error("could not find the LOCALAPPDATA environment variable")]
    #[cfg(windows)]
    CannotFindLocalAppData { source: env::VarError },

    #[error("could not find the COM_MOJANG environment variable")]
    #[diagnostic(help("setting this variable is required on non-Windows systems"))]
    NoComMojangEnvVar { source: env::VarError },

    #[error("the `com.mojang` directory does not exist in `{}`", path.display())]
    ComMojangDoesNotExist {
        source: Option<io::Error>,
        path: PathBuf,
    },

    #[error("invalid world glob pattern `{pattern}`")]
    InvalidWorldGlob {
        source: glob::PatternError,
        pattern: String,
    },

    #[error("two local worlds have conflicting names `{}` <-> `{}`", world_a.display(), world_b.display())]
    #[diagnostic(help(
        "worlds in different directories must have unique names so they are easily identifiable"
    ))]
    LocalWorldNameConflict { world_a: PathBuf, world_b: PathBuf },

    #[error("attempting to export `{name}` when one already exists in `com.mojang`")]
    #[diagnostic(help("use --overwrite to bypass"))]
    ExportWithoutOverwriteAllowed { name: String },

    #[error("attempting to import `{name}` when there is no local world matching it")]
    #[diagnostic(help(
        "worlds must be manually imported to a desired local location for first-time setup"
    ))]
    ImportWithoutLocalMatch { name: String },

    #[error(transparent)]
    #[diagnostic(transparent)]
    NoMatchingWorlds(NoMatchingWorldsError),

    #[error("failed to access a world at `{}`", path.display())]
    WorldAccessFailure { source: io::Error, path: PathBuf },

    #[error("failed to copy world `{}` to `{}`", from.display(), to.display())]
    WorldCopyFailure {
        source: fs_extra::error::Error,
        from: PathBuf,
        to: PathBuf,
    },
}

#[derive(Debug, Error, Diagnostic)]
pub struct NoMatchingWorldsError {
    pub names: Vec<String>,
}

impl fmt::Display for NoMatchingWorldsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no worlds matching ")?;
        for (index, name) in self.names.iter().enumerate() {
            write!(f, "`{name}`")?;
            if index < self.names.len() - 1 {
                write!(f, " and ")?;
            }
        }
        write!(f, " were found")?;

        Ok(())
    }
}
