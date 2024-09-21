use std::{env, fs};

use json_strip_comments::CommentSettings;
use serde::Deserialize;

use crate::error::{Error, Result};

#[derive(Deserialize)]
pub struct Config {
    pub worlds: Vec<String>,
}

impl Config {
    pub fn load(path: String) -> Result<Self> {
        let mut content = fs::read_to_string(&path).map_err(|source| Error::ConfigNotFound {
            path: path.clone(),
            cwd: env::current_dir().unwrap(),
            source,
        })?;

        json_strip_comments::strip_comments_in_place(
            &mut content,
            CommentSettings::c_style(),
            true,
        )
        .unwrap();

        let config = serde_json::from_str(&content).map_err(|source| Error::ConfigFormat {
            path,
            cwd: env::current_dir().unwrap(),
            source,
        })?;

        Ok(config)
    }
}
