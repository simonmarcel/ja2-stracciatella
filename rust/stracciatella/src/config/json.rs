use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use serde_json;

use config::engine::EngineOptions;

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

pub struct JsonConfig {
   path: PathBuf
}

impl JsonConfig {
   pub fn new(stracciatella_home: &Path) -> JsonConfig {
       let mut path = PathBuf::from(stracciatella_home);

       path.push("ja2.json");

      JsonConfig { path: path }
   }

   pub fn ensure_existence(self: &JsonConfig) -> Result<(), String> {
      macro_rules! make_string_err { ($msg:expr) => { $msg.map_err(|why| format!("! {:?}", why.kind())) }; }

      if let Some(parent) = self.path.parent() {
          make_string_err!(fs::create_dir_all(&parent))?;
      }

      if !self.path.is_file() {
          let mut f = make_string_err!(File::create(&self.path))?;
          make_string_err!(f.write_all(DEFAULT_JSON_CONTENT.as_bytes()))?;
      }

      return Ok(());
   }

   pub fn parse(self: &JsonConfig) -> Result<EngineOptions, String> {
       return File::open(&self.path).map_err(|s| format!("Error reading ja2.json config file: {}", s.description()))
           .and_then(|f| serde_json::from_reader(f).map_err(|s| format!("Error parsing ja2.json config file: {}", s)));
   }

   pub fn write(self: &JsonConfig, engine_options: &EngineOptions) -> Result<(), String> {
       let json = serde_json::to_string_pretty(engine_options).map_err(|s| format!("Error creating contents of ja2.json config file: {}", s.description()))?;
       let mut f = File::create(&self.path).map_err(|s| format!("Error creating ja2.json config file: {}", s.description()))?;

       f.write_all(json.as_bytes()).map_err(|s| format!("Error writing ja2.json config file: {}", s.description()))
   }
}

#[cfg(test)]
mod tests {
   extern crate tempdir;

   use super::*;
   use std::io::Read;
   use resources::ResourceVersion;

   fn write_temp_folder_with_ja2_ini(contents: &[u8]) -> tempdir::TempDir {
        let dir = tempdir::TempDir::new("ja2-test").unwrap();
        let mut f = File::create(dir.path().join("ja2.json")).unwrap();

        f.write_all(contents).unwrap();
        f.sync_all().unwrap();

        dir
    }

    #[test]
    fn it_should_be_instantiable() {
        JsonConfig::new(&PathBuf::from("/test"));
    }

    #[test]
    fn it_should_be_able_to_ensure_that_json_exists() {
        let dir = tempdir::TempDir::new("ja2-tests").unwrap();
        let ja2json_path = dir.path().join("ja2.json");
        let cfg = JsonConfig::new(&PathBuf::from(dir.path()));

        cfg.ensure_existence().unwrap();

        assert!(ja2json_path.exists());
        assert!(ja2json_path.is_file());
    }

    #[test]
    fn it_should_be_able_to_ensure_that_json_exists_when_directory_does_not_exist() {
        let dir = tempdir::TempDir::new("ja2-tests").unwrap();
        let home_path = dir.path().join("ja2_home");
        let ja2json_path = home_path.join("ja2.json");
        let cfg = JsonConfig::new(&home_path);

        fs::create_dir_all(dir.path()).unwrap();
        cfg.ensure_existence().unwrap();

        assert!(home_path.exists());
        assert!(ja2json_path.is_file());
    }

    #[test]
    fn ensuring_json_config_existence_should_not_overwrite_existing_file() {
         let dir = tempdir::TempDir::new("ja2-tests").unwrap();
         let ja2json_path = dir.path().join("ja2.json");
         let cfg = JsonConfig::new(dir.path());

         fs::create_dir_all(dir.path()).unwrap();
         let mut f = File::create(&ja2json_path).unwrap();
         f.write("Test".as_bytes()).unwrap();

         cfg.ensure_existence().unwrap();

        let mut f = File::open(ja2json_path.clone()).unwrap();
        let mut content: Vec<u8> = vec!();
        f.read_to_end(&mut content).unwrap();

        assert!(ja2json_path.is_file());
        assert_eq!(content, b"Test");
    }

    #[test]
    fn parsing_json_config_should_fail_with_missing_file() {
        let dir = tempdir::TempDir::new("ja2-tests").unwrap();
        let cfg = JsonConfig::new(dir.path());

        assert_eq!(cfg.parse(), Err(String::from("Error reading ja2.json config file: entity not found")));
    }

