use std::mem::ManuallyDrop;
use std::{
    ffi::OsString,
    fmt::{self, Display},
    fs::{File, Metadata},
    io::{self, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

pub mod iwad;

const LUMP_FILE_MAX_NAME_LENGTH: usize = 8;
const RELOAD_FILE_PREFIX: &str = "~";

pub struct DoomFile {
    pub file_handle: ManuallyDrop<File>,
    pub lumps: Vec<Lump>,
}

struct FileInfo {
    pub path: PathBuf,
    pub extension: OsString,
    pub name: OsString,
    pub should_reload: bool,
    pub size: u32,
}

impl FileInfo {
    pub fn from(file_path: &Path) -> Result<Self, WadError> {
        // TODO I need to convert to a string and then back into a path buf
        // Because strip_prefix on path buf gets rid of the trailing slash for
        // some reason in an absolute path. starts_with on a path buffer also doesn't work on a file
        // path that is just a file name in the current dir(~test.wad).
        // Try and find a better way to do this
        let mut path_str: String = String::from(file_path.to_string_lossy());
        let should_reload: bool = path_str.starts_with(RELOAD_FILE_PREFIX);

        // Need to strip ~ from beginning if its
        // a reloadable file
        if should_reload {
            println!("\nDetected reloadable file: {}", file_path.display());

            path_str = path_str
                .strip_prefix(RELOAD_FILE_PREFIX)
                .unwrap()
                .to_owned();

            println!("New file path: {}", path_str);
        }

        let path = PathBuf::from(path_str);
        let file_meta_data: Metadata = path.metadata()?;
        let size: u32 = file_meta_data.len().try_into().unwrap();

        let extension: OsString = match path.extension() {
            Some(file_extension) => file_extension,
            None => return Err(WadError::NoFileExtension(path)),
        }
        .to_os_string();

        let name: OsString = path.file_stem().unwrap().to_owned();

        Ok(FileInfo {
            path,
            extension,
            name,
            should_reload,
            size,
        })
    }
}

pub struct Lump {
    pub name: String,
    pub file_path: PathBuf,
    pub file_position: u32,
    pub size: u32,
    pub should_reload: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum WadError {
    IoError(String),
    NoFileExtension(PathBuf),
    InvalidFileExtension(PathBuf),
}

impl Display for WadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoFileExtension(path) => write!(
                f,
                "No valid extension for {}. Valid extensions are .wad/.lmp",
                path.display()
            ),
            Self::InvalidFileExtension(path) => write!(
                f,
                "Invalid extension for {}. Valid extensions are .wad/.lmp",
                path.display()
            ),
            Self::IoError(io_error_reason) => write!(f, "{}", io_error_reason),
        }
    }
}

impl From<io::Error> for WadError {
    fn from(value: io::Error) -> Self {
        WadError::IoError(format!("{:?}", value))
    }
}

pub struct WadHeader {
    pub id: WadID,
    pub num_lumps: u32,
    pub lump_location_offset: u32,
}

