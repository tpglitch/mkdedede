//! Mario Kart: Double Dash!! password decoder.
//!
//! Decodes the 16-character ghost data passwords the game generates after a
//! time trial. Each password encodes a course, kart, two drivers, a total
//! race time, and a best lap time, all protected by a checksum.
//!
//! # Example
//! ```
//! use mkdedede::double_dash::decode;
//!
//! match decode("SOME16CHARPASSWD") {
//!     Ok(ghost) => println!("{}", ghost),
//!     Err(e)    => eprintln!("invalid password: {}", e),
//! }
//! ```

use std::fmt;

// Shuffled base-32 alphabet used by the game
const CHAR_TABLE: &[u8] = b"G6EQTXYN4WRHBFKOIJAPCD5S8V7UZ3LM";

/// A race course in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Course {
    LuigiCircuit,
    PeachBeach,
    BabyPark,
    DryDryDesert,
    MushroomBridge,
    MarioCircuit,
    DaisyCruiser,
    WaluigiStadium,
    SherbetLand,
    MushroomCity,
    YoshiCircuit,
    DkMountain,
    WarioColosseum,
    DinoDinoJungle,
    BowsersCastle,
    RainbowRoad,
}

impl Course {
    /// Returns the display name of the course.
    pub fn name(self) -> &'static str {
        match self {
            Course::LuigiCircuit => "Luigi Circuit",
            Course::PeachBeach => "Peach Beach",
            Course::BabyPark => "Baby Park",
            Course::DryDryDesert => "Dry Dry Desert",
            Course::MushroomBridge => "Mushroom Bridge",
            Course::MarioCircuit => "Mario Circuit",
            Course::DaisyCruiser => "Daisy Cruiser",
            Course::WaluigiStadium => "Waluigi Stadium",
            Course::SherbetLand => "Sherbet Land",
            Course::MushroomCity => "Mushroom City",
            Course::YoshiCircuit => "Yoshi Circuit",
            Course::DkMountain => "DK Mountain",
            Course::WarioColosseum => "Wario Colosseum",
            Course::DinoDinoJungle => "Dino Dino Jungle",
            Course::BowsersCastle => "Bowser's Castle",
            Course::RainbowRoad => "Rainbow Road",
        }
    }

    /// Returns the total number of laps for this course.
    ///
    /// Most courses are 3 laps, Baby Park is 7, and Wario Colosseum is 2.
    pub fn laps(self) -> u32 {
        match self {
            Course::BabyPark => 7,
            Course::WarioColosseum => 2,
            _ => 3,
        }
    }

    fn from_index(i: u8) -> Option<Self> {
        match i {
            0 => Some(Course::LuigiCircuit),
            1 => Some(Course::PeachBeach),
            2 => Some(Course::BabyPark),
            3 => Some(Course::DryDryDesert),
            4 => Some(Course::MushroomBridge),
            5 => Some(Course::MarioCircuit),
            6 => Some(Course::DaisyCruiser),
            7 => Some(Course::WaluigiStadium),
            8 => Some(Course::SherbetLand),
            9 => Some(Course::MushroomCity),
            10 => Some(Course::YoshiCircuit),
            11 => Some(Course::DkMountain),
            12 => Some(Course::WarioColosseum),
            13 => Some(Course::DinoDinoJungle),
            14 => Some(Course::BowsersCastle),
            15 => Some(Course::RainbowRoad),
            _ => None,
        }
    }
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A playable character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Character {
    BabyMario,
    BabyLuigi,
    Paratroopa,
    Koopa,
    Peach,
    Daisy,
    Mario,
    Luigi,
    Wario,
    Waluigi,
    Yoshi,
    Birdo,
    DonkeyKong,
    DiddyKong,
    Bowser,
    BowserJr,
    Toad,
    Toadette,
    KingBoo,
    PeteyPiranha,
}

