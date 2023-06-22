use std::{ops::RangeInclusive, path::PathBuf};

use crate::util;

// (option_name, min_num_values - max_num_values)
const DEFAULT_OPTIONS: [(&str, RangeInclusive<u32>); 24] = [
    ("-devparm", 0..=0),
    ("-nomonsters", 0..=0),
    ("-respawn", 0..=0),
    ("-fast", 0..=0),
    ("-debugfile", 0..=0),
    ("-shdev", 0..=0),
    ("-regdev", 0..=0),
    ("-comdev", 0..=0),
    ("-altdeath", 0..=0),
    ("-deathmatch", 0..=0),
    ("-cdrom", 0..=0),
    // int
    ("-turbo", 1..=1),
    // 2 Strings, list
    ("-wart", 2..=2),
    ("-warp", 2..=2),
    // List of paths
    ("-file", 1..=255),
    // String
    ("-playdemo", 1..=1),
    // String
    ("-timedemo", 1..=1),
    // int
    ("-skill", 1..=1),
    // int
    ("-episode", 1..=1),
    // int
    ("-timer", 1..=1),
    ("-avg", 0..=0),
    // void pointer
    ("-statcopy", 1..=1),
    // String
    ("-record", 1..=1),
    // char
    ("-loadgame", 1..=1),
];

pub struct DoomOption {
    name: &'static str,
    pub enabled: bool,
    pub values: Vec<String>,
    min_num_values: u32,
    max_num_values: u32,
}

pub struct DoomOptions {
    options: Vec<DoomOption>,
}

impl DoomOptions {
    pub fn new(cmd_args: Vec<String>) -> DoomOptions {
        DoomOptions {
            options: create_options(cmd_args),
        }
    }

    pub fn get_option_by_name(&self, option_name: &str) -> Option<&DoomOption> {
        self.options
            .iter()
            .find(|option| option.name == option_name)
    }

    pub fn is_option_enabled(&self, option_name: &str) -> bool {
        match self.get_option_by_name(option_name) {
            Some(option) => option.enabled,
            None => false,
        }
    }
}

fn get_response_file_options(file_path: PathBuf) -> Vec<String> {
    let final_file_path = if file_path.is_relative() {
        let mut exe_parent_path: PathBuf = util::exe_parent_path();
        exe_parent_path.push(file_path);
        exe_parent_path
    } else {
        file_path
    };

    let lines: Vec<String> = std::fs::read_to_string(&final_file_path)
        .expect(&format!(
            "Unable to find response file: {}",
            final_file_path.display()
        ))
        .lines()
        .map(|x| x.to_owned())
        .collect();

    println!("Found response file {}", final_file_path.display());

    lines
        .iter()
        .filter(|x| {
            x.chars().all(|c| {
                if c.is_ascii() {
                    c >= ' ' && c <= 'z'
                } else {
                    false
                }
            })
        })
        .flat_map(|line| line.split_ascii_whitespace().map(|x| x.to_owned()))
        .collect()
}

