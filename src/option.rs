use std::ops::RangeInclusive;

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
        let mut doom_options: DoomOptions = DoomOptions {
            options: DEFAULT_OPTIONS
                .iter()
                .map(|x| -> DoomOption {
                    DoomOption {
                        name: x.0,
                        values: None,
                        min_num_values: *x.1.start(),
                        max_num_values: *x.1.end(),
                    }
                })
                .collect(),
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

    pub fn is_option_enabled(&self, option_name: &str) -> bool {
        match self.get_option_by_name(option_name) {
            Some(option) => option.enabled(),
            None => false,
        }
    }

    pub fn is_dev_option_enabled(&self) -> bool {
        self.is_option_enabled("-shdev")
            || self.is_option_enabled("-regdev")
            || self.is_option_enabled("-comdev")
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

    // The DEFAULT_OPTIONS variable contains the original
    // command line flags Doom shipped with, so we will support them
    // and set them up. The constructor for DoomOptions should insert
    // them for us. This is with no cmd line args passed in to check if
    // they are always there.
    #[test]
    fn test_doom_options_new_intializes_with_default_doom_options() {
        let doom_options: DoomOptions = DoomOptions::new(Vec::new());

        assert!(doom_options.options.iter().all(|option| {
            DEFAULT_OPTIONS.iter().any(|x| {
                x.0 == option.name
                    && *x.1.start() == option.min_num_values
                    && *x.1.end() == option.max_num_values
            }) && !option.enabled()
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
        let cmd_args: Vec<String> = vec![
            "-devparm".to_string(),
            "-record".to_string(),
            "test".to_string(),
            "-wart".to_string(),
            "1".to_string(),
            "1".to_string(),
        ];

        let doom_options: DoomOptions = DoomOptions::new(cmd_args);

        // Test an option that does exist that was passed in returns true
        assert!(doom_options.is_option_enabled("-devparm"));
        assert!(doom_options.is_option_enabled("-record"));
        assert!(doom_options.is_option_enabled("-wart"));

        // Test an option that does exist but wasn't passed in returns false
        assert!(!doom_options.is_option_enabled("-shdev"));
        // Test an option that doesn't exist returns false
        assert!(!doom_options.is_option_enabled("-test"));
    }

    #[test]
    fn test_doom_options_is_dev_option_enabled_returns_correct_values() {
        let dev_options: Vec<String> = vec![
            "-shdev".to_string(),
            "-regdev".to_string(),
            "-comdev".to_string(),
        ];

        for dev_option in dev_options {
            let doom_options: DoomOptions = DoomOptions::new(vec![dev_option]);
            assert!(doom_options.is_dev_option_enabled());
        }
    }

    #[test]
    fn test_get_option_by_name_mut_returns_value() {
        let mut doom_options: DoomOptions = DoomOptions::new(Vec::new());

        let dev_option: Option<&mut DoomOption> = doom_options.get_option_by_name_mut("-devparm");
        assert!(dev_option.is_some());
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
    fn test_doom_options_new_sets_doom_option_values_based_on_cmd_args() {
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
    // such as tab or |, (anything less then ascii code for space or
    // anything more then ascii code z) the line will be ignored and
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
