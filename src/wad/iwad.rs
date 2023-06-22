use crate::{
    config::{GameType, Language},
    option::DoomOptions,
    util,
};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub struct IWADInfo<'a> {
    pub name: &'a str,
    pub game_type: GameType,
    pub language: Language,
}

pub static VALID_IWADS: [IWADInfo; 7] = [
    IWADInfo {
        name: "doom2f.wad",
        game_type: GameType::DoomII,
        language: Language::French,
    },
    IWADInfo {
        name: "doom2.wad",
        game_type: GameType::DoomII,
        language: Language::English,
    },
    IWADInfo {
        name: "plutonia.wad",
        game_type: GameType::DoomII,
        language: Language::English,
    },
    IWADInfo {
        name: "tnt.wad",
        game_type: GameType::DoomII,
        language: Language::English,
    },
    IWADInfo {
        name: "doomu.wad",
        game_type: GameType::UltimateDoom,
        language: Language::English,
    },
    IWADInfo {
        name: "doom.wad",
        game_type: GameType::DoomIRegistered,
        language: Language::English,
    },
    IWADInfo {
        name: "doom1.wad",
        game_type: GameType::DoomIShareware,
        language: Language::English,
    },
];

fn generate_dev_path_bufs(paths: &[String]) -> Vec<PathBuf> {
    paths
        .iter()
        .map(|path| -> PathBuf {
            let mut exe_parent_path: PathBuf = util::exe_parent_path();
            exe_parent_path.push(path);
            exe_parent_path
        })
        .collect()
}
pub fn find_valid_iwad_file_paths(
    wad_files_dir: &Path,
    doom_options: &DoomOptions,
) -> Vec<PathBuf> {
    let mut files_to_process: Vec<PathBuf> = Vec::new();
    let dev_data_file_prefix: &str = crate::config::DEV_DATA_FILE_PREFIX;
    let dev_maps_folder_prefix: &str = crate::config::DEV_MAPS_FOLDER_PREFIX;

    if doom_options.is_option_enabled("-shdev") {
        return generate_dev_path_bufs(&[
            format!("{}doom1.wad", dev_data_file_prefix),
            format!("{}cdata/texture1.lmp", dev_maps_folder_prefix),
            format!("{}cdata/pnames.lmp", dev_maps_folder_prefix),
        ]);
    } else if doom_options.is_option_enabled("-regdev") {
        return generate_dev_path_bufs(&[
            format!("{}doom.wad", dev_data_file_prefix),
            format!("{}cdata/texture1.lmp", dev_maps_folder_prefix),
            format!("{}cdata/texture2.lmp", dev_maps_folder_prefix),
            format!("{}cdata/pnames.lmp", dev_maps_folder_prefix),
        ]);
    } else if doom_options.is_option_enabled("-comdev") {
        return generate_dev_path_bufs(&[
            format!("{}doom2.wad", dev_data_file_prefix),
            format!("{}cdata/texture1.lmp", dev_maps_folder_prefix),
            format!("{}cdata/pnames.lmp", dev_maps_folder_prefix),
        ]);
    }

    let mut iwad_file_path: PathBuf = wad_files_dir.to_path_buf();

    for iwad in &VALID_IWADS {
        iwad_file_path.push(iwad.name);

        if iwad_file_path.exists() {
            files_to_process.push(iwad_file_path);
            break;
        }

        iwad_file_path.pop();
    }

    files_to_process
}

pub fn get_iwad_name_from_iwad_paths(iwad_paths: &[PathBuf]) -> &str {
    iwad_paths
        .iter()
        .find_map(|path_buf| {
            if path_buf
                .extension()
                .map_or(false, |extension| extension.eq("wad"))
            {
                path_buf.file_name().unwrap_or(OsStr::new("")).to_str()
            } else {
                None
            }
        })
        .unwrap_or("")
}

#[cfg(test)]
mod tests {
    use crate::option::DoomOptions;
    use crate::wad::iwad::find_valid_iwad_file_paths;
    use std::path::PathBuf;

