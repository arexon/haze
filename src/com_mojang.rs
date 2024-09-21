use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};

const MC_WORLDS: &str = "minecraftWorlds";

#[cfg(windows)]
#[derive(Clone)]
pub enum MinecraftVersion {
    Stable,
    Preview,
    Education,
}

#[cfg(windows)]
impl MinecraftVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Preview => "preview",
            Self::Education => "education",
        }
    }
}

#[cfg(unix)]
pub fn get_and_check() -> Result<PathBuf> {
    let path = from_env()?;
    check_if_exists(&path)?;
    Ok(path)
}

#[cfg(windows)]
pub fn get_and_check(version: &MinecraftVersion) -> Result<PathBuf> {
    let path = from_env().or_else(|_| from_version(version))?;
    check_if_exists(&path)?;
    Ok(path)
}

#[cfg(windows)]
pub fn from_version(version: &MinecraftVersion) -> Result<PathBuf> {
    let version = match version {
        MinecraftVersion::Stable => "UWP",
        MinecraftVersion::Preview => "Beta",
        MinecraftVersion::Education => "EducationEdition",
    };
    let appdata_var =
        env::var("LOCALAPPDATA").map_err(|source| Error::CannotFindLocalAppData { source })?;
    let com_mojang = PathBuf::from(appdata_var)
        .join("Packages")
        .join(format!("Microsoft.Minecraft{version}_8wekyb3d8bbwe"))
        .join("LocalState")
        .join("games")
        .join("com.mojang")
        .join(MC_WORLDS);

    Ok(com_mojang)
}

pub fn from_env() -> Result<PathBuf> {
    let com_mojang_var =
        env::var("COM_MOJANG").map_err(|source| Error::NoComMojangEnvVar { source })?;
    Ok(PathBuf::from(com_mojang_var).join(MC_WORLDS))
}

pub fn check_if_exists(dir: &Path) -> Result<()> {
    match fs::exists(dir) {
        Ok(true) => Ok(()),
        Ok(false) => Err(Error::ComMojangDoesNotExist {
            source: None,
            path: dir.to_path_buf(),
        }),
        Err(source) => Err(Error::ComMojangDoesNotExist {
            source: Some(source),
            path: dir.to_path_buf(),
        }),
    }
}
