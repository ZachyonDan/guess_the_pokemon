use anyhow::{bail, Result};
use std::path::Path;

use crate::VALID_EXTENSIONS;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct PokemonAsset {
    // We don't need to mutate the name or path
    // which is why they're Box pointers, not String and PathBuf
    pub name: Box<str>,
    pub path: Box<Path>,
}

impl PokemonAsset {
    pub fn try_from(path: &Path) -> Result<Self> {
        // Verify path
        if !path.exists() {
            bail!("PokemonAsset path does not exist: {path:?}")
        } else if !path.is_file() {
            bail!("PokemonAsset path is not a directory: {path:?}")
        }

        // Verify file name
        let file_name: Box<str> = if let Some(stem) = path.file_stem() {
            // Use stem (part before extension)
            stem.to_str().unwrap().into()
        } else {
            bail!("Could not get file name for PokemonAsset: {path:?}")
        };

        // Validate file extension
        let extension: &str = if let Some(ext) = path.extension() {
            ext.to_str().unwrap()
        } else {
            bail!("Could not get file extension for PokemonAsset: {path:?}")
        };

        if VALID_EXTENSIONS.contains(&extension) {
            Ok(Self {
                name: file_name,
                path: path.into(),
            })
        } else {
            bail!("PokemonAsset file does not a have a valid extension: {file_name}.{extension}")
        }
    }
    pub fn name_stem(&self) -> Box<str> {
        let args: Vec<&str> = self.name.split('-').collect();
        if args[0] == "tapu" {
            // These aren't the same pokemon
            self.name.clone()
        } else {
            args[0].into()
        }
    }
    // pub fn is_mega(&self) -> bool {
    //     self.name.contains(MEGA_SUFFIX)
    // }
}
