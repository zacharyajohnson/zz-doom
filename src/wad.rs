use std::{path::Path, fs::File, io::{Read, Seek, self, SeekFrom}, fmt::Error};

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

pub struct Lump {
    pub name: String,
    pub file_handle: String,
    pub file_position: u32,
    pub size: u32
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

pub fn process_wad_file(file_path: &Path) -> Result<(), io::Error> {

    if file_path.starts_with("~") {
        
    }

    let mut file: File = File::open(file_path)?;

    let wad_header: WadHeader = WadHeader::from(&mut file)?;

    file.seek(SeekFrom::Start(wad_header.info_table_ofs.into()))?;
    Ok(())
}

#[cfg(test)]
mod tests {

}