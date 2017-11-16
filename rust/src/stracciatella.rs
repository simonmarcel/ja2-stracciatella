#![crate_type = "lib"]

extern crate getopts;
extern crate libc;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate user32;
#[cfg(windows)]
extern crate shell32;

use std::slice;
use std::str;
use std::str::FromStr;
use std::ptr;
use std::fs;
use std::ffi::{CStr, CString};
use std::path::PathBuf;

use libc::{size_t, c_char};

mod cli;
mod config;
mod engine;
mod resources;

use engine::EngineOptions;
use resources::ResourceVersion;
use cli::get_command_line_options;
use config::{write_json_config, build_engine_options_from_env_and_args};

macro_rules! unsafe_from_ptr {
    ($ptr:expr) => { unsafe { assert!(!$ptr.is_null()); &*$ptr } }
}

macro_rules! unsafe_from_ptr_mut {
    ($ptr:expr) => { unsafe { assert!(!$ptr.is_null()); &mut *$ptr } }
}

#[no_mangle]
pub fn create_engine_options(array: *const *const c_char, length: size_t) -> *mut EngineOptions {
    let values = unsafe { slice::from_raw_parts(array, length as usize) };
    let args: Vec<String> = values.iter()
        .map(|&p| unsafe { CStr::from_ptr(p) })  // iterator of &CStr
        .map(|cs| cs.to_bytes())                 // iterator of &[u8]
        .map(|bs| String::from(str::from_utf8(bs).unwrap()))   // iterator of &str
        .collect();

    return match build_engine_options_from_env_and_args(args) {
        Ok(engine_options) => {
            if engine_options.show_help {
                let opts = get_command_line_options();
                let brief = format!("Usage: ja2 [options]");
                print!("{}", opts.usage(&brief));
            }
            Box::into_raw(Box::new(engine_options))
        },
        Err(msg) => {
            println!("{}", msg);
            return ptr::null_mut();
        }
    };
}

#[no_mangle]
pub fn write_engine_options(ptr: *mut EngineOptions) -> bool {
    let engine_options = unsafe_from_ptr!(ptr);
    write_json_config(engine_options).is_ok()
}

#[no_mangle]
pub fn free_engine_options(ptr: *mut EngineOptions) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn get_stracciatella_home(ptr: *const EngineOptions) -> *mut c_char {
    let c_str_home = CString::new(unsafe_from_ptr!(ptr).stracciatella_home.to_str().unwrap()).unwrap();
    c_str_home.into_raw()
}

#[no_mangle]
pub extern fn get_vanilla_data_dir(ptr: *const EngineOptions) -> *mut c_char {
    let c_str_home = CString::new(unsafe_from_ptr!(ptr).vanilla_data_dir.to_str().unwrap()).unwrap();
    c_str_home.into_raw()
}

#[no_mangle]
pub extern fn set_vanilla_data_dir(ptr: *mut EngineOptions, data_dir_ptr: *const c_char) -> () {
    let c_str = unsafe { CStr::from_ptr(data_dir_ptr) };
    unsafe_from_ptr_mut!(ptr).vanilla_data_dir = PathBuf::from(c_str.to_string_lossy().into_owned());
}

#[no_mangle]
pub extern fn get_number_of_mods(ptr: *const EngineOptions) -> u32 {
    return unsafe_from_ptr!(ptr).mods.len() as u32
}

#[no_mangle]
pub extern fn get_mod(ptr: *const EngineOptions, index: u32) -> *mut c_char {
    let str_mod = match unsafe_from_ptr!(ptr).mods.get(index as usize) {
        Some(m) => m,
        None => panic!("Invalid mod index for game options {}", index)
    };
    let c_str_mod = CString::new(str_mod.clone()).unwrap();
    c_str_mod.into_raw()
}

#[no_mangle]
pub extern fn get_resolution_x(ptr: *const EngineOptions) -> u16 {
    unsafe_from_ptr!(ptr).resolution.0
}

#[no_mangle]
pub extern fn get_resolution_y(ptr: *const EngineOptions) -> u16 {
    unsafe_from_ptr!(ptr).resolution.1
}

#[no_mangle]
pub extern fn set_resolution(ptr: *mut EngineOptions, x: u16, y: u16) -> () {
    unsafe_from_ptr_mut!(ptr).resolution = (x, y)
}

#[no_mangle]
pub extern fn get_resource_version(ptr: *const EngineOptions) -> ResourceVersion {
    unsafe_from_ptr!(ptr).resource_version
}

#[no_mangle]
pub extern fn set_resource_version(ptr: *mut EngineOptions, res_ptr: *const c_char) -> () {
    let c_str = unsafe { CStr::from_ptr(res_ptr) };
    let version = c_str.to_str().unwrap();

    if let Ok(v) = ResourceVersion::from_str(version) {
        unsafe_from_ptr_mut!(ptr).resource_version = v
    }
}