impl WadHeader {
    fn from(file: &mut File) -> Result<Self, WadError> {
        println!("Processing Wad Header...");

        let mut wad_header_data: [u8; 12] = [0; 12];
        file.read_exact(&mut wad_header_data)?;

        let id: [u8; 4] = wad_header_data[0..=3].try_into().unwrap();
        let num_lumps: [u8; 4] = wad_header_data[4..=7].try_into().unwrap();
        let lump_location_offset: [u8; 4] = wad_header_data[8..=11].try_into().unwrap();

        let id: WadID = WadID::from(String::from_utf8_lossy(&id).to_string());
        let num_lumps: u32 = u32::from_le_bytes(num_lumps);
        let lump_location_offset: u32 = u32::from_le_bytes(lump_location_offset);

        println!(
            "Creating Wad Header - ID: {}, Number of Lumps: {}, Lump Location Offset: {}",
            id.to_str(),
            num_lumps,
            lump_location_offset
        );

        Ok(WadHeader {
            id,
            num_lumps,
            lump_location_offset,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum WadID {
    Iwad,
    Pwad,
}

impl WadID {
    // TODO This was a panic in the source code of the original,
    // So we will keep it the same for now.
    // Once we port it over one for one we can decide if we want to keep it the same
    fn from(value: String) -> Self {
        match value.as_ref() {
            "IWAD" => WadID::Iwad,
            "PWAD" => WadID::Pwad,
            _ => panic!("Invalid value for Wad ID: {}. Can only be IWAD/PWAD", value),
        }
    }
    fn to_str(&self) -> &str {
        match self {
            WadID::Iwad => "IWAD",
            WadID::Pwad => "PWAD",
        }
    }
}

pub fn process_file(file_path: &Path) -> Result<DoomFile, WadError> {
    let file_info: FileInfo = FileInfo::from(file_path)?;

    println!("\nAdding {}", file_info.path.display());

    if file_info.extension == "wad" {
        process_wad_file(file_info)
    } else if file_info.extension == "lmp" {
        if file_info.name.len() > LUMP_FILE_MAX_NAME_LENGTH {
            // TODO This was an panic in the source code of the original,
            // So we will keep it the same for now.
            // Once we port it over one for one we can decide if we want to keep it the same
            //return Err(WadError::ExceedsLumpFileMaxNameLength(file_info.path, file_info.name.len()));
            panic!(
                "Invalid file name for lump file {}. Max length for file name is {}, actual is {}",
                file_info.path.display(),
                LUMP_FILE_MAX_NAME_LENGTH,
                file_info.name.len()
            );
        }
        process_lump_file(file_info)
    } else {
        Err(WadError::InvalidFileExtension(file_info.path))
    }
}

fn process_wad_file(file_info: FileInfo) -> Result<DoomFile, WadError> {
    println!("Processing wad file {}", file_info.path.display());
    let mut file: File = File::open(&file_info.path)?;

    let wad_header: WadHeader = WadHeader::from(&mut file)?;

    file.seek(SeekFrom::Start(wad_header.lump_location_offset.into()))
        .unwrap();

    let mut lumps: Vec<Lump> = Vec::new();

    for _ in 0..wad_header.num_lumps {
        let mut lump_data: [u8; 16] = [0; 16];
        file.read_exact(&mut lump_data)?;

        let file_position: [u8; 4] = lump_data[0..=3].try_into().unwrap();
        let size: [u8; 4] = lump_data[4..=7].try_into().unwrap();
        let name: [u8; 8] = lump_data[8..=15].try_into().unwrap();

        let file_position: u32 = u32::from_le_bytes(file_position);
        let size: u32 = u32::from_le_bytes(size);
        let name: String = String::from_utf8_lossy(&name).to_string();

        let lump: Lump = Lump {
            name,
            file_path: file_info.path.to_owned(),
            file_position,
            size,
            should_reload: file_info.should_reload,
        };

        lumps.push(lump);
    }

    println!("Wad file processing done for {}", file_info.path.display());
    Ok(DoomFile {
        file_handle: ManuallyDrop::new(file),
        lumps,
    })
}

fn process_lump_file(file_info: FileInfo) -> Result<DoomFile, WadError> {
    println!("Processing lump file {}", file_info.path.display());
    let name: String = String::from(file_info.name.to_string_lossy());
    let file: File = File::open(&file_info.path)?;

    let lump: Lump = Lump {
        name,
        file_path: file_info.path.to_owned(),
        file_position: 0,
        size: file_info.size,
        should_reload: file_info.should_reload,
    };

    //println!("Lump {} = file_handle: {}, file_pos: {}, size: {}, name: {}",i, lump.file_path.display(), lump.file_position, lump.size, lump.name);

    println!("Lump file processing done for {}", file_info.path.display());
    Ok(DoomFile {
        file_handle: ManuallyDrop::new(file),
        lumps: vec![lump],
    })
}

pub fn get_lump_data(doom_files: &mut Vec<DoomFile>, lump_name: &str) -> Vec<u8> {
    let (doom_file, lump_index): (&mut DoomFile, usize) =
        match doom_files.iter_mut().rev().find_map(|doom_file| {
            let lump_index = doom_file
                .lumps
                .iter()
                .rposition(|lump| lump.name.trim_end_matches('\0') == lump_name);
            lump_index.map(|index| (doom_file, index))
        }) {
            Some((doom_file, lump_index)) => (doom_file, lump_index),
            None => panic!("Unable to find data for lump {}", lump_name),
        };

    let lump: &Lump = doom_file.lumps.get(lump_index).unwrap();
    let mut lump_data: Vec<u8> = vec![0; usize::try_from(lump.size).unwrap()];

    if lump.should_reload {
        // This panicked in the original source code
        let lump_file: File = match File::open(&lump.file_path) {
            Ok(file) => file,
            Err(e) => panic!(
                "Unable to get lump data for lump {} located in {}. Error {}",
                lump.name,
                lump.file_path.display(),
                e
            ),
        };

        let mut lump_file: ManuallyDrop<File> = ManuallyDrop::new(lump_file);

        lump_file
            .seek(SeekFrom::Start(lump.file_position.into()))
            .unwrap();

        lump_file.read_exact(&mut lump_data).unwrap();

        unsafe { ManuallyDrop::drop(&mut doom_file.file_handle) };
        doom_file.file_handle = lump_file;
    } else {
        let lump_file: &mut ManuallyDrop<File> = &mut doom_file.file_handle;

        lump_file
            .seek(SeekFrom::Start(lump.file_position.into()))
            .unwrap();

        lump_file.read_exact(&mut lump_data).unwrap();
    };

    println!("test");
    lump_data
}

#[cfg(test)]
mod tests {
    use crate::wad::{
        get_lump_data, process_file, process_wad_file, DoomFile, FileInfo, Lump, WadError, WadID,
    };
    use std::collections::HashMap;
    use std::ffi::OsString;
    use std::fs::File;
    use std::mem::ManuallyDrop;
    use std::path::PathBuf;

    #[test]
    fn test_wad_error_implements_display_trait() {
        let wad_errors: Vec<WadError> = vec![
            WadError::InvalidFileExtension(PathBuf::from("")),
            WadError::IoError(String::from("test")),
            WadError::NoFileExtension(PathBuf::from("")),
        ];

        for wad_error in wad_errors {
            println!("{}", wad_error);
        }
    }

    #[test]
    fn test_wad_error_implements_debug_trait() {
        let wad_error: WadError = WadError::InvalidFileExtension(PathBuf::from("test/"));
        println!("{:?}", wad_error);
    }

    #[test]
    fn test_wad_error_from_supports_correct_exceptions() {
        // IO Error Conversion
        let _ = WadError::from(File::open(PathBuf::from("s")).err().unwrap());
    }

    #[test]
    fn test_wad_id_implements_display_trait() {
        println!("{:?}", WadID::Iwad);
    }

    #[test]
    fn test_wad_id_from_supports_correct_values() {
        let supported_values: HashMap<String, WadID> = HashMap::from([
            (String::from("IWAD"), WadID::Iwad),
            (String::from("PWAD"), WadID::Pwad),
        ]);

        for (supported_value, wad_id) in supported_values {
            assert_eq!(WadID::from(supported_value), wad_id);
        }
    }

    #[test]
    #[should_panic]
    fn test_wad_id_from_panics_on_unsupported_values() {
        WadID::from(String::from("test"));
    }

    #[test]
    fn test_wad_to_str_provides_correct_values() {
        assert_eq!(WadID::Iwad.to_str(), "IWAD");
        assert_eq!(WadID::Pwad.to_str(), "PWAD");
    }

    #[test]
    fn test_process_file_processes_wad_file() {
        let mut wad_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        wad_path.push("tests/resource/test.wad");

        let doom_file: DoomFile = process_file(&wad_path).unwrap();
        let lumps: Vec<Lump> = doom_file.lumps;
        let lump: &Lump = lumps.get(0).unwrap();

        assert_eq!(lumps.len(), 1);
        assert_eq!(lump.file_path, wad_path);
        assert_eq!(lump.name, "DATA\0\0\0\0");
        assert_eq!(lump.file_position, 12);
        assert_eq!(lump.size, 13);
        assert_eq!(lump.should_reload, false);
    }

    #[test]
    fn test_process_file_processes_lump_file() {
        let mut lump_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        lump_path.push("tests/resource/TEST.lmp");

        let doom_file: DoomFile = process_file(&lump_path).unwrap();
        let lumps: Vec<Lump> = doom_file.lumps;

        let lump: &Lump = lumps.get(0).unwrap();

        assert_eq!(lumps.len(), 1);
        assert_eq!(lump.file_path, lump_path);
        assert_eq!(lump.name, "TEST");
        assert_eq!(lump.file_position, 0);
        assert_eq!(lump.size, 0);
        assert_eq!(lump.should_reload, false);
    }

    #[test]
    fn test_process_file_processes_file_with_reload_prefix() {
        let mut wad_path: PathBuf = PathBuf::from(String::from("~") + env!("CARGO_MANIFEST_DIR"));
        wad_path.push("tests/resource/test.wad");

        process_file(&wad_path).unwrap();
    }

    #[test]
    fn test_process_file_returns_io_error_when_failing_to_read_wad_header_data() {
        let mut wad_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        wad_path.push("tests/resource/empty.wad");

        let wad_error: WadError = process_file(&wad_path).err().unwrap();
        assert_eq!(
            wad_error,
            WadError::IoError(String::from(
                "Error { kind: UnexpectedEof, message: \"failed to fill whole buffer\" }"
            ))
        );
    }

    #[test]
    fn test_process_file_returns_io_error_when_failing_to_read_file_metadata() {
        let mut wad_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        wad_path.push("unknown.wad");

        let wad_error: WadError = process_file(&wad_path).err().unwrap();

        // Need to check this way since the display message between window/linux is different
        assert!(wad_error
            .to_string()
            .contains("Os { code: 2, kind: NotFound"));
    }

    #[test]
    fn test_process_file_returns_io_error_when_wad_header_specifies_incorrect_lump_count() {
        let mut wad_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        wad_path.push("tests/resource/wad-header-only.wad");

        let wad_error: WadError = process_file(&wad_path).err().unwrap();
        assert_eq!(
            wad_error,
            WadError::IoError(String::from(
                "Error { kind: UnexpectedEof, message: \"failed to fill whole buffer\" }"
            ))
        );
    }

    #[test]
    #[should_panic]
    fn test_process_file_panics_with_lump_file_name_greater_than_max_limit() {
        let mut lump_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        lump_path.push("tests/resource/REALLYLONGASSNAME.lmp");

        process_file(&lump_path).unwrap();
    }

    #[test]
    fn test_process_file_returns_no_file_extension_error_for_directory_path() {
        let dir_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let wad_error: WadError = process_file(&dir_path).err().unwrap();
        assert_eq!(wad_error, WadError::NoFileExtension(dir_path));
    }
    #[test]
    fn test_process_file_returns_invalid_file_extension_error_for_text_file() {
        let mut text_file_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        text_file_path.push("tests/resource/test.txt");

        let wad_error: WadError = process_file(&text_file_path).err().unwrap();
        assert_eq!(wad_error, WadError::InvalidFileExtension(text_file_path));
    }

    #[test]
    fn test_process_wad_file_returns_io_error_when_failing_to_open_file() {
        let mut wad_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        wad_path.push("unknown.wad");

        let file_info: FileInfo = FileInfo {
            path: wad_path,
            extension: OsString::from(".wad"),
            name: OsString::from("unknown"),
            should_reload: false,
            size: 0,
        };
        let wad_error: WadError = process_wad_file(file_info).err().unwrap();

        // Need to check this way since the display message between window/linux is different
        assert!(wad_error
            .to_string()
            .contains("Os { code: 2, kind: NotFound"));
    }

    #[test]
    fn test_get_lump_data_returns_existing_lumps_data() {
        let mut file_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("tests/resource/test.wad");

        let file: File = File::open(&file_path).unwrap();

        let doom_file: DoomFile = DoomFile {
            file_handle: ManuallyDrop::new(file),
            lumps: vec![Lump {
                name: String::from("DATA\0\0\0\0"),
                file_path,
                file_position: 12,
                size: 13,
                should_reload: false,
            }],
        };

        let mut doom_files: Vec<DoomFile> = vec![doom_file];
        let lump_data: Vec<u8> = get_lump_data(&mut doom_files, "DATA");

        let hello_world: String = String::from_utf8(lump_data).unwrap();
        assert_eq!(hello_world, "Hello, World!");
    }

    #[test]
    #[should_panic]
    fn test_get_lump_data_panics_when_lump_data_not_found() {
        let mut file_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("tests/resource/test.wad");

        let file: File = File::open(&file_path).unwrap();

        let doom_file: DoomFile = DoomFile {
            file_handle: ManuallyDrop::new(file),
            lumps: vec![Lump {
                name: String::from("DATA\0\0\0\0"),
                file_path,
                file_position: 12,
                size: 13,
                should_reload: false,
            }],
        };

        let mut doom_files: Vec<DoomFile> = vec![doom_file];

        get_lump_data(&mut doom_files, "DOESNOTEXIST");
    }
    #[test]
    fn test_get_lump_data_returns_lump_data_when_lump_reloadable() {
        let mut file_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("tests/resource/test.wad");

        let file: File = File::open(&file_path).unwrap();

        let doom_file: DoomFile = DoomFile {
            file_handle: ManuallyDrop::new(file),
            lumps: vec![Lump {
                name: String::from("DATA\0\0\0\0"),
                file_path,
                file_position: 12,
                size: 13,
                should_reload: true,
            }],
        };

        let mut doom_files: Vec<DoomFile> = vec![doom_file];

        let lump_data: Vec<u8> = get_lump_data(&mut doom_files, "DATA");

        let hello_world: String = String::from_utf8(lump_data).unwrap();
        assert_eq!(hello_world, "Hello, World!");
    }

    #[test]
    #[should_panic]
    fn test_get_lump_data_panics_when_fails_to_find_file_when_lump_reloadable() {
        let mut file_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("tests/resource/test.wad");

        let file: File = File::open(&file_path).unwrap();

        let doom_file: DoomFile = DoomFile {
            file_handle: ManuallyDrop::new(file),
            lumps: vec![Lump {
                name: String::from("DATA\0\0\0\0"),
                file_path: PathBuf::from("does-not-exist.wad"),
                file_position: 12,
                size: 13,
                should_reload: true,
            }],
        };

        let mut doom_files: Vec<DoomFile> = vec![doom_file];

        get_lump_data(&mut doom_files, "DATA");
    }

    // Doom allows PWADs(Patch wads) to override the lump data of the main
    // IWAD. This is done by always checking the lumps in reverse order when
    // getting lump data. So overriding a lump is easy as adding a PWAD with the
    // -file command and having it added to the end of the doom_file list
    #[test]
    fn test_get_lump_data_from_must_recent_lump_in_doom_file_list() {
        let mut file_path1: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path1.push("tests/resource/test.wad");

        let file1: File = File::open(&file_path1).unwrap();

        let doom_file1: DoomFile = DoomFile {
            file_handle: ManuallyDrop::new(file1),
            lumps: vec![Lump {
                name: String::from("DATA\0\0\0\0"),
                file_path: file_path1,
                file_position: 12,
                size: 13,
                should_reload: false,
            }],
        };
        let mut file_path2: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path2.push("tests/resource/override.wad");

        let file2: File = File::open(&file_path2).unwrap();

        let doom_file2: DoomFile = DoomFile {
            file_handle: ManuallyDrop::new(file2),
            lumps: vec![Lump {
                name: String::from("DATA\0\0\0\0"),
                file_path: file_path2,
                file_position: 12,
                size: 14,
                should_reload: false,
            }],
        };

        let mut doom_files: Vec<DoomFile> = vec![doom_file1, doom_file2];

        let lump_data: Vec<u8> = get_lump_data(&mut doom_files, "DATA");

        let overrided_data: String = String::from_utf8(lump_data).unwrap();
        assert_eq!(overrided_data, "Override Data!");
    }
}
