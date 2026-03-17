//! Mario Kart DS ghost data password decoder
//!
//! Decodes the 16-character ghost data passwords Mario Kart DS generates after
//! a time trial. Each password encodes a course, character, kart, a race time,
//! and a two-character UTF-16 player name, all protected by a CRC-16 checksum.
//!
//! # Example
//! ```
//! use mkdedede::ds::decode;
//!
//! match decode("SOME16CHARPASSWD") {
//!     Ok(ghost) => println!("{}", ghost),
//!     Err(e)    => eprintln!("invalid password: {}", e),
//! }
//! ```

use std::fmt;

/// Shuffled base-32 alphabet used by Mario Kart DS
const BASE32_TABLE: &[u8] = b"S7LCX3JZE8FG4HBKWN52YPA6RTU9VMDQ";

/// A race course in Mario Kart DS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Course {
    // Mushroom Cup
    Figure8Circuit,
    YoshiFalls,
    CheepCheepBeach,
    LuigisMansion,
    // Flower Cup
    DesertHills,
    DelfinoSquare,
    WaluigiPinball,
    ShroomRidge,
    // Star Cup
    DkPass,
    TickTockClock,
    MarioCircuit,
    AirshipFortress,
    // Special Cup
    WarioStadium,
    PeachGardens,
    BowserCastle,
    RainbowRoad,
    // Shell Cup (retro)
    SnesMarioCircuit1,
    N64MooMooFarm,
    GbaPeachCircuit,
    GcnLuigiCircuit,
    // Banana Cup (retro)
    SnesDonutPlains1,
    N64FrappeSnowland,
    GbaBowserCastle2,
    GcnBabyPark,
    // Leaf Cup (retro)
    SnesKoopaBeach2,
    N64ChocoMountain,
    GbaLuigiCircuit,
    GcnMushroomBridge,
    // Lightning Cup (retro)
    SnesChocoIsland2,
    N64BansheeBoardwalk,
    GbaSkyGarden,
    GcnYoshiCircuit,
}

