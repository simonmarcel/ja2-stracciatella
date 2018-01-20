use std::path::PathBuf;

mod cli;
mod engine;
mod home;
mod json;

pub use self::home::find_stracciatella_home;
pub use self::cli::Cli;
pub use self::json::JsonConfig;
pub use self::cli::parse_resolution;
pub use self::engine::EngineOptions;

pub fn build_engine_options_from_env_and_args(args: Vec<String>) -> Result<EngineOptions, String> {
    let home_dir = find_stracciatella_home()?;
    let json = JsonConfig::new(&home_dir);
    let cli = Cli::new(args);

    json.ensure_existence()?;

    let mut engine_options = json.parse()?;
    engine_options.stracciatella_home = home_dir;
    cli.merge_options(&mut engine_options)?;

    if engine_options.vanilla_data_dir == PathBuf::from("") {
        return Err(String::from("Vanilla data directory has to be set either in config file or per command line switch"))
    }

    Ok(engine_options)
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use super::*;

    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    fn write_temp_folder_with_ja2_ini(contents: &[u8]) -> tempdir::TempDir {
        let dir = tempdir::TempDir::new("ja2-test").unwrap();
        fs::create_dir_all(dir.path().join(".ja2"));
        
        let mut f = File::create(dir.path().join(".ja2/ja2.json")).unwrap();
        f.write_all(contents).unwrap();
        f.sync_all().unwrap();

        dir
    }

    #[test]
    #[cfg(not(windows))]
    fn build_engine_options_from_env_and_args_should_overwrite_json_with_command_line_args() {
        let temp_dir = write_temp_folder_with_ja2_ini(b"{ \"data_dir\": \"/some/place/where/the/data/is\", \"res\": \"1024x768\", \"fullscreen\": true }");
        let args = vec!(String::from("ja2"), String::from("--res"), String::from("1100x480"));
        let old_home = env::var("HOME");

        env::set_var("HOME", temp_dir.path());
        let engine_options_res = build_engine_options_from_env_and_args(args);
        match old_home {
            Ok(home) => env::set_var("HOME", home),
            _ => {}
        }
        let engine_options = engine_options_res.unwrap();

        assert_eq!(engine_options.resolution, (1100, 480));
        assert!(engine_options.start_in_fullscreen);
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
}