    #[test]
    fn test_find_valid_iwad_file_paths_returns_correct_paths_for_shdev_option() {
        let doom_options: DoomOptions = DoomOptions::new(vec![String::from("-shdev")]);
        let expected_shdev_path_values: &[String] = &[
            format!("{}doom1.wad", crate::config::DEV_DATA_FILE_PREFIX),
            format!(
                "{}cdata/texture1.lmp",
                crate::config::DEV_MAPS_FOLDER_PREFIX
            ),
            format!("{}cdata/pnames.lmp", crate::config::DEV_MAPS_FOLDER_PREFIX),
        ];

        let actual_shdev_path_values: Vec<PathBuf> =
            find_valid_iwad_file_paths(&PathBuf::from(""), &doom_options);

        for expected_value in expected_shdev_path_values {
            assert!(actual_shdev_path_values
                .iter()
                .any(|x| x.ends_with(expected_value)))
        }

        assert_eq!(
            actual_shdev_path_values.len(),
            expected_shdev_path_values.len()
        );
    }

    #[test]
    fn test_find_valid_iwad_file_paths_returns_correct_paths_for_regdev_option() {
        let doom_options: DoomOptions = DoomOptions::new(vec![String::from("-regdev")]);
        let expected_regdev_path_values: &[String] = &[
            format!("{}doom.wad", crate::config::DEV_DATA_FILE_PREFIX),
            format!(
                "{}cdata/texture1.lmp",
                crate::config::DEV_MAPS_FOLDER_PREFIX
            ),
            format!(
                "{}cdata/texture2.lmp",
                crate::config::DEV_MAPS_FOLDER_PREFIX
            ),
            format!("{}cdata/pnames.lmp", crate::config::DEV_MAPS_FOLDER_PREFIX),
        ];

        let actual_regdev_path_values: Vec<PathBuf> =
            find_valid_iwad_file_paths(&PathBuf::from(""), &doom_options);

        for expected_value in expected_regdev_path_values {
            assert!(actual_regdev_path_values
                .iter()
                .any(|x| x.ends_with(expected_value)))
        }

        assert_eq!(
            actual_regdev_path_values.len(),
            expected_regdev_path_values.len()
        );
    }

    #[test]
    fn test_find_valid_iwad_file_paths_returns_correct_paths_for_comdev_option() {
        let doom_options: DoomOptions = DoomOptions::new(vec![String::from("-comdev")]);
        let expected_comdev_path_values: &[String] = &[
            format!("{}doom2.wad", crate::config::DEV_DATA_FILE_PREFIX),
            format!(
                "{}cdata/texture1.lmp",
                crate::config::DEV_MAPS_FOLDER_PREFIX
            ),
            format!("{}cdata/pnames.lmp", crate::config::DEV_MAPS_FOLDER_PREFIX),
        ];

        let actual_comdev_path_values: Vec<PathBuf> =
            find_valid_iwad_file_paths(&PathBuf::from(""), &doom_options);

        for expected_value in expected_comdev_path_values {
            assert!(actual_comdev_path_values
                .iter()
                .any(|x| x.ends_with(expected_value)))
        }

        assert_eq!(
            actual_comdev_path_values.len(),
            expected_comdev_path_values.len()
        );
    }

    #[test]
    fn test_find_valid_iwad_file_paths_returns_path_for_iwad() {
        let mut wad_files_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        wad_files_dir.push("tests/resource");

        let files_to_process: Vec<PathBuf> =
            find_valid_iwad_file_paths(&wad_files_dir, &DoomOptions::new(Vec::new()));

        let file_to_process: &PathBuf = files_to_process.get(0).unwrap();

        let mut expected_file_path: PathBuf = wad_files_dir.clone();
        expected_file_path.push("doom.wad");

        assert_eq!(files_to_process.len(), 1);
        assert_eq!(
            file_to_process.to_str().unwrap(),
            expected_file_path.to_str().unwrap()
        )
    }
}