impl Course {
    /// Returns the display name of the course.
    pub fn name(self) -> &'static str {
        match self {
            Course::Figure8Circuit => "Figure-8 Circuit",
            Course::YoshiFalls => "Yoshi Falls",
            Course::CheepCheepBeach => "Cheep Cheep Beach",
            Course::LuigisMansion => "Luigi's Mansion",
            Course::DesertHills => "Desert Hills",
            Course::DelfinoSquare => "Delfino Square",
            Course::WaluigiPinball => "Waluigi Pinball",
            Course::ShroomRidge => "Shroom Ridge",
            Course::DkPass => "DK Pass",
            Course::TickTockClock => "Tick-Tock Clock",
            Course::MarioCircuit => "Mario Circuit",
            Course::AirshipFortress => "Airship Fortress",
            Course::WarioStadium => "Wario Stadium",
            Course::PeachGardens => "Peach Gardens",
            Course::BowserCastle => "Bowser Castle",
            Course::RainbowRoad => "Rainbow Road",
            Course::SnesMarioCircuit1 => "SNES Mario Circuit 1",
            Course::N64MooMooFarm => "N64 Moo Moo Farm",
            Course::GbaPeachCircuit => "GBA Peach Circuit",
            Course::GcnLuigiCircuit => "GCN Luigi Circuit",
            Course::SnesDonutPlains1 => "SNES Donut Plains 1",
            Course::N64FrappeSnowland => "N64 Frappe Snowland",
            Course::GbaBowserCastle2 => "GBA Bowser Castle 2",
            Course::GcnBabyPark => "GCN Baby Park",
            Course::SnesKoopaBeach2 => "SNES Koopa Beach 2",
            Course::N64ChocoMountain => "N64 Choco Mountain",
            Course::GbaLuigiCircuit => "GBA Luigi Circuit",
            Course::GcnMushroomBridge => "GCN Mushroom Bridge",
            Course::SnesChocoIsland2 => "SNES Choco Island 2",
            Course::N64BansheeBoardwalk => "N64 Banshee Boardwalk",
            Course::GbaSkyGarden => "GBA Sky Garden",
            Course::GcnYoshiCircuit => "GCN Yoshi Circuit",
        }
    }

    fn from_index(i: u8) -> Option<Self> {
        match i {
            0 => Some(Course::Figure8Circuit),
            1 => Some(Course::YoshiFalls),
            2 => Some(Course::CheepCheepBeach),
            3 => Some(Course::LuigisMansion),
            4 => Some(Course::DesertHills),
            5 => Some(Course::DelfinoSquare),
            6 => Some(Course::WaluigiPinball),
            7 => Some(Course::ShroomRidge),
            8 => Some(Course::DkPass),
            9 => Some(Course::TickTockClock),
            10 => Some(Course::MarioCircuit),
            11 => Some(Course::AirshipFortress),
            12 => Some(Course::WarioStadium),
            13 => Some(Course::PeachGardens),
            14 => Some(Course::BowserCastle),
            15 => Some(Course::RainbowRoad),
            16 => Some(Course::SnesMarioCircuit1),
            17 => Some(Course::N64MooMooFarm),
            18 => Some(Course::GbaPeachCircuit),
            19 => Some(Course::GcnLuigiCircuit),
            20 => Some(Course::SnesDonutPlains1),
            21 => Some(Course::N64FrappeSnowland),
            22 => Some(Course::GbaBowserCastle2),
            23 => Some(Course::GcnBabyPark),
            24 => Some(Course::SnesKoopaBeach2),
            25 => Some(Course::N64ChocoMountain),
            26 => Some(Course::GbaLuigiCircuit),
            27 => Some(Course::GcnMushroomBridge),
            28 => Some(Course::SnesChocoIsland2),
            29 => Some(Course::N64BansheeBoardwalk),
            30 => Some(Course::GbaSkyGarden),
            31 => Some(Course::GcnYoshiCircuit),
            _ => None,
        }
    }

    pub(crate) fn to_index(self) -> u8 {
        match self {
            Course::Figure8Circuit => 0,
            Course::YoshiFalls => 1,
            Course::CheepCheepBeach => 2,
            Course::LuigisMansion => 3,
            Course::DesertHills => 4,
            Course::DelfinoSquare => 5,
            Course::WaluigiPinball => 6,
            Course::ShroomRidge => 7,
            Course::DkPass => 8,
            Course::TickTockClock => 9,
            Course::MarioCircuit => 10,
            Course::AirshipFortress => 11,
            Course::WarioStadium => 12,
            Course::PeachGardens => 13,
            Course::BowserCastle => 14,
            Course::RainbowRoad => 15,
            Course::SnesMarioCircuit1 => 16,
            Course::N64MooMooFarm => 17,
            Course::GbaPeachCircuit => 18,
            Course::GcnLuigiCircuit => 19,
            Course::SnesDonutPlains1 => 20,
            Course::N64FrappeSnowland => 21,
            Course::GbaBowserCastle2 => 22,
            Course::GcnBabyPark => 23,
            Course::SnesKoopaBeach2 => 24,
            Course::N64ChocoMountain => 25,
            Course::GbaLuigiCircuit => 26,
            Course::GcnMushroomBridge => 27,
            Course::SnesChocoIsland2 => 28,
            Course::N64BansheeBoardwalk => 29,
            Course::GbaSkyGarden => 30,
            Course::GcnYoshiCircuit => 31,
        }
    }
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A playable character in Mario Kart DS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Character {
    Mario,
    DonkeyKong,
    Toad,
    Bowser,
    Peach,
    Wario,
    Yoshi,
    Luigi,
    DryBones,
    Daisy,
    Waluigi,
    Rob,
    ShyGuy,
}

