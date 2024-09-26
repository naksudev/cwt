mod theme;
mod wallpaper;
mod utils;
mod config;

use clap::{Arg, Command as ClapCommand};
use log::error;
use std::process::exit;
use std::time::Duration;
use std::thread;
use config::{load_config, save_config};

fn main() {
    // Init logging
    utils::init_logging();

    // Loading user configuration
    let mut config = match load_config() {
        Ok(cfg) => {
            println!("Loaded configuration: {:?}", cfg);
            cfg
        }

        Err(e) => {
            error!("Failed to load config: {}", e);
            exit(1);
        }
    };

    // Set up CLI args parsing
    let command = ClapCommand::new("Change Wallpaper Theme (with swww)")
        .version("1.0")
        .author("naksudev")
        .about("Change wallpaper themes using swww")
        .subcommand(ClapCommand::new("list").about("List available wallpapers themes"))
        .subcommand(ClapCommand::new("set").about("Set a wallpaper theme").arg(Arg::new("theme").required(true)))
        .subcommand(ClapCommand::new("next").about("Next wallpaper from current theme"))
        .subcommand(ClapCommand::new("set-dir").about("Set the directory for wallpaper themes").arg(Arg::new("dir").required(true)))
        .get_matches();

    // Match and execute the corresponding subcommands 
    match command.subcommand() {
        Some(("list", _)) => {
            // List all themes within the wallpaper themes directory
            theme::list_themes(&config.wallpaper_dir).unwrap_or_else(|e| {
                error!("Error while trying to list all themes: {}", e);
            });
        }

        Some(("set", args)) => {
            // Check if the theme exists
            let theme = args.get_one::<String>("theme").unwrap();
            let theme_path = std::path::Path::new(&config.wallpaper_dir).join(theme);

            if theme_path.exists() && theme_path.is_dir() {
                // Apply theme if it exists and saves to configuration
                config.theme = theme.clone();
                save_config(&config).unwrap();
                
                if let Err(e) = wallpaper::set_wallpaper(&config.theme, &config.wallpaper_dir) {
                    error!("Error while trying to apply wallpaper: {}", e);
                }
            } else {
                // Throw an error if the theme doesn't exist
                eprintln!("The '{}' theme doesn't exist in {}.", theme, config.wallpaper_dir);
            }

        }

        Some(("next", _)) => {
            // Set the next wallpaper from the current theme
            wallpaper::set_wallpaper(&config.theme, &config.wallpaper_dir).unwrap_or_else(|e| {
                error!("Error while trying to set a wallpaper: {}", e);
            });
        }

        Some(("set-dir", args)) => {
            // Update the wallpaper themes directory
            let new_dir = args.get_one::<String>("dir").unwrap();
            config.wallpaper_dir = new_dir.clone();
            
            // Save the new configuration
            if let Err(e) = save_config(&config) {
                error!("Error while trying to save new directory: {}", e);
            } else {
                println!("Wallpaper themes directory set to '{}'", new_dir);
            }
        }

        _ => {
            // Timer to change the wallpaper with the configured interval
            let change_interval = config.change_interval;

            loop {
                thread::sleep(Duration::from_secs(change_interval as u64 * 60));

                wallpaper::set_wallpaper(&config.theme, &config.wallpaper_dir).unwrap_or_else(|e| {
                    error!("Error while trying to load wallpaper: {}", e);
                });
            }
        }
    };
}