#[no_mangle]
pub fn should_run_unittests(ptr: *const EngineOptions) -> bool {
    unsafe_from_ptr!(ptr).run_unittests
}

#[no_mangle]
pub fn should_show_help(ptr: *const EngineOptions) -> bool {
    unsafe_from_ptr!(ptr).show_help
}

#[no_mangle]
pub fn should_run_editor(ptr: *const EngineOptions) -> bool {
    unsafe_from_ptr!(ptr).run_editor
}

#[no_mangle]
pub fn should_start_in_fullscreen(ptr: *const EngineOptions) -> bool {
    unsafe_from_ptr!(ptr).start_in_fullscreen
}

#[no_mangle]
pub fn set_start_in_fullscreen(ptr: *mut EngineOptions, val: bool) -> () {
    unsafe_from_ptr_mut!(ptr).start_in_fullscreen = val
}

#[no_mangle]
pub fn should_start_in_window(ptr: *const EngineOptions) -> bool {
    unsafe_from_ptr!(ptr).start_in_window
}

#[no_mangle]
pub fn should_start_in_debug_mode(ptr: *const EngineOptions) -> bool {
    unsafe_from_ptr!(ptr).start_in_debug_mode
}

#[no_mangle]
pub fn should_start_without_sound(ptr: *const EngineOptions) -> bool {
    unsafe_from_ptr!(ptr).start_without_sound
}

#[no_mangle]
pub fn set_start_without_sound(ptr: *mut EngineOptions, val: bool) -> () {
    unsafe_from_ptr_mut!(ptr).start_without_sound = val
}

#[no_mangle]
pub extern fn get_resource_version_string(version: ResourceVersion) -> *mut c_char {
    let c_str_home = CString::new(version.to_string()).unwrap();
    c_str_home.into_raw()
}

#[no_mangle]
pub extern fn find_ja2_executable(launcher_path_ptr: *const c_char) -> *const c_char {
    let launcher_path = unsafe { CStr::from_ptr(launcher_path_ptr).to_string_lossy() };
    let is_exe = launcher_path.to_lowercase().ends_with(".exe");
    let end_of_executable_slice = launcher_path.len() - if is_exe { 13 } else { 9 };
    let mut executable_path = String::from(&launcher_path[0..end_of_executable_slice]);

    if is_exe {
        executable_path.push_str(if is_exe { ".exe" } else { "" });
    }

    CString::new(executable_path).unwrap().into_raw()
}

#[no_mangle]
pub fn free_rust_string(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}


#[cfg(test)]
mod tests {
    use std::str;
    use std::ffi::{CStr, CString};

    macro_rules! assert_chars_eq { ($got:expr, $expected:expr) => {
        unsafe {
            assert_eq!(str::from_utf8(CStr::from_ptr($got).to_bytes()).unwrap(), $expected);
        }
    } }

    #[test]
    fn get_resource_version_string_should_return_the_correct_resource_version_string() {
        assert_chars_eq!(super::get_resource_version_string(super::ResourceVersion::DUTCH), "DUTCH");
        assert_chars_eq!(super::get_resource_version_string(super::ResourceVersion::ENGLISH), "ENGLISH");
        assert_chars_eq!(super::get_resource_version_string(super::ResourceVersion::FRENCH), "FRENCH");
        assert_chars_eq!(super::get_resource_version_string(super::ResourceVersion::GERMAN), "GERMAN");
        assert_chars_eq!(super::get_resource_version_string(super::ResourceVersion::ITALIAN), "ITALIAN");
        assert_chars_eq!(super::get_resource_version_string(super::ResourceVersion::POLISH), "POLISH");
        assert_chars_eq!(super::get_resource_version_string(super::ResourceVersion::RUSSIAN), "RUSSIAN");
        assert_chars_eq!(super::get_resource_version_string(super::ResourceVersion::RUSSIAN_GOLD), "RUSSIAN_GOLD");
    }

    #[test]
    fn find_ja2_executable_should_determine_game_path_from_launcher_path() {
        assert_chars_eq!(super::find_ja2_executable(CString::new("/home/test/ja2-launcher").unwrap().as_ptr()), "/home/test/ja2");
        assert_chars_eq!(super::find_ja2_executable(CString::new("C:\\\\home\\\\test\\\\ja2-launcher.exe").unwrap().as_ptr()), "C:\\\\home\\\\test\\\\ja2.exe");
        assert_chars_eq!(super::find_ja2_executable(CString::new("ja2-launcher").unwrap().as_ptr()), "ja2");
        assert_chars_eq!(super::find_ja2_executable(CString::new("ja2-launcher.exe").unwrap().as_ptr()), "ja2.exe");
        assert_chars_eq!(super::find_ja2_executable(CString::new("JA2-LAUNCHER.EXE").unwrap().as_ptr()), "JA2.exe");
    }
}