impl Character {
    /// Returns the display name of the character.
    pub fn name(self) -> &'static str {
        match self {
            Character::Mario => "Mario",
            Character::DonkeyKong => "Donkey Kong",
            Character::Toad => "Toad",
            Character::Bowser => "Bowser",
            Character::Peach => "Peach",
            Character::Wario => "Wario",
            Character::Yoshi => "Yoshi",
            Character::Luigi => "Luigi",
            Character::DryBones => "Dry Bones",
            Character::Daisy => "Daisy",
            Character::Waluigi => "Waluigi",
            Character::Rob => "R.O.B.",
            Character::ShyGuy => "Shy Guy",
        }
    }

    fn from_index(i: u8) -> Option<Self> {
        match i {
            0 => Some(Character::Mario),
            1 => Some(Character::DonkeyKong),
            2 => Some(Character::Toad),
            3 => Some(Character::Bowser),
            4 => Some(Character::Peach),
            5 => Some(Character::Wario),
            6 => Some(Character::Yoshi),
            7 => Some(Character::Luigi),
            8 => Some(Character::DryBones),
            9 => Some(Character::Daisy),
            10 => Some(Character::Waluigi),
            11 => Some(Character::Rob),
            12 => Some(Character::ShyGuy),
            _ => None,
        }
    }

    pub(crate) fn to_index(self) -> u8 {
        match self {
            Character::Mario => 0,
            Character::DonkeyKong => 1,
            Character::Toad => 2,
            Character::Bowser => 3,
            Character::Peach => 4,
            Character::Wario => 5,
            Character::Yoshi => 6,
            Character::Luigi => 7,
            Character::DryBones => 8,
            Character::Daisy => 9,
            Character::Waluigi => 10,
            Character::Rob => 11,
            Character::ShyGuy => 12,
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A kart in Mario Kart DS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kart {
    StandardMr,
    ShootingStar,
    BDasher,
    StandardDk,
    Wildlife,
    RambiRider,
    StandardTd,
    Mushmellow,
    FourWheelCradle,
    StandardBw,
    Hurricane,
    Tyrant,
    StandardPc,
    LightTripper,
    Royale,
    StandardWr,
    Brute,
    Dragonfly,
    StandardYs,
    Egg1,
    Cucumber,
    StandardLg,
    Poltergust4000,
    Streamliner,
    StandardDb,
    DryBomber,
    Banisher,
    StandardDs,
    LightDancer,
    PowerFlower,
    StandardWl,
    GoldMantis,
    Zipper,
    StandardRb,
    RobBls,
    RobLgs,
    StandardSg,
}

impl Kart {
    /// Returns the display name of the kart.
    pub fn name(self) -> &'static str {
        match self {
            Kart::StandardMr => "Standard MR",
            Kart::ShootingStar => "Shooting Star",
            Kart::BDasher => "B Dasher",
            Kart::StandardDk => "Standard DK",
            Kart::Wildlife => "Wildlife",
            Kart::RambiRider => "Rambi Rider",
            Kart::StandardTd => "Standard TD",
            Kart::Mushmellow => "Mushmellow",
            Kart::FourWheelCradle => "4-Wheel Cradle",
            Kart::StandardBw => "Standard BW",
            Kart::Hurricane => "Hurricane",
            Kart::Tyrant => "Tyrant",
            Kart::StandardPc => "Standard PC",
            Kart::LightTripper => "Light Tripper",
            Kart::Royale => "Royale",
            Kart::StandardWr => "Standard WR",
            Kart::Brute => "Brute",
            Kart::Dragonfly => "Dragonfly",
            Kart::StandardYs => "Standard YS",
            Kart::Egg1 => "Egg 1",
            Kart::Cucumber => "Cucumber",
            Kart::StandardLg => "Standard LG",
            Kart::Poltergust4000 => "Poltergust 4000",
            Kart::Streamliner => "Streamliner",
            Kart::StandardDb => "Standard DB",
            Kart::DryBomber => "Dry Bomber",
            Kart::Banisher => "Banisher",
            Kart::StandardDs => "Standard DS",
            Kart::LightDancer => "Light Dancer",
            Kart::PowerFlower => "Power Flower",
            Kart::StandardWl => "Standard WL",
            Kart::GoldMantis => "Gold Mantis",
            Kart::Zipper => "Zipper",
            Kart::StandardRb => "Standard RB",
            Kart::RobBls => "ROB-BLS",
            Kart::RobLgs => "ROB-LGS",
            Kart::StandardSg => "Standard SG",
        }
    }

    fn from_index(i: u8) -> Option<Self> {
        match i {
            0 => Some(Kart::StandardMr),
            1 => Some(Kart::ShootingStar),
            2 => Some(Kart::BDasher),
            3 => Some(Kart::StandardDk),
            4 => Some(Kart::Wildlife),
            5 => Some(Kart::RambiRider),
            6 => Some(Kart::StandardTd),
            7 => Some(Kart::Mushmellow),
            8 => Some(Kart::FourWheelCradle),
            9 => Some(Kart::StandardBw),
            10 => Some(Kart::Hurricane),
            11 => Some(Kart::Tyrant),
            12 => Some(Kart::StandardPc),
            13 => Some(Kart::LightTripper),
            14 => Some(Kart::Royale),
            15 => Some(Kart::StandardWr),
            16 => Some(Kart::Brute),
            17 => Some(Kart::Dragonfly),
            18 => Some(Kart::StandardYs),
            19 => Some(Kart::Egg1),
            20 => Some(Kart::Cucumber),
            21 => Some(Kart::StandardLg),
            22 => Some(Kart::Poltergust4000),
            23 => Some(Kart::Streamliner),
            24 => Some(Kart::StandardDb),
            25 => Some(Kart::DryBomber),
            26 => Some(Kart::Banisher),
            27 => Some(Kart::StandardDs),
            28 => Some(Kart::LightDancer),
            29 => Some(Kart::PowerFlower),
            30 => Some(Kart::StandardWl),
            31 => Some(Kart::GoldMantis),
            32 => Some(Kart::Zipper),
            33 => Some(Kart::StandardRb),
            34 => Some(Kart::RobBls),
            35 => Some(Kart::RobLgs),
            36 => Some(Kart::StandardSg),
            _ => None,
        }
    }

    pub(crate) fn to_index(self) -> u8 {
        match self {
            Kart::StandardMr => 0,
            Kart::ShootingStar => 1,
            Kart::BDasher => 2,
            Kart::StandardDk => 3,
            Kart::Wildlife => 4,
            Kart::RambiRider => 5,
            Kart::StandardTd => 6,
            Kart::Mushmellow => 7,
            Kart::FourWheelCradle => 8,
            Kart::StandardBw => 9,
            Kart::Hurricane => 10,
            Kart::Tyrant => 11,
            Kart::StandardPc => 12,
            Kart::LightTripper => 13,
            Kart::Royale => 14,
            Kart::StandardWr => 15,
            Kart::Brute => 16,
            Kart::Dragonfly => 17,
            Kart::StandardYs => 18,
            Kart::Egg1 => 19,
            Kart::Cucumber => 20,
            Kart::StandardLg => 21,
            Kart::Poltergust4000 => 22,
            Kart::Streamliner => 23,
            Kart::StandardDb => 24,
            Kart::DryBomber => 25,
            Kart::Banisher => 26,
            Kart::StandardDs => 27,
            Kart::LightDancer => 28,
            Kart::PowerFlower => 29,
            Kart::StandardWl => 30,
            Kart::GoldMantis => 31,
            Kart::Zipper => 32,
            Kart::StandardRb => 33,
            Kart::RobBls => 34,
            Kart::RobLgs => 35,
            Kart::StandardSg => 36,
        }
    }
}

impl fmt::Display for Kart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A race time stored as milliseconds
///
/// Displays as `M:SS.mmm`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RaceTime(pub u32);

impl RaceTime {
    /// Returns the minutes component.
    pub fn minutes(self) -> u32 {
        self.0 / 60_000
    }

    /// Returns the seconds component (0-59).
    pub fn seconds(self) -> u32 {
        (self.0 / 1_000) % 60
    }

    /// Returns the milliseconds component (0-999).
    pub fn milliseconds(self) -> u32 {
        self.0 % 1_000
    }
}

impl fmt::Display for RaceTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{:02}.{:03}",
            self.minutes(),
            self.seconds(),
            self.milliseconds()
        )
    }
}

