use anyhow::{bail, Result};
use clap::Parser;
use rand::prelude::*;
use rand::thread_rng;
use std::env;
use std::path::PathBuf;

use crate::prelude::*;

mod prelude;
mod utils;

// Valid image file extensions (e.g. ".png" or ".jpg")
pub const VALID_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];
// Time to wait if a guess was close enough
pub const CLOSE_ENOUGH_DURATION_MS: u64 = 5000;
// Mega Pokemon suffix (e.g. aggron-mega)
// pub const MEGA_SUFFIX: &str = "-mega";

mod path_names {
    pub const ASSETS_POKEMON: &str = "assets-pokemon";
    pub const POKEMON_GEN7: &str = "gen7";
    // pub const POKEMON_FEMALE: &str = "female";
}

#[derive(Debug, Parser)]
pub struct Args {
    /// Use exact form names (e.g. aegislash-blade)
    #[clap(short, long, required = false)]
    pub exact_names: bool,
    /// Max number of attempts
    #[clap(short, long, default_value_t = 3)]
    pub attempts: usize,
    /// Milliseconds to wait after correctly guessing or failing every time
    #[clap(short, long, default_value_t = 2000)]
    pub delay: u64,
    /// Number of pokemon to go through
    #[clap(short, long, default_value_t = 10)]
    pub num_guesses: usize,
}

fn main() -> Result<()> {
    let exe_parent: PathBuf = {
        let current_exe: PathBuf = env::current_exe().unwrap();
        current_exe.parent().unwrap().to_path_buf()
    };
    let mut pokemon_assets: Vec<PokemonAsset> = {
        let mut path = exe_parent;
        path.push(path_names::ASSETS_POKEMON);
        path.push(path_names::POKEMON_GEN7);

        path.read_dir()?
            .filter_map(Result::ok)
            .filter_map(|asset_entry| PokemonAsset::try_from(&asset_entry.path()).ok())
            .collect()
    };

    if pokemon_assets.is_empty() {
        bail!(
            "Your {}/{} directory is empty! Check if the images have the correct file extensions:\n{:?}",
            path_names::ASSETS_POKEMON,
            path_names::POKEMON_GEN7,
            VALID_EXTENSIONS
        );
    }

    pokemon_assets.shuffle(&mut thread_rng());

    let cli = Args::parse();
    utils::start_game(&pokemon_assets, &cli)?;

    Ok(())
}