impl Character {
    /// Returns the display name of the character.
    pub fn name(self) -> &'static str {
        match self {
            Character::BabyMario => "Baby Mario",
            Character::BabyLuigi => "Baby Luigi",
            Character::Paratroopa => "Paratroopa",
            Character::Koopa => "Koopa",
            Character::Peach => "Peach",
            Character::Daisy => "Daisy",
            Character::Mario => "Mario",
            Character::Luigi => "Luigi",
            Character::Wario => "Wario",
            Character::Waluigi => "Waluigi",
            Character::Yoshi => "Yoshi",
            Character::Birdo => "Birdo",
            Character::DonkeyKong => "Donkey Kong",
            Character::DiddyKong => "Diddy Kong",
            Character::Bowser => "Bowser",
            Character::BowserJr => "Bowser Jr.",
            Character::Toad => "Toad",
            Character::Toadette => "Toadette",
            Character::KingBoo => "King Boo",
            Character::PeteyPiranha => "Petey Piranha",
        }
    }

    // Character indices in the password are 1-based, subtract 1 before calling this
    fn from_index(i: u8) -> Option<Self> {
        match i {
            0 => Some(Character::BabyMario),
            1 => Some(Character::BabyLuigi),
            2 => Some(Character::Paratroopa),
            3 => Some(Character::Koopa),
            4 => Some(Character::Peach),
            5 => Some(Character::Daisy),
            6 => Some(Character::Mario),
            7 => Some(Character::Luigi),
            8 => Some(Character::Wario),
            9 => Some(Character::Waluigi),
            10 => Some(Character::Yoshi),
            11 => Some(Character::Birdo),
            12 => Some(Character::DonkeyKong),
            13 => Some(Character::DiddyKong),
            14 => Some(Character::Bowser),
            15 => Some(Character::BowserJr),
            16 => Some(Character::Toad),
            17 => Some(Character::Toadette),
            18 => Some(Character::KingBoo),
            19 => Some(Character::PeteyPiranha),
            _ => None,
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A kart
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kart {
    RedFire,
    DkJumbo,
    TurboYoshi,
    KoopaDasher,
    HeartCoach,
    GooGooBuggy,
    WarioCar,
    KoopaKing,
    GreenFire,
    BarrelTrain,
    TurboBirdo,
    ParaWing,
    BloomCoach,
    RattleBuggy,
    WaluigiRacer,
    BulletBlaster,
    ToadKart,
    ToadetteKart,
    BooPipes,
    PiranhaPipes,
    ParadeKart,
}

impl Kart {
    /// Returns the display name of the kart.
    pub fn name(self) -> &'static str {
        match self {
            Kart::RedFire => "Red Fire",
            Kart::DkJumbo => "DK Jumbo",
            Kart::TurboYoshi => "Turbo Yoshi",
            Kart::KoopaDasher => "Koopa Dasher",
            Kart::HeartCoach => "Heart Coach",
            Kart::GooGooBuggy => "Goo-Goo Buggy",
            Kart::WarioCar => "Wario Car",
            Kart::KoopaKing => "Koopa King",
            Kart::GreenFire => "Green Fire",
            Kart::BarrelTrain => "Barrel Train",
            Kart::TurboBirdo => "Turbo Birdo",
            Kart::ParaWing => "Para-Wing",
            Kart::BloomCoach => "Bloom Coach",
            Kart::RattleBuggy => "Rattle Buggy",
            Kart::WaluigiRacer => "Waluigi Racer",
            Kart::BulletBlaster => "Bullet Blaster",
            Kart::ToadKart => "Toad Kart",
            Kart::ToadetteKart => "Toadette Kart",
            Kart::BooPipes => "Boo Pipes",
            Kart::PiranhaPipes => "Piranha Pipes",
            Kart::ParadeKart => "Parade Kart",
        }
    }

    fn from_index(i: u8) -> Option<Self> {
        match i {
            0 => Some(Kart::RedFire),
            1 => Some(Kart::DkJumbo),
            2 => Some(Kart::TurboYoshi),
            3 => Some(Kart::KoopaDasher),
            4 => Some(Kart::HeartCoach),
            5 => Some(Kart::GooGooBuggy),
            6 => Some(Kart::WarioCar),
            7 => Some(Kart::KoopaKing),
            8 => Some(Kart::GreenFire),
            9 => Some(Kart::BarrelTrain),
            10 => Some(Kart::TurboBirdo),
            11 => Some(Kart::ParaWing),
            12 => Some(Kart::BloomCoach),
            13 => Some(Kart::RattleBuggy),
            14 => Some(Kart::WaluigiRacer),
            15 => Some(Kart::BulletBlaster),
            16 => Some(Kart::ToadKart),
            17 => Some(Kart::ToadetteKart),
            18 => Some(Kart::BooPipes),
            19 => Some(Kart::PiranhaPipes),
            20 => Some(Kart::ParadeKart),
            _ => None,
        }
    }
}

impl fmt::Display for Kart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A race time, stored as milliseconds
///
/// Displays as `MM : SS . mmm`.
// Time stored as milliseconds
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LapTime(pub u32);

impl LapTime {
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

impl fmt::Display for LapTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02} : {:02} . {:03}",
            self.minutes(),
            self.seconds(),
            self.milliseconds()
        )
    }
}

