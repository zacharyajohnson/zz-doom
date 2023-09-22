use std::{
    fs::File,
    io::{Seek, Write},
    path::PathBuf,
};

struct LumpInfo {
    pub name: String,
    pub size: u32,
    pub file_position: u32,
    pub data: String,
}

pub fn create_wad(mut file_path: PathBuf) {
    let mut lump_infos: Vec<LumpInfo> = Vec::new();

    println!("Enter wad file name: ");
    let mut wad_file_name: String = String::new();

    std::io::stdin().read_line(&mut wad_file_name).unwrap();
    wad_file_name = wad_file_name.trim().to_owned();

    file_path.push(wad_file_name);

    println!("Enter wad type(IWAD/PWAD)");

    let mut wad_type: String = String::new();
    std::io::stdin().read_line(&mut wad_type).unwrap();
    wad_type = wad_type.trim().to_owned();

    println!("Enter number of lumps in wad");
    let mut num_lumps: String = String::new();
    std::io::stdin().read_line(&mut num_lumps).unwrap();

    let num_lumps: u32 = num_lumps.trim().parse::<u32>().unwrap();

    let mut wad_file: File = File::create(file_path).unwrap();

    let mut info_table_offset: u32 = 0;

    wad_file.write_all(wad_type.as_bytes()).unwrap();
    wad_file.write_all(&num_lumps.to_le_bytes()).unwrap();

    for i in 0..num_lumps {
        let lump_x: u32 = i + 1;

        println!("Enter name for lump {}", lump_x);
        let mut name: String = String::new();
        std::io::stdin().read_line(&mut name).unwrap();

        // TODO INTELIJ PROFILES
        name = name.trim().to_owned();

        for _ in name.len()..8 {
            name.push('\0');
        }
        println!("Enter data for lump {}", lump_x);
        let mut data: String = String::new();
        std::io::stdin().read_line(&mut data).unwrap();
        data = data.trim().to_owned();

        let lump_size: u32 = data.len().try_into().unwrap();
        let mut lump_position: u32 = wad_file.stream_position().unwrap().try_into().unwrap();
        lump_position += 4;

        println!("Lump Size: {}", lump_size);
        println!("Lump position: {}", lump_position);

        lump_infos.push(LumpInfo {
            data,
            name,
            size: lump_size,
            file_position: lump_position,
        });

        info_table_offset = lump_position + lump_size;
    }

    println!("Info table offset: {}", info_table_offset);
    wad_file
        .write_all(&info_table_offset.to_le_bytes())
        .unwrap();

    for lump_info in &lump_infos {
        wad_file.write_all(lump_info.data.as_bytes()).unwrap();
    }

    for lump_info in lump_infos {
        wad_file
            .write_all(&lump_info.file_position.to_le_bytes())
            .unwrap();
        wad_file.write_all(&lump_info.size.to_le_bytes()).unwrap();
        wad_file.write_all(lump_info.name.as_bytes()).unwrap();
    }
}