/// Errors returned by [`decode`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Password was not exactly 16 valid characters after filtering
    WrongLength,
    /// The CRC-16 checksum did not match
    InvalidChecksum,
    /// The course index was out of range (0-31)
    InvalidCourse,
    /// The character index was out of range (0-12)
    InvalidCharacter,
    /// The kart index was out of range (0-36)
    InvalidKart,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::WrongLength => write!(f, "Password must be exactly 16 valid characters"),
            DecodeError::InvalidChecksum => write!(f, "Invalid checksum"),
            DecodeError::InvalidCourse => write!(f, "Invalid course index"),
            DecodeError::InvalidCharacter => write!(f, "Invalid character index"),
            DecodeError::InvalidKart => write!(f, "Invalid kart index"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Decoded ghost data from a Mario Kart DS password
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GhostData {
    pub course: Course,
    pub character: Character,
    pub kart: Kart,
    pub time: RaceTime,
    /// The two UTF-16 characters that make up the player's name as stored in
    /// the password. Both are `'\0'` when the name is absent.
    pub player: [char; 2],
}

impl fmt::Display for GhostData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Course:    {}", self.course)?;
        writeln!(f, "Character: {}", self.character)?;
        writeln!(f, "Kart:      {}", self.kart)?;
        writeln!(f, "Time:      {}", self.time)?;
        let name: String = self.player.iter().filter(|&&c| c != '\0').collect();
        write!(f, "Player:    {}", name)
    }
}

