use std::path::Path;

pub mod iwad;

pub enum WadID {
    IWAD,
    PWAD
}

impl WadID {
    pub fn to_str(&self) -> &str {
        match self {
            IWAD => "IWAD",
            PWAD => "PWAD"
        }
    }
}

pub enum ValidFileType {
    LMP,
    WAD
}

pub struct WadHeader {
    pub id: WadID,
    pub num_lumps: i32,
    pub info_table_ofs: i32,
}

pub fn process_wad_file(file_path: &Path) {
    if file_path.starts_with("~") {
        
    }
}

#[cfg(test)]
mod tests {

}