/// Errors returned by [`decode`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Password was not exactly 16 characters after stripping invalid input
    WrongLength,
    /// A character was encountered that does not belong to the game's alphabet
    InvalidCharacter(char),
    /// The embedded checksum did not match the computed one
    InvalidChecksum,
    /// The course index was out of range
    InvalidCourse,
    /// The kart index was out of range
    InvalidKart,
    /// Driver 1's index was out of range
    InvalidDriver1,
    /// Driver 2's index was out of range
    InvalidDriver2,
    /// Both drivers decoded to the same character, which the game never produces
    SameDrivers,
    /// Best lap time multiplied by lap count exceeds total time
    InvalidTimes,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::WrongLength => write!(f, "Password must be exactly 16 characters"),
            DecodeError::InvalidCharacter(c) => write!(f, "Invalid character in password: '{}'", c),
            DecodeError::InvalidChecksum => write!(f, "Invalid checksum"),
            DecodeError::InvalidCourse => write!(f, "Invalid course"),
            DecodeError::InvalidKart => write!(f, "Invalid kart"),
            DecodeError::InvalidDriver1 => write!(f, "Invalid driver 1"),
            DecodeError::InvalidDriver2 => write!(f, "Invalid driver 2"),
            DecodeError::SameDrivers => write!(f, "Driver 1 and driver 2 cannot be the same"),
            DecodeError::InvalidTimes => write!(f, "Best lap time is inconsistent with total time"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Decoded ghost data from a password
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GhostData {
    pub course: Course,
    pub kart: Kart,
    pub driver1: Character,
    pub driver2: Character,
    pub total_time: LapTime,
    pub best_lap: LapTime,
}

impl fmt::Display for GhostData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Course:     {}", self.course)?;
        writeln!(f, "Kart:       {}", self.kart)?;
        writeln!(f, "Driver 1:   {}", self.driver1)?;
        writeln!(f, "Driver 2:   {}", self.driver2)?;
        writeln!(f, "Total Time: {}", self.total_time)?;
        write!(f, "Best Lap:   {}", self.best_lap)
    }
}