/// Look up a character's 5-bit index in the DS base-32 table.
fn base32_index(c: u8) -> Option<u8> {
    BASE32_TABLE.iter().position(|&b| b == c).map(|p| p as u8)
}

/// Pack 16 five-bit symbols into 10 bytes (identical bit-layout to MKDD).
fn pack_symbols(syms: &[u8; 16]) -> [u8; 10] {
    let s = syms;
    let mut out = [0u8; 10];
    out[0] = ((s[0] as u16) << 3 | (s[1] as u16) >> 2) as u8;
    out[1] = ((s[1] as u16) << 6 | (s[2] as u16) << 1 | (s[3] as u16) >> 4) as u8;
    out[2] = ((s[3] as u16) << 4 | (s[4] as u16) >> 1) as u8;
    out[3] = ((s[4] as u16) << 7 | (s[5] as u16) << 2 | (s[6] as u16) >> 3) as u8;
    out[4] = ((s[6] as u16) << 5 | s[7] as u16) as u8;
    out[5] = ((s[8] as u16) << 3 | (s[9] as u16) >> 2) as u8;
    out[6] = ((s[9] as u16) << 6 | (s[10] as u16) << 1 | (s[11] as u16) >> 4) as u8;
    out[7] = ((s[11] as u16) << 4 | (s[12] as u16) >> 1) as u8;
    out[8] = ((s[12] as u16) << 7 | (s[13] as u16) << 2 | (s[14] as u16) >> 3) as u8;
    out[9] = ((s[14] as u16) << 5 | s[15] as u16) as u8;
    out
}

/// XOR-chain decryption used by MKDS.
///
/// `code[i] ^= code[i+1]` for i in 0..8, then `code[9] ^= 0xC3`.
fn decrypt(code: &mut [u8; 10]) {
    for i in 0..9 {
        code[i] ^= code[i + 1];
    }
    code[9] ^= 0xC3;
}

/// XOR-chain encryption used by MKDS (inverse of [`decrypt`]).
///
/// Reverses the decryption steps in the opposite order.
fn encrypt(code: &mut [u8; 10]) {
    code[9] ^= 0xC3;
    for i in (0..9).rev() {
        code[i] ^= code[i + 1];
    }
}

/// Unpack 10 bytes back into 16 five-bit symbols (inverse of [`pack_symbols`]).
fn unpack_symbols(bytes: &[u8; 10]) -> [u8; 16] {
    let b = bytes;
    let mut s = [0u8; 16];
    s[0] = b[0] >> 3;
    s[1] = ((b[0] & 0x07) << 2) | (b[1] >> 6);
    s[2] = (b[1] >> 1) & 0x1F;
    s[3] = ((b[1] & 0x01) << 4) | (b[2] >> 4);
    s[4] = ((b[2] & 0x0F) << 1) | (b[3] >> 7);
    s[5] = (b[3] >> 2) & 0x1F;
    s[6] = ((b[3] & 0x03) << 3) | (b[4] >> 5);
    s[7] = b[4] & 0x1F;
    s[8] = b[5] >> 3;
    s[9] = ((b[5] & 0x07) << 2) | (b[6] >> 6);
    s[10] = (b[6] >> 1) & 0x1F;
    s[11] = ((b[6] & 0x01) << 4) | (b[7] >> 4);
    s[12] = ((b[7] & 0x0F) << 1) | (b[8] >> 7);
    s[13] = (b[8] >> 2) & 0x1F;
    s[14] = ((b[8] & 0x03) << 3) | (b[9] >> 5);
    s[15] = b[9] & 0x1F;
    s
}

/// Standard CRC-16/CCITT (polynomial 0x1021, initial value 0).
fn crc16(data: &[u8]) -> u16 {
    let mut sum: u16 = 0;
    for &byte in data.iter() {
        let mut ch = byte;
        for _ in 0..8 {
            if sum & 0x8000 != 0 {
                sum = (sum << 1) ^ 0x1021;
            } else {
                sum <<= 1;
            }
            if ch & 0x80 != 0 {
                sum ^= 1;
            }
            ch <<= 1;
        }
    }
    sum
}

