use std::ffi::{CStr, CString, c_char};
use std::ptr;

use crate::double_dash;
use crate::ds;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MkddGhostDataC {
    pub course: u8,
    pub kart: u8,
    pub driver1: u8,
    pub driver2: u8,
    pub total_time_ms: u32,
    pub best_lap_ms: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DsGhostDataC {
    pub course: u8,
    pub character: u8,
    pub kart: u8,
    pub time_ms: u32,
    pub player_name: [u16; 2],
}

#[repr(C)]
pub enum MkdededeDecodeStatus {
    Success = 0,
    WrongLength = 1,
    InvalidCharacter = 2,
    InvalidChecksum = 3,
    InvalidCourse = 4,
    InvalidKart = 5,
    InvalidDriver = 6,
    NullPointer = 7,
    Utf8Error = 8,
    SameDrivers = 9,
    InvalidTimes = 10,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mkdedede_mkdd_decode(
    password: *const c_char,
    out_data: *mut MkddGhostDataC,
) -> MkdededeDecodeStatus {
    if password.is_null() || out_data.is_null() {
        return MkdededeDecodeStatus::NullPointer;
    }

    let c_str = unsafe { CStr::from_ptr(password) };
    let str_slice = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return MkdededeDecodeStatus::Utf8Error,
    };

    match double_dash::decode(str_slice) {
        Ok(data) => {
            unsafe {
                *out_data = MkddGhostDataC {
                    course: data.course.to_index(),
                    kart: data.kart.to_index(),
                    driver1: data.driver1.to_index(),
                    driver2: data.driver2.to_index(),
                    total_time_ms: data.total_time.0,
                    best_lap_ms: data.best_lap.0,
                };
            }
            MkdededeDecodeStatus::Success
        }
        Err(e) => match e {
            double_dash::DecodeError::WrongLength => MkdededeDecodeStatus::WrongLength,
            double_dash::DecodeError::InvalidCharacter(_) => MkdededeDecodeStatus::InvalidCharacter,
            double_dash::DecodeError::InvalidChecksum => MkdededeDecodeStatus::InvalidChecksum,
            double_dash::DecodeError::InvalidCourse => MkdededeDecodeStatus::InvalidCourse,
            double_dash::DecodeError::InvalidKart => MkdededeDecodeStatus::InvalidKart,
            double_dash::DecodeError::InvalidDriver1 | double_dash::DecodeError::InvalidDriver2 => {
                MkdededeDecodeStatus::InvalidDriver
            }
            double_dash::DecodeError::SameDrivers => MkdededeDecodeStatus::SameDrivers,
            double_dash::DecodeError::InvalidTimes => MkdededeDecodeStatus::InvalidTimes,
        },
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mkdedede_ds_decode(
    password: *const c_char,
    out_data: *mut DsGhostDataC,
) -> MkdededeDecodeStatus {
    if password.is_null() || out_data.is_null() {
        return MkdededeDecodeStatus::NullPointer;
    }

    let c_str = unsafe { CStr::from_ptr(password) };
    let str_slice = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return MkdededeDecodeStatus::Utf8Error,
    };

    match ds::decode(str_slice) {
        Ok(data) => {
            unsafe {
                *out_data = DsGhostDataC {
                    course: data.course.to_index(),
                    character: data.character.to_index(),
                    kart: data.kart.to_index(),
                    time_ms: data.time.0,
                    player_name: [data.player[0] as u16, data.player[1] as u16],
                };
            }
            MkdededeDecodeStatus::Success
        }
        Err(e) => match e {
            ds::DecodeError::WrongLength => MkdededeDecodeStatus::WrongLength,
            ds::DecodeError::InvalidChecksum => MkdededeDecodeStatus::InvalidChecksum,
            ds::DecodeError::InvalidCourse => MkdededeDecodeStatus::InvalidCourse,
            ds::DecodeError::InvalidCharacter => MkdededeDecodeStatus::InvalidCharacter,
            ds::DecodeError::InvalidKart => MkdededeDecodeStatus::InvalidKart,
        },
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mkdedede_mkdd_encode(
    data: *const MkddGhostDataC,
    out_password: *mut c_char,
) -> MkdededeDecodeStatus {
    if data.is_null() || out_password.is_null() {
        return MkdededeDecodeStatus::NullPointer;
    }

    // SAFETY: We checked for null pointers
    unsafe {
        let data = &*data;

        let Some(course) = double_dash::Course::from_index(data.course) else {
            return MkdededeDecodeStatus::InvalidCourse;
        };
        let Some(kart) = double_dash::Kart::from_index(data.kart) else {
            return MkdededeDecodeStatus::InvalidKart;
        };
        let Some(driver1) = double_dash::Character::from_index(data.driver1) else {
            return MkdededeDecodeStatus::InvalidDriver;
        };
        let Some(driver2) = double_dash::Character::from_index(data.driver2) else {
            return MkdededeDecodeStatus::InvalidDriver;
        };

        let ghost_data = double_dash::GhostData {
            course,
            kart,
            driver1,
            driver2,
            total_time: double_dash::LapTime(data.total_time_ms),
            best_lap: double_dash::LapTime(data.best_lap_ms),
        };

        let pass = double_dash::encode(&ghost_data);
        let pass_c = CString::new(pass).unwrap();

        // Copy the resulting string including null terminator
        ptr::copy_nonoverlapping(pass_c.as_ptr(), out_password, 17);
    }

    MkdededeDecodeStatus::Success
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mkdedede_ds_encode(
    data: *const DsGhostDataC,
    out_password: *mut c_char,
) -> MkdededeDecodeStatus {
    if data.is_null() || out_password.is_null() {
        return MkdededeDecodeStatus::NullPointer;
    }

    unsafe {
        let data = &*data;

        let Some(course) = ds::Course::from_index(data.course) else {
            return MkdededeDecodeStatus::InvalidCourse;
        };
        let Some(kart) = ds::Kart::from_index(data.kart) else {
            return MkdededeDecodeStatus::InvalidKart;
        };
        let Some(character) = ds::Character::from_index(data.character) else {
            return MkdededeDecodeStatus::InvalidCharacter;
        };

        let player_chars = [data.player_name[0] as u32, data.player_name[1] as u32];
        let mut player = ['\0', '\0'];
        if let Some(c) = std::char::from_u32(player_chars[0]) {
            player[0] = c;
        }
        if let Some(c) = std::char::from_u32(player_chars[1]) {
            player[1] = c;
        }

        let ghost_data = ds::GhostData {
            course,
            kart,
            character,
            time: ds::RaceTime(data.time_ms),
            player,
        };

        let pass = ds::encode(&ghost_data);
        let pass_c = CString::new(pass).unwrap();

        // Copy the resulting string including null terminator
        ptr::copy_nonoverlapping(pass_c.as_ptr(), out_password, 17);
    }

    MkdededeDecodeStatus::Success
}
