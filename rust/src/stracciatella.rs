#![crate_type = "lib"]

extern crate stracciatella;
extern crate libc;
extern crate serde;
extern crate serde_json;

use std::slice;
use std::str;
use std::str::FromStr;
use std::ptr;
use std::ffi::{CStr, CString};
use std::path::PathBuf;
use stracciatella::config::{build_engine_options_from_env_and_args, EngineOptions, Cli, JsonConfig};
use stracciatella::resources::{ResourceVersion};

use libc::{size_t, c_char};

fn parse_args(mut engine_options: &mut EngineOptions, args: Vec<String>) -> Option<String> {
   Cli::new(args).merge_options(&mut engine_options).err()
}

fn ensure_json_config_existence(stracciatella_home: PathBuf) -> Result<PathBuf, String> {
   JsonConfig::new(&stracciatella_home).ensure_existence()?;
   Ok(stracciatella_home)
}

fn parse_json_config(stracciatella_home: PathBuf) -> Result<EngineOptions, String> {
   JsonConfig::new(&stracciatella_home).parse()
}

fn write_json_config(engine_options: &EngineOptions) -> Result<(), String> {
   JsonConfig::new(&engine_options.stracciatella_home).write(engine_options)
}

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
               let opts = Cli::options();
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
    extern crate regex;
    extern crate tempdir;

    use std::path::{PathBuf};
    use std::str;
    use std::ffi::{CStr, CString};
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::env;

    macro_rules! assert_chars_eq { ($got:expr, $expected:expr) => {
        unsafe {
            assert_eq!(str::from_utf8(CStr::from_ptr($got).to_bytes()).unwrap(), $expected);
        }
    } }

    #[test]
    fn parse_args_should_abort_on_unknown_arguments() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("testunknown"));
        assert_eq!(super::parse_args(&mut engine_options, input).unwrap(), "Unknown arguments: 'testunknown'.");
    }

    #[test]
    fn parse_args_should_abort_on_unknown_switch() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("--testunknown"));
        assert_eq!(super::parse_args(&mut engine_options, input).unwrap(), "Unrecognized option: 'testunknown'");
    }

    #[test]
    fn parse_args_should_have_correct_fullscreen_default_value() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert!(!super::should_start_in_fullscreen(&engine_options));
    }

    #[test]
    fn parse_args_should_be_able_to_change_fullscreen_value() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("-fullscreen"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert!(super::should_start_in_fullscreen(&engine_options));
    }

    #[test]
    fn parse_args_should_be_able_to_show_help() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("-help"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert!(super::should_show_help(&engine_options));
    }

    #[test]
    fn parse_args_should_continue_with_multiple_known_switches() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("-debug"), String::from("-mod"), String::from("a"), String::from("--mod"), String::from("ö"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert!(super::should_start_in_debug_mode(&engine_options));
        assert_eq!(super::get_number_of_mods(&engine_options), 2);
        unsafe {
            assert_eq!(CString::from_raw(super::get_mod(&engine_options, 0)), CString::new("a").unwrap());
            assert_eq!(CString::from_raw(super::get_mod(&engine_options, 1)), CString::new("ö").unwrap());
        }
    }

    #[test]
    fn parse_args_should_fail_with_unknown_resversion() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("--resversion"), String::from("TESTUNKNOWN"));
        assert_eq!(super::parse_args(&mut engine_options, input).unwrap(), "Resource version TESTUNKNOWN is unknown");
    }

    #[test]
    fn parse_args_should_return_the_correct_resversion_for_russian() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("-resversion"), String::from("RUSSIAN"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert!(super::get_resource_version(&engine_options) == super::ResourceVersion::RUSSIAN);
    }

    #[test]
    fn parse_args_should_return_the_correct_resversion_for_italian() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("-resversion"), String::from("ITALIAN"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert!(super::get_resource_version(&engine_options) == super::ResourceVersion::ITALIAN);
    }

    #[test]
    fn parse_args_should_return_the_correct_resolution() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("--res"), String::from("1120x960"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert_eq!(super::get_resolution_x(&engine_options), 1120);
        assert_eq!(super::get_resolution_y(&engine_options), 960);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn parse_args_should_return_the_correct_canonical_data_dir_on_mac() {
        let mut engine_options: super::EngineOptions = Default::default();
        let temp_dir = tempdir::TempDir::new("ja2-tests").unwrap();
        let dir_path = temp_dir.path().join("foo");

        fs::create_dir_all(dir_path).unwrap();

        let input = vec!(String::from("ja2"), String::from("--datadir"), String::from(temp_dir.path().join("foo/../foo/../").to_str().unwrap()));

        assert_eq!(super::parse_args(&mut engine_options, input), None);
        unsafe {
            let comp = str::from_utf8(CStr::from_ptr(super::get_vanilla_data_dir(&engine_options)).to_bytes()).unwrap();
            let temp = fs::canonicalize(temp_dir.path()).expect("Problem during building of reference value.");
            let base = temp.to_str().unwrap();

            assert_eq!(comp, base);
        }
    }

    #[test]
    #[cfg(all(not(windows), not(target_os = "macos")))]
    fn parse_args_should_return_the_correct_canonical_data_dir_on_linux() {
        let mut engine_options: super::EngineOptions = Default::default();
        let temp_dir = tempdir::TempDir::new("ja2-tests").unwrap();
        let dir_path = temp_dir.path().join("foo");

        fs::create_dir_all(dir_path).unwrap();

        let input = vec!(String::from("ja2"), String::from("--datadir"), String::from(temp_dir.path().join("foo/../foo/../").to_str().unwrap()));

        assert_eq!(super::parse_args(&mut engine_options, input), None);
        unsafe {
            assert_eq!(str::from_utf8(CStr::from_ptr(super::get_vanilla_data_dir(&engine_options)).to_bytes()).unwrap(), temp_dir.path().to_str().unwrap());
        }
    }

    #[test]
    #[cfg(windows)]
    fn parse_args_should_return_the_correct_canonical_data_dir_on_windows() {
        let mut engine_options: super::EngineOptions = Default::default();
        let temp_dir = tempdir::TempDir::new("ja2-tests").unwrap();
        let dir_path = temp_dir.path().join("foo");

        fs::create_dir_all(dir_path).unwrap();

        let input = vec!(String::from("ja2"), String::from("--datadir"), String::from(temp_dir.path().to_str().unwrap()));

        assert_eq!(super::parse_args(&mut engine_options, input), None);
        unsafe {
            assert_eq!(str::from_utf8(CStr::from_ptr(super::get_vanilla_data_dir(&engine_options)).to_bytes()).unwrap(), temp_dir.path().to_str().unwrap());
        }
    }

    #[test]
    fn parse_args_should_fail_with_non_existing_directory() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("--datadir"), String::from("somethingelse"));

        assert_eq!(super::parse_args(&mut engine_options, input), Some(String::from("Please specify an existing datadir.")));
    }

    fn write_temp_folder_with_ja2_ini(contents: &[u8]) -> tempdir::TempDir {
        let dir = tempdir::TempDir::new("ja2-test").unwrap();
        let ja2_home_dir = dir.path().join(".ja2");
        let file_path = ja2_home_dir.join("ja2.json");

        fs::create_dir(ja2_home_dir).unwrap();
        let mut f = File::create(file_path).unwrap();
        f.write_all(contents).unwrap();
        f.sync_all().unwrap();

        return dir
    }

    #[test]
    fn ensure_json_config_existence_should_ensure_existence_of_config_dir() {
        let dir = tempdir::TempDir::new("ja2-tests").unwrap();
        let home_path = dir.path().join("ja2_home");
        let ja2json_path = home_path.join("ja2.json");

        super::ensure_json_config_existence(home_path.clone()).unwrap();

        assert!(home_path.exists());
        assert!(ja2json_path.is_file());
    }

    #[test]
    fn ensure_json_config_existence_should_not_overwrite_existing_ja2json() {
        let dir = write_temp_folder_with_ja2_ini(b"Test");
        let ja2json_path = dir.path().join(".ja2/ja2.json");

        super::ensure_json_config_existence(PathBuf::from(dir.path())).unwrap();

        let mut f = File::open(ja2json_path.clone()).unwrap();
        let mut content: Vec<u8> = vec!();
        f.read_to_end(&mut content).unwrap();

        assert!(ja2json_path.is_file());
        assert_eq!(content, b"Test");
    }

    #[test]
    fn parse_json_config_should_fail_with_missing_file() {
        let temp_dir = tempdir::TempDir::new("ja2-tests").unwrap();
        let stracciatella_home = PathBuf::from(temp_dir.path());

        assert_eq!(super::parse_json_config(stracciatella_home), Err(String::from("Error reading ja2.json config file: entity not found")));
    }

    #[test]
    fn parse_json_config_should_fail_with_invalid_json() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ not json }");
        let stracciatella_home = PathBuf::from(temp_dir.path().join(".ja2"));

        assert_eq!(super::parse_json_config(stracciatella_home), Err(String::from("Error parsing ja2.json config file: key must be a string at line 1 column 3")));
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_set_stracciatella_home() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"stracciatella_home\": \"/aaa\" }");
        let stracciatella_home = PathBuf::from(temp_dir.path().join(".ja2"));
        let engine_options = super::parse_json_config(stracciatella_home.clone()).unwrap();

        assert_eq!(engine_options.stracciatella_home, PathBuf::from(""));
    }

    #[test]
    fn parse_json_config_should_be_able_to_change_data_dir() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"data_dir\": \"/dd\" }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert_chars_eq!(super::get_vanilla_data_dir(&engine_options), "/dd");
    }

    #[test]
    fn parse_json_config_should_be_able_to_change_fullscreen_value() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"fullscreen\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(super::should_start_in_fullscreen(&engine_options));
    }

    #[test]
    fn parse_json_config_should_be_able_to_change_debug_value() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"debug\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(super::should_start_in_debug_mode(&engine_options));
    }

    #[test]
    fn parse_json_config_should_be_able_to_start_without_sound() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"nosound\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(super::should_start_without_sound(&engine_options));
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_run_help() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"help\": true, \"show_help\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(!super::should_show_help(&engine_options));
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_run_unittests() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"unittests\": true, \"run_unittests\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(!super::should_run_unittests(&engine_options));
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_run_editor() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"editor\": true, \"run_editor\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(!super::should_run_editor(&engine_options));
    }

    #[test]
    fn parse_json_config_should_not_be_able_start_in_window_explicitly() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"window\": true, \"start_in_window\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(!super::should_start_in_window(&engine_options));
    }

    #[test]
    fn parse_json_config_should_fail_with_invalid_mod() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"mods\": [ \"a\", true ] }");
        let stracciatella_home = PathBuf::from(temp_dir.path().join(".ja2"));

        assert_eq!(super::parse_json_config(stracciatella_home), Err(String::from("Error parsing ja2.json config file: invalid type: boolean `true`, expected a string at line 1 column 21")));
    }

    #[test]
    fn parse_json_config_should_continue_with_multiple_known_switches() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"debug\": true, \"mods\": [ \"m1\", \"a2\" ] }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(super::should_start_in_debug_mode(&engine_options));
        assert!(super::get_number_of_mods(&engine_options) == 2);
    }

    #[test]
    fn parse_json_config_should_fail_with_unknown_resversion() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"resversion\": \"TESTUNKNOWN\" }");
        let stracciatella_home = PathBuf::from(temp_dir.path().join(".ja2"));

        assert_eq!(super::parse_json_config(stracciatella_home), Err(String::from("Error parsing ja2.json config file: unknown variant `TESTUNKNOWN`, expected one of `DUTCH`, `ENGLISH`, `FRENCH`, `GERMAN`, `ITALIAN`, `POLISH`, `RUSSIAN`, `RUSSIAN_GOLD` at line 1 column 29")));
    }

    #[test]
    fn parse_json_config_should_return_the_correct_resversion_for_russian() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"resversion\": \"RUSSIAN\" }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert_eq!(super::get_resource_version(&engine_options), super::ResourceVersion::RUSSIAN);
    }

    #[test]
    fn parse_json_config_should_return_the_correct_resversion_for_italian() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"resversion\": \"ITALIAN\" }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert_eq!(super::get_resource_version(&engine_options), super::ResourceVersion::ITALIAN);
    }

    #[test]
    fn parse_json_config_should_return_the_correct_resolution() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"res\": \"1024x768\" }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert_eq!(super::get_resolution_x(&engine_options), 1024);
        assert_eq!(super::get_resolution_y(&engine_options), 768);
    }

    #[test]
    #[cfg(not(windows))]
    fn build_engine_options_from_env_and_args_should_overwrite_json_with_command_line_args() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"data_dir\": \"/some/place/where/the/data/is\", \"res\": \"1024x768\", \"fullscreen\": true }");
        let args = vec!(String::from("ja2"), String::from("--res"), String::from("1100x480"));
        let old_home = env::var("HOME");

        env::set_var("HOME", temp_dir.path());
        let engine_options_res = super::build_engine_options_from_env_and_args(args);
        match old_home {
            Ok(home) => env::set_var("HOME", home),
            _ => {}
        }
        let engine_options = engine_options_res.unwrap();

        assert_eq!(super::get_resolution_x(&engine_options), 1100);
        assert_eq!(super::get_resolution_y(&engine_options), 480);
        assert_eq!(super::should_start_in_fullscreen(&engine_options), true);
    }

    #[test]
    #[cfg(not(windows))]
    fn build_engine_options_from_env_and_args_should_return_an_error_if_datadir_is_not_set() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"res\": \"1024x768\", \"fullscreen\": true }");
        let args = vec!(String::from("ja2"), String::from("--res"), String::from("1100x480"));
        let old_home = env::var("HOME");
        let expected_error_message = "Vanilla data directory has to be set either in config file or per command line switch";

        env::set_var("HOME", temp_dir.path());
        let engine_options_res = super::build_engine_options_from_env_and_args(args);
        match old_home {
            Ok(home) => env::set_var("HOME", home),
            _ => {}
        }
        assert_eq!(engine_options_res, Err(String::from(expected_error_message)));
    }

    #[test]
    fn write_engine_options_should_write_a_json_file_that_can_be_serialized_again() {
        let mut engine_options = super::EngineOptions::default();
        let temp_dir = write_temp_folder_with_ja2_ini(b"Invalid JSON");
        let stracciatella_home = PathBuf::from(temp_dir.path().join(".ja2"));

        engine_options.stracciatella_home = stracciatella_home.clone();
        engine_options.resolution = (100, 100);

        super::write_engine_options(&mut engine_options);

        let got_engine_options = super::parse_json_config(stracciatella_home).unwrap();

        assert_eq!(got_engine_options.resolution, engine_options.resolution);
    }

    #[test]
    fn write_engine_options_should_write_a_pretty_json_file() {
        let mut engine_options = super::EngineOptions::default();
        let temp_dir = write_temp_folder_with_ja2_ini(b"Invalid JSON");
        let stracciatella_home = PathBuf::from(temp_dir.path().join(".ja2"));
        let stracciatella_json = PathBuf::from(temp_dir.path().join(".ja2/ja2.json"));

        engine_options.stracciatella_home = stracciatella_home.clone();
        engine_options.resolution = (100, 100);

        super::write_engine_options(&mut engine_options);

        let mut config_file_contents = String::from("");
        File::open(stracciatella_json).unwrap().read_to_string(&mut config_file_contents).unwrap();

        assert_eq!(config_file_contents,
r##"{
  "data_dir": "",
  "mods": [],
  "res": "100x100",
  "resversion": "ENGLISH",
  "fullscreen": false,
  "debug": false,
  "nosound": false
}"##);
    }

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
