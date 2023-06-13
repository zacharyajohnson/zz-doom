use std::{path::{Path, PathBuf}, fs::File, io::{Read, Seek, self, SeekFrom}, fmt::Error};

pub mod iwad;

#[derive(Debug)]
pub enum WadID {
    IWAD,
    PWAD
}

impl WadID {
    fn from(value: impl AsRef<str>) -> Self{
    match value.as_ref() {
        "IWAD" => WadID::IWAD,
        "PWAD" => WadID::PWAD,
        _ => panic!("Invalid value for Wad ID: {}. Can only be IWAD/PWAD", value.as_ref())
    }

}
    fn to_str(&self) -> &str {
        match self {
            WadID::IWAD => "IWAD",
            WadID::PWAD => "PWAD"
        }
    }
}

pub enum ValidFileType {
    LMP,
    WAD
}

pub struct WadHeader {
    pub id: WadID,
    pub num_lumps: u32,
    pub info_table_ofs: u32,
}

impl WadHeader {
    fn from(file: &mut File) -> Result<Self, io::Error> {
        let mut id: [u8; 4] = [0; 4];
        let mut num_lumps: [u8; 4] = [0; 4];
        let mut info_table_ofs: [u8; 4] = [0; 4];

        file.read(&mut id)?;
        file.read(&mut num_lumps)?;
        file.read(&mut info_table_ofs)?;

        let id: WadID = WadID::from(String::from_utf8_lossy(&id).to_string());
        let num_lumps: u32 = u32::from_le_bytes(num_lumps);
        let info_table_ofs: u32 = u32::from_le_bytes(info_table_ofs);

        println!("{}", id.to_str());
        println!("{}", num_lumps);
        println!("{}", info_table_ofs);

        Ok(WadHeader { id, num_lumps, info_table_ofs })
    }
}

pub struct Lump {
    pub name: String,
    pub file_handle: String,
    pub file_position: u32,
    pub size: u32
}

pub struct Wad {
    pub name: String,
    pub file_path: PathBuf,
    pub header: WadHeader,
    pub lumps: Vec<Lump>,
    pub should_reload: bool
}

pub fn process_wad_file(file_path: &Path) -> Result<Vec<Lump>, io::Error> {
    let should_reload: bool = if file_path.starts_with("~") {
        true
    } else {
        false
    };

    let mut file: File = File::open(file_path)?;

    let wad_header: WadHeader = WadHeader::from(&mut file)?;

    file.seek(SeekFrom::Start(wad_header.info_table_ofs.into()))?;

    let mut lumps: Vec<Lump> = Vec::new();

    for i in 0..wad_header.num_lumps {
        let mut file_position: [u8; 4] = [0; 4];
        let mut size: [u8; 4] = [0; 4];
        let mut name: [u8; 8] = [0; 8];

        file.read(&mut file_position)?;
        file.read(&mut size)?;
        file.read(&mut name)?;

        let file_position: u32 = u32::from_le_bytes(file_position);
        let size: u32 = u32::from_le_bytes(size);
        let name: String = String::from_utf8_lossy(&name).to_string();

        let lump: Lump = Lump { name, file_handle: String::from(""), file_position, size};

        println!("Lump {} = file_pos: {}, size: {}, name: {}",i, lump.file_position, lump.size, lump.name);
        lumps.push(lump);

    }

    let wad: Wad = Wad { name, file_path, header, lumps, should_reload}
    Ok()

}

#[cfg(test)]
mod tests {

}