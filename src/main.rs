use std::{env, path::PathBuf};

mod config;
mod option;
mod util;
mod wad;

use config::Config;
use option::DoomOptions;

use crate::wad::DoomFile;

fn main() {
    // Skipping the first arg as this is the executable name
    // and we don't want that
    let cmd_args: Vec<String> = env::args().skip(1).collect();
    let doom_options: DoomOptions = DoomOptions::new(cmd_args);

    let mut config: Config = Config::new(&doom_options);

    let mut wads_to_process: Vec<PathBuf> =
        wad::iwad::find_valid_iwad_file_paths(&config.wad_files_dir, &doom_options);

    if doom_options.is_option_enabled("-file") {
        let mut file_paths: Vec<PathBuf> = doom_options
            .get_option_by_name("-file")
            .unwrap()
            .values
            .iter()
            .map(PathBuf::from)
            .collect();
        wads_to_process.append(&mut file_paths);
    }

    config.set_game_type_by_iwad_paths(&wads_to_process);
    config.set_language_by_iwad_paths(&wads_to_process);

    println!("{}", config.game_title());

    if doom_options.is_option_enabled("-devparm") {
        println!("Development mode ON.");
    }

    let mut doom_files: Vec<DoomFile> = Vec::new();

    for wad_path in wads_to_process {
        // Original engine didn't error out if it had issues reading a file
        match wad::process_file(&wad_path) {
            Ok(doom_file) => doom_files.push(doom_file),
            Err(error) => eprintln!("Error processing file {}.\n {}", wad_path.display(), error),
        }
    }
}
