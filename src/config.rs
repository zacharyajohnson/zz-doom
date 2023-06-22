use std::env;
use std::path::PathBuf;

use crate::option::{DoomOption, DoomOptions};
use crate::util;
use crate::wad::iwad;

pub const DEV_DATA_FILE_PREFIX: &str = "devdata";
pub const DEV_CONFIG_FILE_NAME: &str = "devdatadefault.cfg";
pub const PROD_CONFIG_FILE_NAME: &str = ".doomrc";
pub const DEV_MAPS_FOLDER_PREFIX: &str = "devmaps";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    French,
}

impl Language {
    pub fn from_wad_file_name(wad_file_name: &str) -> Language {
        iwad::VALID_IWADS
            .iter()
            .find(|iwad| wad_file_name.contains(iwad.name))
            .map(|iwad| iwad.language.clone())
            .unwrap_or(Language::English)
    }
}

// Equivalent to GameMode in original source
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameType {
    // DOOM 1 Shareware(Was enum Shareware)
    DoomIShareware,
    // DOOM 1 Registered Copy(Was enum Registered)
    DoomIRegistered,
    // The Ultimate DOOM (Doom 1 with 1 extra mission/ 9 extra levels)
    // Was enum Retail
    UltimateDoom,
    // DOOM II/ French Version
    // and Final DOOM(The Plutonia Experiment and TNT: Evilution)
    // Was Commercial enum
    DoomII,
    // Everything else
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameDifficulty {
    Baby,
    Easy,
    Medium,
    Hard,
    Nightmare,
}

impl GameType {
    pub fn from_wad_file_name(wad_file_name: &str) -> GameType {
        iwad::VALID_IWADS
            .iter()
            .find(|iwad| wad_file_name.contains(iwad.name))
            .map(|iwad| iwad.game_type.clone())
            .unwrap_or(GameType::Unknown)
    }
}

pub struct Config<'a> {
    pub config_file_path: PathBuf,
    pub wad_files_dir: PathBuf,
    pub engine_version: &'a str,
    pub game_type: GameType,
    pub game_difficulty: GameDifficulty,
    pub language: Language,
    pub auto_start: bool,
    pub start_episode: u32,
}

impl<'a> Config<'a> {
    pub fn game_title(&self) -> String {
        match self.game_type {
            GameType::DoomIShareware => format!(
                "                         DOOM Shareware Startup v{}                         ",
                self.engine_version
            ),
            GameType::DoomIRegistered => format!(
                "                         DOOM Registered Startup v{}                         ",
                self.engine_version
            ),
            GameType::UltimateDoom => format!(
                "                         The Ultimate DOOM Startup v{}                         ",
                self.engine_version
            ),
            GameType::DoomII => format!(
                "                         DOOM 2: Hell on Earth v{}                         ",
                self.engine_version
            ),
            GameType::Unknown => format!(
                "                         Public DOOM - v{}                         ",
                self.engine_version
            ),
        }
    }

    pub fn set_game_type_by_iwad_paths(&mut self, iwad_paths: &[PathBuf]) {
        let iwad_name = iwad::get_iwad_name_from_iwad_paths(iwad_paths);
        self.game_type = GameType::from_wad_file_name(iwad_name);
    }

    pub fn set_language_by_iwad_paths(&mut self, iwad_paths: &[PathBuf]) {
        let iwad_name: &str = iwad::get_iwad_name_from_iwad_paths(iwad_paths);
        self.language = Language::from_wad_file_name(iwad_name);
    }