/// Decodes a 16-character MKDD ghost data password.
///
/// Input is case-insensitive. Characters outside the game's alphabet are
/// stripped before length validation.
///
/// Returns a [`GhostData`] on success, or a [`DecodeError`] describing why
/// the password was rejected.
pub fn decode(password: &str) -> Result<GhostData, DecodeError> {
    // Normalize uppercase and strip anything outside the valid alphabet
    let password: String = password
        .to_ascii_uppercase()
        .chars()
        .filter(|c| CHAR_TABLE.contains(&(*c as u8)))
        .collect();

    if password.len() != 16 {
        return Err(DecodeError::WrongLength);
    }

    // Convert each character to its 5-bit index in CHAR_TABLE
    let mut buf = [0u8; 16];
    for (i, ch) in password.chars().enumerate() {
        match CHAR_TABLE.iter().position(|&b| b == ch as u8) {
            Some(pos) => buf[i] = pos as u8,
            None => return Err(DecodeError::InvalidCharacter(ch)),
        }
    }

    // Pack 16x5-bit symbols into 10 bytes
    let mut d = [0u8; 10];
    d[0] = ((buf[0] << 3) | (buf[1] >> 2)) & 0xFF;
    d[1] = ((buf[1] << 6) | (buf[2] << 1) | (buf[3] >> 4)) & 0xFF;
    d[2] = ((buf[3] << 4) | (buf[4] >> 1)) & 0xFF;
    d[3] = ((buf[4] << 7) | (buf[5] << 2).wrapping_add(buf[6] >> 3)) & 0xFF;
    d[4] = ((buf[6] << 5) | buf[7]) & 0xFF;
    d[5] = ((buf[8] << 3) | (buf[9] >> 2)) & 0xFF;
    d[6] = ((buf[9] << 6) | (buf[10] << 1) | (buf[11] >> 4)) & 0xFF;
    d[7] = ((buf[11] << 4) | (buf[12] >> 1)) & 0xFF;
    d[8] = ((buf[12] << 7) | (buf[13] << 2).wrapping_add(buf[14] >> 3)) & 0xFF;
    d[9] = ((buf[14] << 5) | buf[15]) & 0xFF;

    let mut x: i32 = ((d[8] as i32) << 8) | (d[9] as i32);
    for byte in d[0..8].iter_mut() {
        *byte ^= (x >> 8) as u8;
        let y = (67_983_421i64 * (x as i64) + 1) as i32;
        let z = (y / 100_000_000) * 100_000_000;
        x = y - z;
    }

    let kart_idx = (d[6] >> 1) & 0x1F;
    let driver1_idx = (((d[7] >> 4) as u16) | ((d[6] as u16) << 4)) as u8 & 0x1F;
    let driver2_idx = (((d[2] as u16) << 3) | ((d[3] as u16) >> 5)) as u8 & 0x1F;
    let course_idx = d[7] & 0xF;

    let total_ms = ((d[0] as u32) << 11) | ((d[1] as u32) << 3) | ((d[2] as u32) >> 5);
    let best_ms = ((d[4] as u32) << 10) | ((d[5] as u32) << 2) | ((d[6] as u32) >> 6);

    let embedded_checksum = d[3] & 0x1F;
    let tmp = ((d[2] >> 2) & 0x3) as u32
        + ((d[2] >> 4) & 0x1) as u32
        + course_idx as u32
        + kart_idx as u32
        + driver1_idx as u32
        + driver2_idx as u32
        + total_ms
        + best_ms;

    let computed_checksum =
        ((tmp & 0xFF) + ((tmp >> 8) & 0xFF) + ((tmp >> 16) & 0xFF) + ((tmp >> 24) & 0xFF)) as u8
            & 0x1F;

    if embedded_checksum != computed_checksum {
        return Err(DecodeError::InvalidChecksum);
    }

    let course = Course::from_index(course_idx).ok_or(DecodeError::InvalidCourse)?;
    let kart = Kart::from_index(kart_idx).ok_or(DecodeError::InvalidKart)?;

    let d1_idx = driver1_idx
        .checked_sub(1)
        .ok_or(DecodeError::InvalidDriver1)?;
    let driver1 = Character::from_index(d1_idx).ok_or(DecodeError::InvalidDriver1)?;

    let d2_idx = driver2_idx
        .checked_sub(1)
        .ok_or(DecodeError::InvalidDriver2)?;
    let driver2 = Character::from_index(d2_idx).ok_or(DecodeError::InvalidDriver2)?;

    if driver1 == driver2 {
        return Err(DecodeError::SameDrivers);
    }

    if best_ms * course.laps() > total_ms {
        return Err(DecodeError::InvalidTimes);
    }

    Ok(GhostData {
        course,
        kart,
        driver1,
        driver2,
        total_time: LapTime(total_ms),
        best_lap: LapTime(best_ms),
    })
}
