use std::env;

mod option;

use option::DoomOptions;

fn main() {
    // Skipping the first arg as this is the executable name
    // and we don't want that
    let cmd_args: Vec<String> = env::args().skip(1).collect();

    let mut doom_options: DoomOptions = DoomOptions::new();

    option::set_options(&mut doom_options, cmd_args);
}
