use std::env;
use std::path::PathBuf;

pub struct DoomConfig {
    pub config_file_path: PathBuf,
    pub wad_files_dir: PathBuf,
}

impl Default for DoomConfig {
    fn default() -> Self {
        let home_dir: PathBuf = match home::home_dir() {
            Some(x) => x,
            None => panic!("Unable to find home directory"),
        };

        let mut config_file_path: PathBuf = home_dir.clone();
        config_file_path.push(".doomrc");

        println!("Setting config file location to {}", config_file_path.display());

        let wad_files_dir: PathBuf = match env::var("DOOMWADDIR") {
            Ok(x) => {
                println!("Setting wad dir to DOOMWADDIR env variable value: {}", x);
                PathBuf::from(x)
            }
            Err(_) => {
                match env::current_dir() {
                    Ok(current_dir) => {
                        println!("DOOMWADDIR env variable not set. Defaulting wad dir to current dir: {}", current_dir.display());
                        current_dir
                    }
                    Err(e) => panic!(
                        "Unable to access dir where game was started.\n Error: {:?}",
                        e
                    ),
                }
            }
        };

        DoomConfig {
            config_file_path,
            wad_files_dir,
        }
    }
}
