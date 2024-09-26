use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::exit;
use rand::seq::SliceRandom;
use log::{error, info};

pub fn set_wallpaper(theme: &str, wallpaper_dir: &str) -> Result<(), io::Error> {
    let theme_path = Path::new(wallpaper_dir).join(theme);

    if !theme_path.exists() {
        error!("The '{}' theme doesn't exist.", theme);
        exit(1);
    };

    let images: Vec<PathBuf> = fs::read_dir(&theme_path)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() { Some(path) } else { None }
        })
        .collect();

    if images.is_empty() {
        error!("No wallpapers found for '{}' theme. Using 'generic' by default.", theme);
        return set_wallpaper("generic", wallpaper_dir);
    };

    let chosen_image = images.choose(&mut rand::thread_rng())
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "No wallpapers found."))?;

    Command::new("swww")
        .args(&["img", chosen_image.to_str().unwrap(), "--transition-fps=60"])
        .status()
        .expect("Failed to change wallpaper.");
    info!("Wallpaper changed successfully with '{}' theme", theme);

    Ok(())
}

