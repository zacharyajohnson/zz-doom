use std::env;

mod config;
mod option;
mod wad;

use option::DoomOptions;

fn main() {
    // Skipping the first arg as this is the executable name
    // and we don't want that
    let cmd_args: Vec<String> = env::args().skip(1).collect();
    let doom_options: DoomOptions = DoomOptions::new(cmd_args);
}
