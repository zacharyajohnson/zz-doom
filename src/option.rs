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
    pub fn new() -> DoomOptions {
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

        DoomOptions {
            options: default_options,
        }
    }
}

pub fn get_option_by_name<'a>(
    doom_options: &'a DoomOptions,
    option_name: &str,
) -> Option<&'a DoomOption> {
    for doom_option in &doom_options.options {
        if doom_option.name == option_name {
            return Some(doom_option);
        }
    }

    return None;
}

pub fn get_option_by_name_mut<'a>(
    doom_options: &'a mut DoomOptions,
    option_name: &str,
) -> Option<&'a mut DoomOption> {
    for doom_option in doom_options.options.iter_mut() {
        if doom_option.name == option_name {
            return Some(doom_option);
        }
    }

    return None;
}

pub fn set_options(doom_options: &mut DoomOptions, cmd_args: Vec<&str>) {
    if cmd_args.is_empty() {
        return;
    }

    if !cmd_args[0].starts_with('-') {
        panic!("Invalid option {}. Must start with a -", cmd_args[0]);
    }

    let mut cmd_arg_index = 0;
    while cmd_arg_index < cmd_args.len() {
        let option_name: &str = cmd_args[cmd_arg_index];
        let option: Option<&mut DoomOption> = get_option_by_name_mut(doom_options, option_name);

        match option {
            Some(i) => {
                let mut option_value: String = String::new();
                let mut value_index = 0;
                let min_num_values = i.min_num_values;
                let max_num_values = i.max_num_values;

                cmd_arg_index += 1;

                if max_num_values == 0 {
                    option_value.push_str("true");
                } else {
                    while cmd_arg_index != cmd_args.len()
                        && !cmd_args[cmd_arg_index].starts_with("-")
                    {
                        value_index += 1;

                        if value_index > max_num_values {
                            panic!("Too many values provided for option {}. Requires min of {} values and max of {} values, but {} supplied.", i.name, min_num_values, max_num_values, value_index);
                        }

                        option_value.push_str(cmd_args[cmd_arg_index]);
                        cmd_arg_index += 1;

                        //TODO This is ugly
                        if cmd_arg_index != cmd_args.len()
                            && !cmd_args[cmd_arg_index].starts_with("-")
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
    fn test_doom_options_intializes_with_default_doom_options() {
        let doom_options: DoomOptions = DoomOptions::new();
        let doom_option_names: Vec<&str> = vec![
            "-devparm",
            "-nomonsters",
            "-respawn",
            "-fast",
            "-debugfile",
            "-shdev",
            "-regdev",
            "-comdev",
            "-altdeath",
            "-deathmatch",
            "-cdrom",
            "-turbo",
            "-wart",
            "-warp",
            "-file",
            "-playdemo",
            "-timedemo",
            "-skill",
            "-episode",
            "-timer",
            "-avg",
            "-statcopy",
            "-record",
            "-loadgame",
        ];

        assert!(doom_options.options.iter().all(|option| {
            doom_option_names.contains(&option.name) && !option.enabled() && option.values == None
        }));
        assert_eq!(doom_options.options.len(), doom_option_names.len());
    }

    #[test]
    fn test_get_option_by_name_returns_value() {
        let doom_options: DoomOptions = DoomOptions::new();

        let dev_option: Option<&DoomOption> = get_option_by_name(&doom_options, "-devparm");
        assert!(dev_option.is_some());
    }

    #[test]
    fn test_get_option_by_name_mut_returns_value() {
        let mut doom_options: DoomOptions = DoomOptions::new();

        let dev_option: Option<&mut DoomOption> =
            get_option_by_name_mut(&mut doom_options, "-devparm");
        assert!(dev_option.is_some());
    }

    // set_options should still work
    // even if no cmd args are passed into it
    // Which means no options will be set
    #[test]
    fn test_set_options_works_with_no_cmd_args() {
        let mut doom_options: DoomOptions = DoomOptions::new();
        let cmd_args: Vec<&str> = Vec::new();
        set_options(&mut doom_options, cmd_args);
    }

    // All options should start with a -,
    // and the first option is no exception.
    // Needed to parse our options correctly
    #[test]
    #[should_panic]
    fn test_set_options_with_invalid_option_at_beginning() {
        let mut doom_options: DoomOptions = DoomOptions::new();
        let cmd_args: Vec<&str> = vec!["test", "fail", "fail"];
        set_options(&mut doom_options, cmd_args);
    }

    // We want to fail if we pass too many values for an option
    // In this case its wart that should fail since it requires
    // a max of two values but we provided 3.
    #[test]
    #[should_panic]
    fn test_set_options_with_too_many_option_values() {
        let mut doom_options: DoomOptions = DoomOptions::new();
        let cmd_args: Vec<&str> = vec![
            "-devparm",
            "-record",
            "test",
            "-wart",
            "1",
            "1",
            "1",
            "-shdev",
            "-file",
            "file1",
            "path/to/file2",
            "/path/to/file3",
            "-comdev",
        ];
        set_options(&mut doom_options, cmd_args);
    }

    // We want to fail if we pass not enough values for an option
    // In this case its wart that should fail since it requires
    // a min of two values but we provided 1.
    #[test]
    #[should_panic]
    fn test_set_options_with_not_enough_option_values() {
        let mut doom_options: DoomOptions = DoomOptions::new();
        let cmd_args: Vec<&str> = vec![
            "-wart",
            "1",
            "-devparm",
            "-record",
            "test",
            "-shdev",
            "-file",
            "file1",
            "path/to/file2",
            "/path/to/file3",
            "-comdev",
        ];
        set_options(&mut doom_options, cmd_args);
    }

    // Custom args, for now we don't support custom args but
    // it might be nice to figure out how to do it
    #[test]
    #[should_panic]
    fn test_set_options_with_custom_args() {
        let mut doom_options: DoomOptions = DoomOptions::new();
        let cmd_args: Vec<&str> = vec!["-test", "-test1value", "1", "-test3value", "1", "2", "3"];

        set_options(&mut doom_options, cmd_args);
    }

    // Existing args, values should be set,
    // no new structs should be created
    #[test]
    fn test_set_existing_doom_option_values() {
        let mut doom_options = DoomOptions::new();

        let cmd_args: Vec<&str> = vec![
            "-wart",
            "1",
            "1",
            "-devparm",
            "-record",
            "test",
            "-shdev",
            "-file",
            "file1",
            "path/to/file2",
            "/path/to/file3",
            "-comdev",
        ];

        set_options(&mut doom_options, cmd_args);

        let devparm_option: Option<&DoomOption> = get_option_by_name(&doom_options, "-devparm");
        assert!(devparm_option.is_some());

        let devparm: &DoomOption = devparm_option.unwrap();
        assert!(devparm.values.as_ref().unwrap().eq("true"));
        assert!(devparm.enabled());

        let shdev_option: Option<&DoomOption> = get_option_by_name(&doom_options, "-devparm");
        assert!(shdev_option.is_some());

        let shdev: &DoomOption = shdev_option.unwrap();
        assert!(shdev.values.as_ref().unwrap().eq("true"));
        assert!(shdev.enabled());

        let wart_option: Option<&DoomOption> = get_option_by_name(&doom_options, "-wart");
        assert!(wart_option.is_some());

        let wart: &DoomOption = wart_option.unwrap();
        assert!(wart.values.as_ref().unwrap().eq("1 1"));
        assert!(wart.enabled());

        let record_option: Option<&DoomOption> = get_option_by_name(&doom_options, "-record");
        assert!(record_option.is_some());

        let record: &DoomOption = record_option.unwrap();
        assert!(record.values.as_ref().unwrap().eq("test"));
        assert!(record.enabled());

        let file_option: Option<&DoomOption> = get_option_by_name(&doom_options, "-file");
        assert!(file_option.is_some());

        let file: &DoomOption = file_option.unwrap();
        assert!(file
            .values
            .as_ref()
            .unwrap()
            .eq("file1 path/to/file2 /path/to/file3"));
        assert!(file.enabled());

        let comdev_option: Option<&DoomOption> = get_option_by_name(&doom_options, "-comdev");
        assert!(comdev_option.is_some());

        let comdev: &DoomOption = comdev_option.unwrap();
        assert!(comdev.values.as_ref().unwrap().eq("true"));
        assert!(comdev.enabled());
    }
}
