// doomdef stuff

pub enum GameLanguage {
    English,
    French,
    German,
    Unknown,
}

pub enum GameMode {
    Shareware,  // DOOM 1 Shareware
    Registered, // DOOM 1 Registered Copy
    Retail,     // The Ultimate DOOM (Doom 1 with 1 extra mission/ 9 extra levels)
    Commerical, // DOOM II/ French Version / Final DOOM(The Plutonia Experiment and TNT: Evilution)
    Unknown,
}

pub enum GameState {
    Level,
    Intermission,
    Finale,
    Demoscreen,
}

pub enum GameDifficulty {
    Baby,
    Easy,
    Medium,
    Hard,
    Nightmare,
}

pub enum GameWeapon {
    Fist,
    Pistol,
    Shotgun,
    Chaingun,
    MissileLauncher,
    PlasmaRifle,
    BFG,
    Chainsaw,
    SuperShotgun,
}

pub enum GameAmmo {
    Clip,
    Shell,
    Cell,
    Missle,
    // TODO Could replace with Option to represent?
    NoAmmo,
}

pub enum GamePowerup {
    Invulnerability,
    Strength,
    Invisibility,
    IronFeet,
    AllMap,
    Infrared,
}

// Revist to see if we can combine this with GamePowerup
// 35 tics per second
//pub enum GamePowerupDuration {
//    InvulnerabilityDuration = 1050, //(30*TIC_RATE)
//    InvisibilityDuration = 2100,//(60*TIC_RATE)
//    InfraredDuration = 4200,//(120*TIC_RATE)
//    IronFeetDuration = 2100//(60 * TIC_RATE)
//}

pub enum GameKey {
    BlueCard,
    YellowCard,
    RedCard,
    BlueSkull,
    YellowSkull,
    RedSkull,
}

// TODO The original DOOM had key layout configs defind
// Not sure if I need them so I'm leaving them out
// They look to be default values used in default_t(m_misc.c)
pub const RANGE_CHECK: bool = true;
pub const USE_SOUND_SERVER: bool = true;
pub const SCREEN_BASE_WIDTH: u32 = 320;
pub const SCREEN_BASE_HEIGHT: u32 = 200;
pub const MAX_PLAYERS: u32 = 4;
pub const TIC_RATE: u32 = 35;
pub const MTF_AMBUSH: u32 = 8;

// dstrings stuff
// TODO save name should be configurable later
pub const SAVE_NAME: &str = "doomsav";

// Used as a prefix for the dev wad
pub const DEV_WAD_PREFIX: &str = "devdata";

// Used as a prefix for the dev map folder
pub const DEV_MAP_FOLDER_PREFIX: &str = "devmaps";

// Various messages dispalyed when you
// go to quit the game
// TODO Should be refactored into game specific configs
pub const GAME_QUIT_MESSAGES: [&str; 23] = [
    // Doom 1
    // QUITMSG - for some reason only this line is found in d_englsh.h
    "are you sure you want to\nquit this great game?",
    "please don't leave, there's more\ndemons to toast!",
    "let's beat it -- this is turning\ninto a bloodbath!",
    "i wouldn't leave if i were you.\ndos is much worse.",
    "you're trying to say you like dos\nbetter than me, right?",
    "don't leave yet -- there's a\ndemon around that corner!",
    "ya know, next time you come in here\ni'm gonna toast ya.",
    "go ahead and leave. see if i care.",
    // Doom II
    "you want to quit?\nthen, thou hast lost an eighth!",
    "don't go now, there's a \ndimensional shambler waiting\nat the dos prompt!",
    "get outta here and go back\nto your boring programs.",
    "if i were your boss, i'd \n deathmatch ya in a minute!",
    "look, bud. you leave now\nand you forfeit your body count!",
    "just leave. when you come\nback, i'll be waiting with a bat.",
    "you're lucky i don't smack\nyou for thinking about leaving.",
    // Final Doom
    "fuck you, pussy!\nget the fuck out!",
    "you quit and i'll jizz\nin your cystholes!",
    "if you leave, i'll make\nthe lord drink my jizz.",
    "hey, ron! can we say\n'fuck' in the game?",
    "i'd leave: this is just\nmore monsters and levels.\nwhat a load.",
    "suck it down, asshole!\nyou're a fucking wimp!",
    "don't quit now! we're \nstill spending your money!",
    // Internal debug. Different style, too.
    "THIS IS NO MESSAGE!\nPage intentionally left blank.",
];
