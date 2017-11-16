use fs;
use std::path::PathBuf;
use std::error::Error;
use std::io::Write;
use serde_json;

use engine::EngineOptions;

#[cfg(not(windows))]
static DEFAULT_JSON_CONTENT: &'static str = r##"{
    "help": "Put the directory to your original ja2 installation into the line below",
    "data_dir": "/some/place/where/the/data/is"
}"##;


#[cfg(windows)]
static DEFAULT_JSON_CONTENT: &'static str = r##"{
   "help": "Put the directory to your original ja2 installation into the line below. Make sure to use double backslashes.",
   "data_dir": "C:\\Program Files\\Jagged Alliance 2"
}"##;

fn build_json_config_location(stracciatella_home: &PathBuf) -> PathBuf {
    let mut path = PathBuf::from(stracciatella_home);
    path.push("ja2.json");
    return path;
}

pub fn ensure_json_config_existence(stracciatella_home: PathBuf) -> Result<PathBuf, String> {
    macro_rules! make_string_err { ($msg:expr) => { $msg.map_err(|why| format!("! {:?}", why.kind())) }; }

    let path = build_json_config_location(&stracciatella_home);

    if !stracciatella_home.exists() {
        try!(make_string_err!(fs::create_dir_all(&stracciatella_home)));
    }

    if !path.is_file() {
        let mut f = try!(make_string_err!(fs::File::create(path)));
        try!(make_string_err!(f.write_all(DEFAULT_JSON_CONTENT.as_bytes())));
    }

    return Ok(stracciatella_home);
}


pub fn parse_json_config(stracciatella_home: PathBuf) -> Result<EngineOptions, String> {
    let path = build_json_config_location(&stracciatella_home);
    return fs::File::open(path).map_err(|s| format!("Error reading ja2.json config file: {}", s.description()))
        .and_then(|f| serde_json::from_reader(f).map_err(|s| format!("Error parsing ja2.json config file: {}", s)))
        .map(|mut engine_options: EngineOptions| {
            engine_options.stracciatella_home = stracciatella_home;
            engine_options
        });
}

pub fn write_json_config(engine_options: &EngineOptions) -> Result<(), String> {
    let json = serde_json::to_string_pretty(engine_options).map_err(|s| format!("Error creating contents of ja2.json config file: {}", s.description()))?;
    let path = build_json_config_location(&engine_options.stracciatella_home);
    let mut f = fs::File::create(path).map_err(|s| format!("Error creating ja2.json config file: {}", s.description()))?;

    f.write_all(json.as_bytes()).map_err(|s| format!("Error creating ja2.json config file: {}", s.description()))
}

#[cfg(not(windows))]
pub fn find_stracciatella_home() -> Result<PathBuf, String> {
    use std::env;

    match env::home_dir() {
        Some(mut path) => {
            path.push(".ja2");
            return Ok(path);
        },
        None => Err(String::from("Could not find home directory")),
    }
}

#[cfg(windows)]
pub fn find_stracciatella_home() -> Result<PathBuf, String> {
    use shell32::SHGetFolderPathW;
    use winapi::shlobj::{CSIDL_PERSONAL, CSIDL_FLAG_CREATE};
    use winapi::minwindef::MAX_PATH;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    let mut home: [u16; MAX_PATH] = [0; MAX_PATH];

    return match unsafe { SHGetFolderPathW(ptr::null_mut(), CSIDL_PERSONAL | CSIDL_FLAG_CREATE, ptr::null_mut(), 0, home.as_mut_ptr()) } {
        0 => {
            let home_trimmed: Vec<u16> = home.iter().take_while(|x| **x != 0).map(|x| *x).collect();

            return match OsString::from_wide(&home_trimmed).to_str() {
                Some(s) => {
                    let mut buf = PathBuf::from(s);
                    buf.push("JA2");
                    return Ok(buf);
                },
                None => Err(format!("Could not decode documents folder string."))
            }
        },
        i => Err(format!("Could not get documents folder: {}", i))
    };
}

#[cfg(test)]
mod tests {
   extern crate tempdir;

   use fs;
   use std::env;
   use std::path::PathBuf;
   use std::io::{Write, Read};

   use resources::ResourceVersion;

   fn write_temp_folder_with_ja2_ini(contents: &[u8]) -> tempdir::TempDir {
        let dir = tempdir::TempDir::new("ja2-test").unwrap();
        let ja2_home_dir = dir.path().join(".ja2");
        let file_path = ja2_home_dir.join("ja2.json");

        fs::create_dir(ja2_home_dir).unwrap();
        let mut f = fs::File::create(file_path).unwrap();
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

      let mut f = fs::File::open(ja2json_path.clone()).unwrap();
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
    fn parse_json_config_should_set_stracciatella_home() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{}");
        let stracciatella_home = PathBuf::from(temp_dir.path().join(".ja2"));
        let engine_options = super::parse_json_config(stracciatella_home.clone()).unwrap();

        assert_eq!(engine_options.stracciatella_home, stracciatella_home);
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_set_stracciatella_home() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"stracciatella_home\": \"/aaa\" }");
        let stracciatella_home = PathBuf::from(temp_dir.path().join(".ja2"));
        let engine_options = super::parse_json_config(stracciatella_home.clone()).unwrap();

