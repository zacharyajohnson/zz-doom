use std::env;
use std::path::PathBuf;

pub struct DoomConfig<'a> {
    pub config_file_path: PathBuf,
    pub wad_files_dir: PathBuf,
    pub game_version: &'a str,
}

impl<'a> Default for DoomConfig<'a> {
    fn default() -> Self {
        let mut config_file_path: PathBuf = match home::home_dir() {
            Some(x) => {
                println!("Home directory set to: {}", x.display());
                x
            }
            None => panic!(
                "Unable to find home directory.\n
                            Please set the USERPROFILE environment variable if on Windows or
                            the HOME environment variable if on Linux/Unix"
            ),
        };

        config_file_path.push(".doomrc");

        println!(
            "Setting config file location to {}",
            config_file_path.display()
        );

        let wad_files_dir: PathBuf = match env::var("DOOMWADDIR") {
            Ok(x) => {
                println!(
                    "Setting wad dir to DOOMWADDIR environment variable value: {}",
                    x
                );
                PathBuf::from(x)
            }
            Err(_) => match env::current_dir() {
                Ok(current_dir) => {
                    println!("DOOMWADDIR environment variable not set. Defaulting wad dir to current dir: {}", current_dir.display());
                    current_dir
                }
                Err(e) => panic!(
                    "Unable to access dir where game was started.\n Error: {}",
                    e
                ),
            },
        };

        DoomConfig {
            config_file_path,
            wad_files_dir,
            game_version: env!("CARGO_PKG_VERSION"),
        }
    }
}
