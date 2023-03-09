pub struct DoomOption {
    name: &'static str,
    values: Option<String>,
    min_num_values: u32,
    max_num_values: u32,
}

impl DoomOption {
    pub fn enabled(&self) -> bool {
        self.values.is_some()
    }
}

pub struct DoomOptions {
    options: Vec<DoomOption>,
}

impl DoomOptions {
    pub fn new(cmd_args: Vec<String>) -> DoomOptions {
        let default_options: Vec<DoomOption> = vec![
            DoomOption {
                name: "-devparm",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            DoomOption {
                name: "-nomonsters",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            DoomOption {
                name: "-respawn",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            DoomOption {
                name: "-fast",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            DoomOption {
                name: "-debugfile",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            DoomOption {
                name: "-shdev",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            DoomOption {
                name: "-regdev",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            DoomOption {
                name: "-comdev",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            DoomOption {
                name: "-altdeath",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            DoomOption {
                name: "-deathmatch",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            DoomOption {
                name: "-cdrom",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            // int
            DoomOption {
                name: "-turbo",
                values: None,
                min_num_values: 1,
                max_num_values: 1,
            },
            // 2 Strings, list
            DoomOption {
                name: "-wart",
                values: None,
                min_num_values: 2,
                max_num_values: 2,
            },
            DoomOption {
                name: "-warp",
                values: None,
                min_num_values: 2,
                max_num_values: 2,
            },
            // List of paths
            DoomOption {
                name: "-file",
                values: None,
                min_num_values: 1,
                max_num_values: 255,
            },
            // String
            DoomOption {
                name: "-playdemo",
                values: None,
                min_num_values: 1,
                max_num_values: 1,
            },
            // String
            DoomOption {
                name: "-timedemo",
                values: None,
                min_num_values: 1,
                max_num_values: 1,
            },
            // int
            DoomOption {
                name: "-skill",
                values: None,
                min_num_values: 1,
                max_num_values: 1,
            },
            // int
            DoomOption {
                name: "-episode",
                values: None,
                min_num_values: 1,
                max_num_values: 1,
            },
            // int
            DoomOption {
                name: "-timer",
                values: None,
                min_num_values: 1,
                max_num_values: 1,
            },
            DoomOption {
                name: "-avg",
                values: None,
                min_num_values: 0,
                max_num_values: 0,
            },
            // void pointer
            DoomOption {
                name: "-statcopy",
                values: None,
                min_num_values: 1,
                max_num_values: 1,
            },
            // String
            DoomOption {
                name: "-record",
                values: None,
                min_num_values: 1,
                max_num_values: 1,
            },
            // char
            DoomOption {
                name: "-loadgame",
                values: None,
                min_num_values: 1,
                max_num_values: 1,
            },
        ];

        let mut doom_options: DoomOptions = DoomOptions {
            options: default_options,
        };

        set_options(&mut doom_options, cmd_args);

        doom_options
    }

    pub fn get_option_by_name(&self, option_name: &str) -> Option<&DoomOption> {
        for doom_option in self.options.iter() {
            if doom_option.name == option_name {
                return Some(doom_option);
            }
        }

        return None;
    }

    fn get_option_by_name_mut(&mut self, option_name: &str) -> Option<&mut DoomOption> {
        for doom_option in self.options.iter_mut() {
            if doom_option.name == option_name {
                return Some(doom_option);
            }
        }

        return None;
    }
}

fn get_response_file_options(file_path: &str) -> Vec<String> {
    let lines: Vec<String> = std::fs::read_to_string(file_path)
        .expect(&format!("Unable to find response file {}", file_path))
        .lines()
        .map(|x| x.to_owned())
        .collect();

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

fn set_options(doom_options: &mut DoomOptions, cmd_args: Vec<String>) {
    let mut arg_index = 0;

    let args_to_process = match cmd_args.iter().position(|x| x.starts_with("@")) {
        Some(i) => {
            let mut response_file_args: Vec<String> =
                get_response_file_options(cmd_args[i].trim_start_matches("@"));

            // Grab all the args after response and put it into the args list
            // to process since thats how the original game behaves
            if i < cmd_args.len() - 1 {
                response_file_args.extend_from_slice(&cmd_args[i + 1..]);
            }

            response_file_args
        }
        None => cmd_args,
    };

    while arg_index < args_to_process.len() {
        let option_name: &str = &args_to_process[arg_index];
        let option: Option<&mut DoomOption> = doom_options.get_option_by_name_mut(option_name);

        match option {
            Some(i) => {
                let mut option_value: String = String::new();
                let min_num_values = i.min_num_values;
                let max_num_values = i.max_num_values;

                arg_index += 1;

                if max_num_values == 0 {
                    option_value.push_str("true");
                } else {
                    let mut value_index = 0;

                    while arg_index != args_to_process.len()
                        && !args_to_process[arg_index].starts_with("-")
                    {
                        value_index += 1;

                        if value_index > max_num_values {
                            panic!("Too many values provided for option {}. Requires min of {} values and max of {} values, but {} supplied.", i.name, min_num_values, max_num_values, value_index);
                        }

                        option_value.push_str(&args_to_process[arg_index]);
                        arg_index += 1;

                        //TODO This is ugly
                        if arg_index != args_to_process.len()
                            && !args_to_process[arg_index].starts_with("-")
                        {
                            option_value.push_str(" ");
                        }
                    }

                    if value_index < min_num_values {
                        panic!("Not enough values provided for option {}. Requires min of {} values and max of {} values, but {} supplied.", i.name, min_num_values, max_num_values, value_index);
                    }
                }

                i.values = Some(option_value);
            }
            None => panic!("Option {} does not exist.", option_name),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::option::*;

    // These are the original command line flags Doom
    // shipped with, so we will support them and set them
    // up. The constructor for DoomOptions should insert
    // them for us.
    #[test]
    fn test_doom_options_new_intializes_with_default_doom_options() {
        let doom_options: DoomOptions = DoomOptions::new(Vec::new());
        let doom_option_names: Vec<String> = vec![
            "-devparm".to_string(),
            "-nomonsters".to_string(),
            "-respawn".to_string(),
            "-fast".to_string(),
            "-debugfile".to_string(),
            "-shdev".to_string(),
            "-regdev".to_string(),
            "-comdev".to_string(),
            "-altdeath".to_string(),
            "-deathmatch".to_string(),
            "-cdrom".to_string(),
            "-turbo".to_string(),
            "-wart".to_string(),
            "-warp".to_string(),
            "-file".to_string(),
            "-playdemo".to_string(),
            "-timedemo".to_string(),
            "-skill".to_string(),
            "-episode".to_string(),
            "-timer".to_string(),
            "-avg".to_string(),
            "-statcopy".to_string(),
            "-record".to_string(),
            "-loadgame".to_string(),
        ];

        assert!(doom_options.options.iter().all(|option| {
            doom_option_names.iter().any(|x| x == option.name)
                && !option.enabled()
                && option.values == None
        }));
        assert_eq!(doom_options.options.len(), doom_option_names.len());
    }

    #[test]
    fn test_get_option_by_name_returns_value() {
        let doom_options: DoomOptions = DoomOptions::new(Vec::new());

        let dev_option: Option<&DoomOption> = doom_options.get_option_by_name("-devparm");
        assert!(dev_option.is_some());
    }

    #[test]
    fn test_get_option_by_name_mut_returns_value() {
        let mut doom_options: DoomOptions = DoomOptions::new(Vec::new());

        let dev_option: Option<&mut DoomOption> = doom_options.get_option_by_name_mut("-devparm");
        assert!(dev_option.is_some());
    }

    // Creating doom options should still work with no options passed in
    // even if no cmd args are passed into it
    // No options should have their values set
    #[test]
    fn test_doom_options_new_works_with_no_cmd_args() {
        let doom_options: DoomOptions = DoomOptions::new(Vec::new());

        assert!(doom_options
            .options
            .iter()
            .all(|option| { !option.enabled() && option.values == None }));
    }

    // All options should start with a -,
    // invalidOption should fail
    #[test]
    #[should_panic]
    fn test_doom_options_new_with_invalid_option() {
        let cmd_args: Vec<String> = vec![
            "-devparm".to_string(),
            "-record".to_string(),
            "test".to_string(),
            "invalidOption".to_string(),
            "-wart".to_string(),
            "1".to_string(),
            "1".to_string(),
        ];

        DoomOptions::new(cmd_args);
    }

    // We want to fail if we pass too many values for an option
    // In this case its wart that should fail since it requires
    // a max of two values but we provided 3.
    #[test]
    #[should_panic]
    fn test_doom_options_new_with_too_many_option_values() {
        let cmd_args: Vec<String> = vec![
            "-devparm".to_string(),
            "-record".to_string(),
            "test".to_string(),
            "-wart".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "-shdev".to_string(),
            "-file".to_string(),
            "file1".to_string(),
            "path/to/file2".to_string(),
            "/path/to/file3".to_string(),
            "-comdev".to_string(),
        ];

        DoomOptions::new(cmd_args);
    }

    // We want to fail if we pass not enough values for an option
    // In this case its wart that should fail since it requires
    // a min of two values but we provided 1.
    #[test]
    #[should_panic]
    fn test_doom_options_new_with_not_enough_option_values() {
        let cmd_args: Vec<String> = vec![
            "-wart".to_string(),
            "1".to_string(),
            "-devparm".to_string(),
            "-record".to_string(),
            "test".to_string(),
            "-shdev".to_string(),
            "-file".to_string(),
            "file1".to_string(),
            "path/to/file2".to_string(),
            "/path/to/file3".to_string(),
            "-comdev".to_string(),
        ];

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
    fn test_doom_options_new_sets_existing_doom_option_values() {
        let cmd_args: Vec<String> = vec![
            "-wart".to_string(),
            "1".to_string(),
            "1".to_string(),
            "-devparm".to_string(),
            "-record".to_string(),
            "test".to_string(),
            "-shdev".to_string(),
            "-file".to_string(),
            "file1".to_string(),
            "path/to/file2".to_string(),
            "/path/to/file3".to_string(),
            "-comdev".to_string(),
        ];

        let doom_options = DoomOptions::new(cmd_args);

        let devparm_option: Option<&DoomOption> = doom_options.get_option_by_name("-devparm");
        assert!(devparm_option.is_some());

        let devparm: &DoomOption = devparm_option.unwrap();
        assert!(devparm.values.as_ref().unwrap().eq("true"));
        assert!(devparm.enabled());

        let shdev_option: Option<&DoomOption> = doom_options.get_option_by_name("-shdev");
        assert!(shdev_option.is_some());

        let shdev: &DoomOption = shdev_option.unwrap();
        assert!(shdev.values.as_ref().unwrap().eq("true"));
        assert!(shdev.enabled());

        let wart_option: Option<&DoomOption> = doom_options.get_option_by_name("-wart");
        assert!(wart_option.is_some());

        let wart: &DoomOption = wart_option.unwrap();
        assert!(wart.values.as_ref().unwrap().eq("1 1"));
        assert!(wart.enabled());

        let record_option: Option<&DoomOption> = doom_options.get_option_by_name("-record");
        assert!(record_option.is_some());

        let record: &DoomOption = record_option.unwrap();
        assert!(record.values.as_ref().unwrap().eq("test"));
        assert!(record.enabled());

        let file_option: Option<&DoomOption> = doom_options.get_option_by_name("-file");
        assert!(file_option.is_some());

        let file: &DoomOption = file_option.unwrap();
        assert!(file
            .values
            .as_ref()
            .unwrap()
            .eq("file1 path/to/file2 /path/to/file3"));
        assert!(file.enabled());

        let comdev_option: Option<&DoomOption> = doom_options.get_option_by_name("-comdev");
        assert!(comdev_option.is_some());

        let comdev: &DoomOption = comdev_option.unwrap();
        assert!(comdev.values.as_ref().unwrap().eq("true"));
        assert!(comdev.enabled());
    }

    // Read arguments in from a response file denoated
    // by @response_file_path
    // The response file will have one option and its values per line
    // Will drop all options passed into the command line before it
    // Will keep all options passed into the command line after it
    // If the option line in a response file has an invalid char
    // such as tab or |, (anything less than ascii code for space or
    // anything more than ascii code z) the line will be ignored and
    // not be processed
    #[test]
    fn test_doom_options_new_sets_doom_options_from_response_file() {
        let cmd_args: Vec<String> = vec![
            // Should be dropped since
            // it is before @responsefile
            "-wart".to_string(),
            "1".to_string(),
            "1".to_string(),
            "-devparm".to_string(),
            "-record".to_string(),
            "test".to_string(),
            // For this test, response file will
            // have:
            //      -shdev
            //      -file file1 path/to/file2 /path/to/file3
            //      -|test 1 2 3
            //      -(tab)test2 3 4 5
            //-shdev and -file are valid and should be processed
            //-test and test2 have invalid chars so they should be ignored
            // Should keep -comdev since its after @responsefile
            "@tests/resource/responsefile".to_string(),
            "-comdev".to_string(),
        ];

        let doom_options = DoomOptions::new(cmd_args);

        // Before response file. Should not be set
        let wart_option: Option<&DoomOption> = doom_options.get_option_by_name("-wart");
        assert!(wart_option.is_some());

        let wart: &DoomOption = wart_option.unwrap();
        assert!(wart.values.as_ref().is_none());

        let devparm_option: Option<&DoomOption> = doom_options.get_option_by_name("-devparm");

        let devparm: &DoomOption = devparm_option.unwrap();
        assert!(devparm.values.as_ref().is_none());

        let record_option: Option<&DoomOption> = doom_options.get_option_by_name("-record");
        assert!(record_option.is_some());

        let record: &DoomOption = record_option.unwrap();
        assert!(record.values.as_ref().is_none());

        // Start response file
        let shdev_option: Option<&DoomOption> = doom_options.get_option_by_name("-shdev");
        assert!(shdev_option.is_some());

        let shdev: &DoomOption = shdev_option.unwrap();
        assert!(shdev.values.as_ref().unwrap().eq("true"));
        assert!(shdev.enabled());

        let file_option: Option<&DoomOption> = doom_options.get_option_by_name("-file");
        assert!(file_option.is_some());

        let file: &DoomOption = file_option.unwrap();
        assert!(file
            .values
            .as_ref()
            .unwrap()
            .eq("file1 path/to/file2 /path/to/file3"));
        assert!(file.enabled());
        // End response File

        // After response file
        let comdev_option: Option<&DoomOption> = doom_options.get_option_by_name("-comdev");
        assert!(comdev_option.is_some());

        let comdev: &DoomOption = comdev_option.unwrap();
        assert!(comdev.values.as_ref().unwrap().eq("true"));
        assert!(comdev.enabled());
    }
}
