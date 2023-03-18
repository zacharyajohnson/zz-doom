use crate::{
    config::{GameType, Language},
    option::DoomOptions,
    util,
};
use std::path::{Path, PathBuf};

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
