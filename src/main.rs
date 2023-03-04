use std::env;

mod config;
mod option;
mod wad;

use option::DoomOptions;
use config:: DoomConfig;
use wad::DoomWads;

fn main() {
    let mut doom_config: DoomConfig = Default::default();
    let mut doom_wads: DoomWads = Default::default();

    // Skipping the first arg as this is the executable name
    // and we don't want that
    let cmd_args: Vec<String> = env::args().skip(1).collect();
    let mut doom_options: DoomOptions = DoomOptions::new();

    option::set_options(&mut doom_options, cmd_args);
}
