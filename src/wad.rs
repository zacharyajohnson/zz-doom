pub struct DoomWad {
    name: &'static str,
}

pub struct DoomWads {
    pub wads: Vec<DoomWad>,
}

impl Default for DoomWads {
    fn default() -> Self {
        DoomWads { wads: 
            vec![
                DoomWad {
                    name: "doom1.wad"
                },
                DoomWad {
                    name: "doom.wad"
                },
                DoomWad {
                    name: "doomu.wad"
                },
                DoomWad {
                    name: "doom2.wad"
                },
                DoomWad {
                    name: "doom2f.wad"
                },
                DoomWad {
                    name: "tnt.wad"
                },
                DoomWad {
                    name: "plutonia.wad"
                },
            ]
         }
    }
}