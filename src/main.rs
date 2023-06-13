use std::{env, path::PathBuf};

mod config;
mod option;
mod util;
mod wad;

use config::Config;
use option::DoomOptions;

use crate::wad::{Lump, process_wad_file};

fn main() {
    // Skipping the first arg as this is the executable name
    // and we don't want that
    let cmd_args: Vec<String> = env::args().skip(1).collect();
    let doom_options: DoomOptions = DoomOptions::new(cmd_args);

    let mut config: Config = Config::new(&doom_options);

    let wads_to_process: Vec<PathBuf> =
        wad::iwad::find_valid_iwad_file_paths(&config.wad_files_dir, &doom_options);

    config.set_game_type_by_iwad_paths(&wads_to_process);
    config.set_language_by_iwad_paths(&wads_to_process);

    println!("{}", config.game_title());

    if doom_options.is_option_enabled("-devparm") {
        println!("Development mode ON.");
    }

    let mut lumps: Vec<Lump> = Vec::new();
    
    for wad_path in wads_to_process {
        // Original engine didn't error out if it had issues reading a file
        match wad::process_wad_file(&wad_path) {
            Ok(mut wad_lumps) => lumps.append(&mut wad_lumps),
            Err(error) => println!("Error processing file {}.\n {error}", wad_path.display())
        } 
    }

    println!("{}", lumps.len());

}