    pub fn new(doom_options: &DoomOptions) -> Self {
        let auto_start: bool = is_auto_start(doom_options);

        let game_difficulty: GameDifficulty = if doom_options.is_option_enabled("-skill") {
            let skill_option: &DoomOption = doom_options.get_option_by_name("-skill").unwrap();
            match skill_option
                .values
                .get(0)
                .unwrap_or_else(|| panic!("Unable to get value for -skill option"))
                .as_str()
            {
                "1" => GameDifficulty::Baby,
                "2" => GameDifficulty::Easy,
                "3" => GameDifficulty::Medium,
                "4" => GameDifficulty::Hard,
                "5" => GameDifficulty::Nightmare,
                _ => panic!("Invalid value for -skill option. Valid range is 1-5"),
            }
        } else {
            GameDifficulty::Medium
        };

        let start_episode: u32 = if doom_options.is_option_enabled("-episode") {
            let episode_option: &DoomOption = doom_options.get_option_by_name("-episode").unwrap();
            let value: u32 = episode_option
                .values
                .get(0)
                .unwrap_or_else(|| panic!("Unable to get value for -episode option"))
                .parse::<u32>()
                .unwrap_or_else(|_e| panic!("Unable to parse -episode value to number"));
            value
        } else {
            1
        };

        //TODO Put warp/wart flag logic here so it overrides the -episode flag values like in the original

        let config_file_path = if doom_options.is_option_enabled("-shdev")
            || doom_options.is_option_enabled("-regdev")
            || doom_options.is_option_enabled("-comdev")
        {
            let mut path: PathBuf = util::exe_parent_path();
            path.push(DEV_CONFIG_FILE_NAME);
            path
        } else {
            let home_dir_env: &str = match env::consts::FAMILY {
                "windows" => "USERPROFILE",
                _ => "HOME",
            };
            env::var(home_dir_env)
                .map(PathBuf::from)
                .map(|mut path| {
                    println!("Home directory set to: {}", path.display());
                    path.push(PROD_CONFIG_FILE_NAME);
                    path
                })
                .unwrap_or_else(|_| {
                    panic!(
                        "Unable to find home directory.\n
                        Please set the USERPROFILE environment variable if on Windows or the 
                        HOME environment variable if on Unix"
                    )
                })
        };

        println!(
            "Setting config file location to {}",
            config_file_path.display()
        );

        let wad_files_dir = env::var("DOOMWADDIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| util::exe_parent_path());

        println!("Setting wad directory to {}", wad_files_dir.display());

        Config {
            config_file_path,
            wad_files_dir,
            game_difficulty,
            auto_start,
            start_episode,
            ..Default::default()
        }
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            config_file_path: Default::default(),
            wad_files_dir: Default::default(),
            engine_version: env!("CARGO_PKG_VERSION"),
            game_type: GameType::Unknown,
            game_difficulty: GameDifficulty::Medium,
            language: Language::English,
            auto_start: false,
            start_episode: 1,
        }
    }
}

