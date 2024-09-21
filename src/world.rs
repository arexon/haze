use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

use color_print::cstr;
use fs_extra::dir::{self, CopyOptions};
use walkdir::WalkDir;

use crate::error::{Error, NoMatchingWorldsError, Result};

pub type LocalWorldMap = HashMap<String, PathBuf>;
pub type ComMojangWorldSet = HashMap<String, ()>;

/// Holds info about local and `com.mojang` worlds.
pub struct WorldManager {
    local_worlds: LocalWorldMap,
    com_mojang_worlds: ComMojangWorldSet,
    com_mojang: PathBuf,
}

impl WorldManager {
    pub fn new(patterns: Vec<String>, com_mojang: PathBuf) -> Result<Self> {
        let local_worlds = patterns
            .clone()
            .into_iter()
            .map(|pattern| {
                glob::glob(&pattern).map_err(|source| Error::InvalidWorldGlob { source, pattern })
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .try_fold(
                HashMap::new(),
                |mut worlds, path| -> Result<LocalWorldMap> {
                    let path = path.map_err(|e| Error::WorldAccessFailure {
                        path: e.path().to_path_buf(),
                        source: e.into_error(),
                    })?;
                    let name = world_name_from_path(&path);
                    match worlds.get(&name) {
                        Some(old_path) => {
                            return Err(Error::LocalWorldNameConflict {
                                world_a: path,
                                world_b: old_path.clone(),
                            });
                        }
                        None => worlds.insert(name, path),
                    };
                    Ok(worlds)
                },
            )?;

        let com_mojang_worlds = WalkDir::new(&com_mojang)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|entry| match entry {
                Ok(entry) if entry.file_type().is_dir() => {
                    Some(Ok((world_name_from_path(entry.path()), ())))
                }
                Ok(_) => None,
                Err(err) => match err.io_error() {
                    Some(_) => Some(Err(Error::WorldAccessFailure {
                        path: err.path()?.to_path_buf(),
                        source: err.into_io_error().unwrap(),
                    })),
                    None => None,
                },
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            local_worlds,
            com_mojang_worlds,
            com_mojang,
        })
    }

    /// Sequentially exports the given local worlds to `com.mojang`.
    pub fn export(mut self, names: Vec<String>, overwrite: bool) -> Result<()> {
        let names = HashSet::<String>::from_iter(names);
        let names_not_found: Vec<_> = names
            .iter()
            .filter(|&name| (!self.local_worlds.contains_key(name)))
            .cloned()
            .collect();

        if !names_not_found.is_empty() {
            return Err(Error::NoMatchingWorlds(NoMatchingWorldsError {
                names: names_not_found,
            }));
        }

        for name in names {
            // We've already checked that `name` *does* exist in `local_worlds`.
            let from = self.local_worlds.remove(&name).unwrap();
            let to = self.com_mojang.join(&name);

            match (self.com_mojang_worlds.contains_key(&name), overwrite) {
                // 1. Target world does exist and we can delete it before copying.
                (true, true) => {
                    fs::remove_dir_all(&to).map_err(|source| Error::WorldAccessFailure {
                        source,
                        path: to.to_path_buf(),
                    })?;
                    copy_world(&from, &to)?;
                }
                // 2. Target world does exist, but we cannot overwrite it.
                (true, false) => return Err(Error::ExportWithoutOverwriteAllowed { name }),
                // 3. Target world does not exist, we can copy normally.
                _ => copy_world(&from, &to)?,
            }

            log::info!("exported `{}` to `{}`", from.display(), to.display());
        }

        Ok(())
    }

    /// Sequentially imports the given worlds from `com.mojang` and stores them
    /// locally.
    pub fn import(mut self, names: Vec<String>) -> Result<()> {
        let names = HashSet::<String>::from_iter(names);
        let names_not_found: Vec<_> = names
            .iter()
            .filter(|&name| (!self.com_mojang_worlds.contains_key(name)))
            .cloned()
            .collect();

        if !names_not_found.is_empty() {
            return Err(Error::NoMatchingWorlds(NoMatchingWorldsError {
                names: names_not_found,
            }));
        }

        for name in names {
            // We've already checked that `name` *does* exist in `com.mojang`.
            let (from, _) = self.com_mojang_worlds.remove_entry(&name).unwrap();
            let from = self.com_mojang.join(from);
            let to = self
                .local_worlds
                .remove(&name)
                .ok_or_else(|| Error::ImportWithoutLocalMatch { name })?;

            fs::remove_dir_all(&to).map_err(|source| Error::WorldAccessFailure {
                source,
                path: to.to_path_buf(),
            })?;
            copy_world(&from, &to)?;

            log::info!("imoprted `{}` to `{}`", from.display(), to.display());
        }

        Ok(())
    }

    /// List worlds stored locally and in `com.mojang`.
    pub fn list(self) -> Result<()> {
        let mut output = String::new();

        let has_local_worlds = !self.local_worlds.is_empty();
        let has_com_mojang_worlds = !self.com_mojang_worlds.is_empty();

        if has_local_worlds {
            writeln!(
                output,
                cstr!("<y>{}--</> <s>local project</>"),
                if has_com_mojang_worlds { '|' } else { '`' }
            )
            .unwrap();
            for (index, path) in self.local_worlds.values().enumerate() {
                let is_last = self.local_worlds.len() - 1 == index;
                write!(
                    output,
                    cstr!("<y>{}   {}--</> {}"),
                    if has_com_mojang_worlds { '|' } else { ' ' },
                    if is_last { '`' } else { '|' },
                    path.display()
                )
                .unwrap();
                if !is_last || has_com_mojang_worlds {
                    writeln!(output).unwrap();
                }
            }
        }

        if has_com_mojang_worlds {
            writeln!(output, cstr!("<y>`--</> <s>com.mojang</>")).unwrap();
            for (index, path) in self.com_mojang_worlds.keys().enumerate() {
                let is_last = self.com_mojang_worlds.len() - 1 == index;
                write!(
                    output,
                    cstr!("<y>    {}--</> {}"),
                    if is_last { '`' } else { '|' },
                    path
                )
                .unwrap();
                if !is_last {
                    writeln!(output).unwrap();
                }
            }
        }

        log::info!("listing all worlds at..\n{output}");

        Ok(())
    }
}

fn copy_world(from: &Path, to: &Path) -> Result<()> {
    let options = CopyOptions::new().content_only(true);
    dir::copy(from, to, &options).map_err(|source| Error::WorldCopyFailure {
        source,
        from: from.to_path_buf(),
        to: to.to_path_buf(),
    })?;
    Ok(())
}

fn world_name_from_path(path: &Path) -> String {
    path.file_name().unwrap().to_string_lossy().to_string()
}
