use std::env;

mod option;

use option::DoomOptions;

fn main() {
    // Skipping the first arg as this is the executable name
    // and we don't want that
    //TODO Figure out how to condense this into one step
    let cmd_args_string: Vec<String> = env::args().skip(1).collect();
    let cmd_args_str = cmd_args_string.iter().map(AsRef::as_ref).collect();

    let mut doom_options: DoomOptions = DoomOptions::new();

    option::set_options(&mut doom_options, cmd_args_str);
}