fn is_auto_start(doom_options: &DoomOptions) -> bool {
    doom_options.is_option_enabled("-skill") || doom_options.is_option_enabled("-episode")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new_defaults_are_correct_values() {
        let config: Config = Config::new(&DoomOptions::new(Vec::new()));
        assert_eq!(config.engine_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(config.game_type, GameType::Unknown);
        assert_eq!(config.language, Language::English);
        assert_eq!(config.game_difficulty, GameDifficulty::Medium);
        assert_eq!(config.auto_start, false);
        assert_eq!(config.start_episode, 1);
    }

    #[test]
    fn test_config_new_game_difficulty_set_based_on_skill_option_value() {
        let valid_skill_values: [&str; 5] = ["1", "2", "3", "4", "5"];

        for value in valid_skill_values {
            let cmd_args: Vec<String> = vec![String::from("-skill"), String::from(value)];
            let doom_options: DoomOptions = DoomOptions::new(cmd_args);

            let enum_value: u8 = value.parse::<u8>().unwrap() - 1;

            let config: Config = Config::new(&doom_options);
            assert_eq!(config.game_difficulty as u8, enum_value);
            assert_eq!(config.auto_start, true);
        }
    }

    #[test]
    #[should_panic]
    fn test_config_new_game_difficulty_with_skill_option_value_below_min_value() {
        let cmd_args: Vec<String> = vec![String::from("-skill"), String::from("0")];
        let doom_options: DoomOptions = DoomOptions::new(cmd_args);
        Config::new(&doom_options);
    }

    #[test]
    #[should_panic]
    fn test_config_new_game_difficulty_with_skill_option_value_above_max_value() {
        let cmd_args: Vec<String> = vec![String::from("-skill"), String::from("6")];
        let doom_options: DoomOptions = DoomOptions::new(cmd_args);
        Config::new(&doom_options);
    }

    #[test]
    fn test_config_new_start_episode_set_based_on_episode_option_value() {
        let cmd_args: Vec<String> = vec![String::from("-episode"), String::from("5")];
        let doom_options: DoomOptions = DoomOptions::new(cmd_args);
        let config: Config = Config::new(&doom_options);

        assert_eq!(config.start_episode, 5);
        assert_eq!(config.auto_start, true);
    }

    #[test]
    #[should_panic]
    fn test_config_new_start_episode_when_invalid_number_for_episode_option_value() {
        let cmd_args: Vec<String> = vec![String::from("-episode"), String::from("Hello")];
        let doom_options: DoomOptions = DoomOptions::new(cmd_args);
        Config::new(&doom_options);
    }

    #[test]
    fn test_config_new_config_file_path_when_dev_options_are_set() {
        let dev_options: [&str; 3] = ["-shdev", "-comdev", "-regdev"];

        let mut exe_parent_path = util::exe_parent_path();
        exe_parent_path.push(DEV_CONFIG_FILE_NAME);

        for option in dev_options {
            let cmd_args: Vec<String> = vec![String::from(option)];
            let doom_options: DoomOptions = DoomOptions::new(cmd_args);
            let config: Config = Config::new(&doom_options);
            assert_eq!(config.config_file_path, exe_parent_path);
        }
    }

    #[cfg(target_family = "windows")]
    #[test]
    fn test_config_new_config_file_path_is_in_home_dir_windows() {
        temp_env::with_var("USERPROFILE", Some("home"), || {
            let config: Config = Config::new(&DoomOptions::new(Vec::new()));
            let mut home_dir: PathBuf = PathBuf::from("home");
            home_dir.push(".doomrc");

            assert_eq!(config.config_file_path, home_dir);
        });
    }

    #[cfg(target_family = "windows")]
    #[test]
    #[should_panic]
    fn test_config_new_when_userprofile_environment_variable_is_not_set_windows() {
        temp_env::with_var_unset("USERPROFILE", || {
            Config::new(&DoomOptions::new(Vec::new()));
        });
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn test_config_new_config_file_path_is_in_home_dir_unix() {
        temp_env::with_var("HOME", Some("home"), || {
            let config: Config = Config::new(&DoomOptions::new(Vec::new()));
            let mut home_dir: PathBuf = PathBuf::from("home");
            home_dir.push(".doomrc");

            assert_eq!(config.config_file_path, home_dir);
        });
    }

    #[cfg(target_family = "unix")]
    #[test]
    #[should_panic]
    fn test_config_new_when_home_environment_variable_is_not_set_unix() {
        temp_env::with_var_unset("HOME", || {
            Config::new(&DoomOptions::new(Vec::new()));
        });
    }

    #[test]
    fn test_config_new_wad_files_dir_defaults_to_exe_current_dir() {
        temp_env::with_var_unset("DOOMWADDIR", || {
            let config: Config = Config::new(&DoomOptions::new(Vec::new()));
            let exe_parent_path = util::exe_parent_path();

            assert_eq!(config.wad_files_dir, exe_parent_path);
        });
    }

    #[test]
    fn test_config_new_wad_files_dir_is_set_to_doomwaddir_env_variable_if_set() {
        temp_env::with_var("DOOMWADDIR", Some("test"), || {
            let config: Config = Config::new(&DoomOptions::new(Vec::new()));
            assert_eq!(config.wad_files_dir, PathBuf::from("test"));
        });
    }

    #[test]
    fn test_config_game_title_returns_correct_values() {
        let mut config: Config = Default::default();

        config.game_type = GameType::DoomIShareware;
        assert!(config.game_title().contains("DOOM Shareware Startup"));

        config.game_type = GameType::DoomIRegistered;
        assert!(config.game_title().contains("DOOM Registered Startup"));

        config.game_type = GameType::UltimateDoom;
        assert!(config.game_title().contains("The Ultimate DOOM Startup"));

        config.game_type = GameType::DoomII;
        assert!(config.game_title().contains("DOOM 2: Hell on Earth"));

        config.game_type = GameType::Unknown;
        assert!(config.game_title().contains("Public DOOM"));
    }

    #[test]
    fn test_config_set_language_by_iwad_paths_sets_correct_values() {
        let mut config: Config = Default::default();

        // Test that they are set to the correct game type based on wad name
        config.set_language_by_iwad_paths(&[PathBuf::from("doom.wad")]);
        assert_eq!(config.language, Language::English);

        // Test that it still works as long as it has iwad.wad
        // in the file name(-shdev,-regdev,-comdev)
        config.set_language_by_iwad_paths(&[
            PathBuf::from("lump.lmp"),
            PathBuf::from("path/to/wad/devdatadoom.wad"),
        ]);
        assert_eq!(config.language, Language::English);

        config.set_language_by_iwad_paths(&[PathBuf::from("doom2f.wad")]);
        assert_eq!(config.language, Language::French);

        config.set_language_by_iwad_paths(&[
            PathBuf::from("lump.lmp"),
            PathBuf::from("path/to/wad/devdatadoom2f.wad"),
            PathBuf::from("path/to/wad/unknown.wad"),
        ]);
        assert_eq!(config.language, Language::French);
    }
    #[test]
    fn test_config_set_game_type_by_iwad_paths_sets_correct_values() {
        let mut config: Config = Default::default();

        // Test that they are set to the correct game type based on wad name
        config.set_game_type_by_iwad_paths(&[PathBuf::from("doom1.wad")]);
        assert_eq!(config.game_type, GameType::DoomIShareware);

        config.set_game_type_by_iwad_paths(&[PathBuf::from("doom.wad")]);
        assert_eq!(config.game_type, GameType::DoomIRegistered);

        config.set_game_type_by_iwad_paths(&[PathBuf::from("doomu.wad")]);
        assert_eq!(config.game_type, GameType::UltimateDoom);

        config.set_game_type_by_iwad_paths(&[PathBuf::from("doom2.wad")]);
        assert_eq!(config.game_type, GameType::DoomII);

        config.set_game_type_by_iwad_paths(&[PathBuf::from("doom2f.wad")]);
        assert_eq!(config.game_type, GameType::DoomII);

        config.set_game_type_by_iwad_paths(&[PathBuf::from("plutonia.wad")]);
        assert_eq!(config.game_type, GameType::DoomII);

        config.set_game_type_by_iwad_paths(&[PathBuf::from("tnt.wad")]);
        assert_eq!(config.game_type, GameType::DoomII);

        config.set_game_type_by_iwad_paths(&[PathBuf::from("unknown.wad")]);
        assert_eq!(config.game_type, GameType::Unknown);

        // Test if we don't find a wad it defaults to Unknown
        config.set_game_type_by_iwad_paths(&[PathBuf::from("unknown.lmp")]);
        assert_eq!(config.game_type, GameType::Unknown);

        // Test that it still works with a file path
        config.set_game_type_by_iwad_paths(&[PathBuf::from("path/to/wad/doom2.wad")]);
        assert_eq!(config.game_type, GameType::DoomII);

        // Test that it still works with multiple files(One wad plus multiple lumps)
        config.set_game_type_by_iwad_paths(&[
            PathBuf::from("lump.lmp"),
            PathBuf::from("path/to/wad/doom2.wad"),
        ]);
        assert_eq!(config.game_type, GameType::DoomII);

        config.set_game_type_by_iwad_paths(&[
            PathBuf::from("lump.lmp"),
            PathBuf::from("path/to/wad/doom.wad"),
            PathBuf::from("path/to/wad/doom2.wad"),
        ]);
        assert_eq!(config.game_type, GameType::DoomIRegistered);

        // Test that it still works as long as it has iwad.wad
        // in the file name(-shdev,-regdev,-comdev)
        config.set_game_type_by_iwad_paths(&[
            PathBuf::from("lump.lmp"),
            PathBuf::from("path/to/wad/devdatadoom.wad"),
            PathBuf::from("path/to/wad/devdatadoom2.wad"),
        ]);
        assert_eq!(config.game_type, GameType::DoomIRegistered);
    }

    #[test]
    fn test_game_type_from_wad_file_name_returns_correct_values() {
        for iwad in &iwad::VALID_IWADS {
            assert_eq!(GameType::from_wad_file_name(iwad.name), iwad.game_type);
        }

        // Test that the method will return the correct type if we
        // get passed in a static wad name for the -shdev, -regdev, or -comdev options
        assert_eq!(
            GameType::from_wad_file_name("devdatadoom1.wad"),
            GameType::DoomIShareware
        );
    }

    #[test]
    fn test_language_from_wad_file_name_returns_correct_values() {
        for iwad in &iwad::VALID_IWADS {
            assert_eq!(Language::from_wad_file_name(iwad.name), iwad.language);
        }

        assert_eq!(
            Language::English,
            Language::from_wad_file_name("devdatadoom.wad")
        );
    }
}