        assert_eq!(engine_options.stracciatella_home, stracciatella_home);
    }

    #[test]
    fn parse_json_config_should_be_able_to_change_data_dir() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"data_dir\": \"/dd\" }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert_eq!(engine_options.vanilla_data_dir.to_str().unwrap(), "/dd");
    }

    #[test]
    fn parse_json_config_should_be_able_to_change_fullscreen_value() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"fullscreen\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(engine_options.start_in_fullscreen);
    }

    #[test]
    fn parse_json_config_should_be_able_to_change_debug_value() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"debug\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(engine_options.start_in_debug_mode);
    }

    #[test]
    fn parse_json_config_should_be_able_to_start_without_sound() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"nosound\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(engine_options.start_without_sound);
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_run_help() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"help\": true, \"show_help\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(!engine_options.show_help);
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_run_unittests() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"unittests\": true, \"run_unittests\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(!engine_options.run_unittests);
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_run_editor() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"editor\": true, \"run_editor\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(!engine_options.run_editor);
    }

    #[test]
    fn parse_json_config_should_not_be_able_start_in_window_explicitly() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"window\": true, \"start_in_window\": true }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert!(!engine_options.start_in_window);
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

        assert!(engine_options.start_in_debug_mode);
        assert_eq!(engine_options.mods, vec!["m1", "a2"]);
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

        assert_eq!(engine_options.resource_version, ResourceVersion::RUSSIAN);
    }

    #[test]
    fn parse_json_config_should_return_the_correct_resversion_for_italian() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"resversion\": \"ITALIAN\" }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert_eq!(engine_options.resource_version, ResourceVersion::ITALIAN);
    }

    #[test]
    fn parse_json_config_should_return_the_correct_resolution() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"res\": \"1024x768\" }");
        let engine_options = super::parse_json_config(PathBuf::from(temp_dir.path().join(".ja2"))).unwrap();

        assert_eq!(engine_options.resolution, (1024, 768));
    }

    #[test]
    #[cfg(not(windows))]
    fn find_stracciatella_home_should_find_the_correct_stracciatella_home_path_on_unixlike() {
        let mut engine_options: super::EngineOptions = Default::default();
        engine_options.stracciatella_home = super::find_stracciatella_home().unwrap();

        assert_eq!(engine_options.stracciatella_home.to_str().unwrap(), format!("{}/.ja2", env::var("HOME").unwrap()));
    }

    #[test]
    #[cfg(windows)]
    fn find_stracciatella_home_should_find_the_correct_stracciatella_home_path_on_windows() {
        use self::regex::Regex;

        let mut engine_options: super::EngineOptions = Default::default();
        engine_options.stracciatella_home = super::find_stracciatella_home().unwrap();

        let regex = Regex::new(r"^[A-Z]:\\(.*)+\\JA2").unwrap();
        assert!(regex.is_match(engine_options.stracciatella_home.to_str().unwrap()), "{} is not a valid home dir for windows", result);
    }

    #[test]
    fn write_json_config_should_write_a_json_file_that_can_be_serialized_again() {
        let mut engine_options = super::EngineOptions::default();
        let temp_dir = write_temp_folder_with_ja2_ini(b"Invalid JSON");
        let stracciatella_home = PathBuf::from(temp_dir.path().join(".ja2"));

        engine_options.stracciatella_home = stracciatella_home.clone();
        engine_options.resolution = (100, 100);

        super::write_json_config(&mut engine_options).unwrap();

        let got_engine_options = super::parse_json_config(stracciatella_home).unwrap();

        assert_eq!(got_engine_options.resolution, engine_options.resolution);
    }

    #[test]
    fn write_json_config_should_write_a_pretty_json_file() {
        let mut engine_options = super::EngineOptions::default();
        let temp_dir = write_temp_folder_with_ja2_ini(b"Invalid JSON");
        let stracciatella_home = PathBuf::from(temp_dir.path().join(".ja2"));
        let stracciatella_json = PathBuf::from(temp_dir.path().join(".ja2/ja2.json"));

        engine_options.stracciatella_home = stracciatella_home.clone();
        engine_options.resolution = (100, 100);

        super::write_json_config(&mut engine_options).unwrap();

        let mut config_file_contents = String::from("");
        fs::File::open(stracciatella_json).unwrap().read_to_string(&mut config_file_contents).unwrap();

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
}