/// Encodes a [`GhostData`] into a 16-character Mario Kart DS password.
///
/// The returned string uses only uppercase letters from the game's base-32
/// alphabet (`S7LCX3JZE8FG4HBKWN52YPA6RTU9VMDQ`) and is always exactly 16
/// characters long. Passing the result to [`decode`] will recover the original
/// [`GhostData`].
pub fn encode(data: &GhostData) -> String {
    let char_idx = data.character.to_index() as u32;
    let kart_idx = data.kart.to_index() as u32;
    let course_idx = data.course.to_index() as u32;

    let kartch = char_idx * 37 + kart_idx;
    let stats = (data.time.0 << 14) | (course_idx << 9) | kartch;

    let mut code = [0u8; 10];
    let stats_bytes = stats.to_le_bytes();
    code[0] = stats_bytes[0];
    code[1] = stats_bytes[1];
    code[2] = stats_bytes[2];
    code[3] = stats_bytes[3];

    let c1 = data.player[0] as u16;
    let c2 = data.player[1] as u16;
    code[4] = (c1 & 0xFF) as u8;
    code[5] = (c1 >> 8) as u8;
    code[6] = (c2 & 0xFF) as u8;
    code[7] = (c2 >> 8) as u8;
    // code[8] and code[9] remain 0 for CRC computation

    let crc = crc16(&code);
    code[8] = (crc >> 8) as u8;
    code[9] = (crc & 0xFF) as u8;

    encrypt(&mut code);

    let syms = unpack_symbols(&code);
    syms.iter()
        .map(|&s| BASE32_TABLE[s as usize] as char)
        .collect()
}

/// Decodes a 16-character Mario Kart DS ghost data password.
///
/// Input is case-insensitive. Characters outside the game's base-32 alphabet
/// (`S7LCX3JZE8FG4HBKWN52YPA6RTU9VMDQ`) are silently ignored before length
/// validation.
///
/// Returns a [`GhostData`] on success, or a [`DecodeError`] describing why the
/// password was rejected.
pub fn decode(password: &str) -> Result<GhostData, DecodeError> {
    // Filter to valid alphabet characters (case-insensitive)
    let mut syms = [0u8; 16];
    let mut count = 0usize;

    for ch in password.chars() {
        if count == 16 {
            break;
        }
        let upper = ch.to_ascii_uppercase() as u8;
        if let Some(idx) = base32_index(upper) {
            syms[count] = idx;
            count += 1;
        }
    }

    if count != 16 {
        return Err(DecodeError::WrongLength);
    }

    let mut code = pack_symbols(&syms);
    decrypt(&mut code);

    // Bytes 8-9 hold the big-endian CRC-16; zero them before recomputing
    let stored_crc = (code[8] as u16) << 8 | code[9] as u16;
    code[8] = 0;
    code[9] = 0;

    let computed_crc = crc16(&code);
    if stored_crc != computed_crc {
        return Err(DecodeError::InvalidChecksum);
    }

    // Parse the packed fields from the first 4 bytes (little-endian u32)
    let stats = u32::from_le_bytes([code[0], code[1], code[2], code[3]]);
    let time_ms = stats >> 14;
    let course_idx = ((stats >> 9) % 32) as u8;
    let kartch = (stats % 0x200) as u16; // lower 9 bits
    let char_idx = (kartch / 37) as u8;
    let kart_idx = (kartch % 37) as u8;

    let course = Course::from_index(course_idx).ok_or(DecodeError::InvalidCourse)?;
    let character = Character::from_index(char_idx).ok_or(DecodeError::InvalidCharacter)?;
    let kart = Kart::from_index(kart_idx).ok_or(DecodeError::InvalidKart)?;

    // Player name: two little-endian UTF-16 code units stored at bytes 4-7
    let c1 = u16::from_le_bytes([code[4], code[5]]);
    let c2 = u16::from_le_bytes([code[6], code[7]]);
    let player = [
        char::from_u32(c1 as u32).unwrap_or('\0'),
        char::from_u32(c2 as u32).unwrap_or('\0'),
    ];

    Ok(GhostData {
        course,
        character,
        kart,
        time: RaceTime(time_ms),
        player,
    })
}