fn create_options(cmd_args: Vec<String>) -> Vec<DoomOption> {
    let mut arg_index = 0;

    let mut doom_options: Vec<DoomOption> = DEFAULT_OPTIONS
        .iter()
        .map(|x| -> DoomOption {
            DoomOption {
                name: x.0,
                enabled: false,
                values: Vec::new(),
                min_num_values: *x.1.start(),
                max_num_values: *x.1.end(),
            }
        })
        .collect();

    let args_to_process: Vec<String> = cmd_args
        .iter()
        .position(|x| x.starts_with("@"))
        .map(|index| {
            let mut response_file_args: Vec<String> =
                get_response_file_options(PathBuf::from(cmd_args[index].trim_start_matches("@")));

            // Grab all the args after response and put it into the args list
            // to process since thats how the original game behaves
            if index < cmd_args.len() - 1 {
                response_file_args.extend_from_slice(&cmd_args[index + 1..]);
            }

            response_file_args
        })
        .unwrap_or_else(|| cmd_args);

    while arg_index < args_to_process.len() {
        let option_name: &str = &args_to_process[arg_index];
        let option: &mut DoomOption = doom_options
            .iter_mut()
            .find(|option| option.name == option_name)
            .unwrap_or_else(|| panic!("Option {} does not exist.", option_name));

        let mut option_values: Vec<String> = Vec::new();
        let min_num_values = option.min_num_values;
        let max_num_values = option.max_num_values;

        arg_index += 1;

        let mut value_index = 0;

        while arg_index != args_to_process.len() && !args_to_process[arg_index].starts_with("-") {
            value_index += 1;

            option_values.push(args_to_process[arg_index].clone());
            arg_index += 1;
        }

        if value_index > max_num_values {
            panic!("Too many values provided for option {}. Requires min of {} values and max of {} values, but {} supplied.", option.name, min_num_values, max_num_values, value_index);
        } else if value_index < min_num_values {
            panic!("Not enough values provided for option {}. Requires min of {} values and max of {} values, but {} supplied.", option.name, min_num_values, max_num_values, value_index);
        }

        option.values = option_values;
        option.enabled = true;
    }
    doom_options
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::option::*;

    fn generate_cmd_args(option_value_map: &HashMap<&str, &[&str]>) -> Vec<String> {
        let mut cmd_args: Vec<String> = Vec::new();
        for (option, values) in option_value_map {
            cmd_args.push(String::from(*option));

            for value in *values {
                cmd_args.push(value.to_string());
            }
        }

        cmd_args
    }

    // The DEFAULT_OPTIONS variable contains the original
    // command line flags Doom shipped with, so we will support them
    // and set them up. The constructor for DoomOptions should insert
    // them for us. This is with no cmd line args passed in to check if
    // they are always there.
    #[test]
    fn test_doom_options_new_initializes_with_default_doom_options() {
        let doom_options: DoomOptions = DoomOptions::new(Vec::new());

        assert!(doom_options.options.iter().all(|option| {
            DEFAULT_OPTIONS.iter().any(|x| {
                x.0 == option.name
                    && *x.1.start() == option.min_num_values
                    && *x.1.end() == option.max_num_values
            }) && !option.enabled
        }));

        assert_eq!(doom_options.options.len(), DEFAULT_OPTIONS.len());
    }

    #[test]
    fn test_get_option_by_name_returns_value() {
        let doom_options: DoomOptions = DoomOptions::new(Vec::new());

        let dev_option: Option<&DoomOption> = doom_options.get_option_by_name("-devparm");
        assert!(dev_option.is_some());
    }

    #[test]
    fn test_doom_options_is_option_enabled() {
        let option_value_map: HashMap<&str, &[&str]> = HashMap::from([
            ("-devparm", [].as_slice()),
            ("-record", ["test"].as_slice()),
            ("-wart", ["1", "1"].as_slice()),
        ]);

        let cmd_args: Vec<String> = generate_cmd_args(&option_value_map);
        let doom_options: DoomOptions = DoomOptions::new(cmd_args);

        // Test an option that does exist that was passed in returns true
        for option in option_value_map.keys() {
            assert!(doom_options.is_option_enabled(option));
        }

        // Test an option that does exist but wasn't passed in returns false
        assert!(!doom_options.is_option_enabled("-shdev"));
        // Test an option that doesn't exist returns false
        assert!(!doom_options.is_option_enabled("-test"));
    }

    // All options should start with a -,
    // invalidOption should fail
    #[test]
    #[should_panic]
    fn test_doom_options_new_with_invalid_option() {
        let cmd_args: Vec<String> = vec!["invalidOption".to_string()];

        DoomOptions::new(cmd_args);
    }

    // We want to fail if we pass too many values for an option
    // In this case its wart that should fail since it requires
    // a max of two values but we provided 3.
    #[test]
    #[should_panic]
    fn test_doom_options_new_with_too_many_option_values() {
        let cmd_args: Vec<String> = vec![
            "-wart".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
        ];

        DoomOptions::new(cmd_args);
    }

    // We want to fail if we pass not enough values for an option
    // In this case its wart that should fail since it requires
    // a min of two values but we provided 1.
    #[test]
    #[should_panic]
    fn test_doom_options_new_with_not_enough_option_values() {
        let cmd_args: Vec<String> = vec!["-wart".to_string(), "1".to_string()];

        DoomOptions::new(cmd_args);
    }

    // Custom options, for now we don't support custom options but
    // it might be nice to figure out how to do it
    #[test]
    #[should_panic]
    fn test_doom_options_new_with_custom_options() {
        let cmd_args: Vec<String> = vec![
            "-test".to_string(),
            "-test1value".to_string(),
            "1".to_string(),
            "-test3value".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ];

        DoomOptions::new(cmd_args);
    }

    // Existing args, values should be set,
    // no new structs should be created
    #[test]
    fn test_doom_options_new_sets_doom_option_values_based_on_cmd_args() {
        let option_value_map: HashMap<&str, &[&str]> = HashMap::from([
            ("-wart", ["1", "1"].as_slice()),
            ("-devparm", [].as_slice()),
            ("-record", ["test"].as_slice()),
            ("-shdev", [].as_slice()),
            (
                "-file",
                ["file1", "path/to/file2", "/path/to/file3"].as_slice(),
            ),
            ("-comdev", [].as_slice()),
        ]);

        let cmd_args: Vec<String> = generate_cmd_args(&option_value_map);

        let doom_options = DoomOptions::new(cmd_args);

        for (option_name, values) in option_value_map {
            let option: &DoomOption = doom_options
                .get_option_by_name(option_name)
                .unwrap_or_else(|| panic!("Could not find option {}", option_name));

            for value in values {
                assert!(option.values.contains(&value.to_string()));
            }

            assert_eq!(option.values.len(), values.len());
            assert!(option.enabled);
        }
    }

    // Read arguments in from a response file denoted
    // by @response_file_path
    // Relative paths for response file will always be interpreted
    // as starting at the dir the exe is located in
    // The response file will have one option and its values per line
    // Will drop all options passed into the command line before it
    // Will keep all options passed into the command line after it
    // If the option line in a response file has an invalid char
    // such as tab or |, (anything less then ascii code for space or
    // anything more then ascii code z) the line will be ignored and
    // not be processed
    #[test]
    fn test_doom_options_new_sets_doom_options_from_response_file() {
        let mut exe_parent_path: PathBuf = util::exe_parent_path();
        exe_parent_path.push("responsefile");

        std::fs::copy("tests/resource/responsefile", exe_parent_path)
            .unwrap_or_else(|error| panic!("{}", error));

        // Should be dropped since
        // it is before @responsefile
        let option_value_map_before: HashMap<&str, &[&str]> = HashMap::from([
            ("-wart", ["1", "1"].as_slice()),
            ("-devparm", [].as_slice()),
            ("-record", ["test"].as_slice()),
        ]);

        // We have to construct the cmd args piecemeal since we need to preserve
        // the order
        let mut cmd_args: Vec<String> = generate_cmd_args(&option_value_map_before);

        // For this test, response file will
        // have:
        //      -shdev
        //      -file file1 path/to/file2 /path/to/file3
        //      -|test 1 2 3
        //      -(tab)test2 3 4 5
        //-shdev and -file are valid and should be processed
        //-test and test2 have invalid chars so they should be ignored
        // Should keep -comdev since its after @responsefile
        cmd_args.push("@responsefile".to_string());
        cmd_args.push("-comdev".to_string());

        let doom_options = DoomOptions::new(cmd_args);

        // Before response file. Should not be set
        for (option_name, values) in option_value_map_before {
            let option: &DoomOption = doom_options
                .get_option_by_name(option_name)
                .unwrap_or_else(|| panic!("Could not find option {}", option_name));
            assert!(!option.enabled);

            for value in values {
                assert!(!option.values.contains(&value.to_string()));
            }
        }

        // Start response file
        let response_file_option_map: HashMap<&str, &[&str]> = HashMap::from([
            ("-shdev", [].as_slice()),
            (
                "-file",
                ["file1", "path/to/file2", "/path/to/file3"].as_slice(),
            ),
        ]);

        for (option_name, values) in response_file_option_map {
            let option: &DoomOption = doom_options
                .get_option_by_name(option_name)
                .unwrap_or_else(|| panic!("Could not find option {}", option_name));
            for value in values {
                assert!(option.values.contains(&value.to_string()));
                assert!(option.enabled);
            }
        }
        // End response File

        // After response file
        let comdev_option: Option<&DoomOption> = doom_options.get_option_by_name("-comdev");
        assert!(comdev_option.is_some());

        let comdev: &DoomOption = comdev_option.unwrap();
        assert!(comdev.enabled);
    }

    #[test]
    #[should_panic]
    // If we can't find the response file we should exit
    fn test_doom_options_invalid_response_file() {
        let cmd_args: Vec<String> = vec!["@invalid".to_string()];
        DoomOptions::new(cmd_args);
    }
}