    #[test]
    fn parsing_json_config_should_fail_with_invalid_json() {
        let dir = write_temp_folder_with_ja2_ini(b"{ not json }");
        let cfg = JsonConfig::new(dir.path());

        assert_eq!(cfg.parse(), Err(String::from("Error parsing ja2.json config file: key must be a string at line 1 column 3")));
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_set_stracciatella_home() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"stracciatella_home\": \"/aaa\" }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert_eq!(engine_options.stracciatella_home, PathBuf::from(""));
    }

    #[test]
    fn parse_json_config_should_be_able_to_change_data_dir() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"data_dir\": \"/dd\" }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert_eq!(engine_options.vanilla_data_dir, PathBuf::from("/dd"));
    }

    #[test]
    fn parse_json_config_should_be_able_to_change_fullscreen_value() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"fullscreen\": true }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert!(engine_options.start_in_fullscreen);
    }

    #[test]
    fn parse_json_config_should_be_able_to_change_debug_value() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"debug\": true }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert!(engine_options.start_in_debug_mode);
    }

    #[test]
    fn parse_json_config_should_be_able_to_start_without_sound() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"nosound\": true }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert!(engine_options.start_without_sound);
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_run_help() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"help\": true, \"show_help\": true }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert!(!engine_options.show_help);
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_run_unittests() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"unittests\": true, \"run_unittests\": true }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert!(!engine_options.run_unittests);
    }

    #[test]
    fn parse_json_config_should_not_be_able_to_run_editor() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"editor\": true, \"run_editor\": true }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert!(!engine_options.run_editor);
    }

    #[test]
    fn parse_json_config_should_not_be_able_start_in_window_explicitly() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"window\": true, \"start_in_window\": true }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert!(!engine_options.start_in_window);
    }

    #[test]
    fn parse_json_config_should_fail_with_invalid_mod() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"mods\": [ \"a\", true ] }");
        let cfg = JsonConfig::new(dir.path());

        assert_eq!(cfg.parse(), Err(String::from("Error parsing ja2.json config file: invalid type: boolean `true`, expected a string at line 1 column 21")));
    }

    #[test]
    fn parse_json_config_should_continue_with_multiple_known_switches() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"debug\": true, \"mods\": [ \"m1\", \"a2\" ] }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert!(engine_options.start_in_debug_mode);
        assert_eq!(engine_options.mods.len(), 2);
    }

    #[test]
    fn parse_json_config_should_fail_with_unknown_resversion() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"resversion\": \"TESTUNKNOWN\" }");
        let cfg = JsonConfig::new(dir.path());

        assert_eq!(cfg.parse(), Err(String::from("Error parsing ja2.json config file: unknown variant `TESTUNKNOWN`, expected one of `DUTCH`, `ENGLISH`, `FRENCH`, `GERMAN`, `ITALIAN`, `POLISH`, `RUSSIAN`, `RUSSIAN_GOLD` at line 1 column 29")));
    }

    #[test]
    fn parse_json_config_should_parse_resversion() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"resversion\": \"RUSSIAN\" }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert_eq!(engine_options.resource_version, ResourceVersion::RUSSIAN);
    }

    #[test]
    fn parse_json_config_should_return_the_correct_resolution() {
        let dir = write_temp_folder_with_ja2_ini(b"{ \"res\": \"1024x768\" }");
        let cfg = JsonConfig::new(dir.path());
        let engine_options = cfg.parse().unwrap();

        assert_eq!(engine_options.resolution, (1024, 768));
    }

    #[test]
    fn write_should_write_a_json_file_that_can_be_serialized_again() {
        let mut engine_options = super::EngineOptions::default();
        let dir = write_temp_folder_with_ja2_ini(b"");
        let cfg = JsonConfig::new(dir.path());

        engine_options.stracciatella_home = dir.path().to_path_buf();
        engine_options.resolution = (100, 100);

        cfg.write(&engine_options).unwrap();

        let got_engine_options = cfg.parse().unwrap();

        assert_eq!(got_engine_options.resolution, engine_options.resolution);
    }

    #[test]
    fn write_should_write_a_pretty_json_file() {
        let mut engine_options = super::EngineOptions::default();
        let dir = write_temp_folder_with_ja2_ini(b"Invalid JSON");
        let stracciatella_json = PathBuf::from(dir.path().join("ja2.json"));
        let cfg = JsonConfig::new(dir.path());

        engine_options.stracciatella_home = dir.path().to_path_buf();
        engine_options.resolution = (100, 100);

        cfg.write(&engine_options).unwrap();

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
}