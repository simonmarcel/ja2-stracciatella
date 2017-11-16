use std::path::PathBuf;
use std::str::FromStr;
use fs;
use getopts::Options;

use engine::EngineOptions;
use resources::ResourceVersion;

#[cfg(not(windows))]
static DATA_DIR_OPTION_EXAMPLE: &'static str = "/opt/ja2";
#[cfg(windows)]
static DATA_DIR_OPTION_EXAMPLE: &'static str = "C:\\JA2";

pub fn parse_resolution(resolution_str: &str) -> Result<(u16, u16), String> {
    let mut resolutions = resolution_str.split("x").filter_map(|r_str| r_str.parse::<u16>().ok());

    match (resolutions.next(), resolutions.next()) {
        (Some(x), Some(y)) => Ok((x, y)),
        _ => Err(String::from("Incorrect resolution format, should be WIDTHxHEIGHT."))
    }
}

pub fn get_command_line_options() -> Options {
    let mut opts = Options::new();

    opts.long_only(true);

    opts.optmulti(
        "",
        "datadir",
        "Set path for data directory",
        DATA_DIR_OPTION_EXAMPLE
    );
    opts.optmulti(
        "",
        "mod",
        "Start one of the game modifications. MOD_NAME is the name of modification, e.g. 'from-russia-with-love. See mods folder for possible options'.",
        "MOD_NAME"
    );
    opts.optopt(
        "",
        "res",
        "Screen resolution, e.g. 800x600. Default value is 640x480",
        "WIDTHxHEIGHT"
    );
    opts.optopt(
        "",
        "resversion",
        "Version of the game resources. Possible values: DUTCH, ENGLISH, FRENCH, GERMAN, ITALIAN, POLISH, RUSSIAN, RUSSIAN_GOLD. Default value is ENGLISH. RUSSIAN is for BUKA Agonia Vlasty release. RUSSIAN_GOLD is for Gold release",
        "RUSSIAN_GOLD"
    );
    opts.optflag(
        "",
        "unittests",
        "Perform unit tests. E.g. 'ja2.exe -unittests --gtest_output=\"xml:report.xml\" --gtest_repeat=2'");
    opts.optflag(
        "",
        "editor",
        "Start the map editor (Editor.slf is required)"
    );
    opts.optflag(
        "",
        "fullscreen",
        "Start the game in the fullscreen mode"
    );
    opts.optflag(
        "",
        "nosound",
        "Turn the sound and music off"
    );
    opts.optflag(
        "",
        "window",
        "Start the game in a window"
    );
    opts.optflag(
        "",
        "debug",
        "Enable Debug Mode"
    );
    opts.optflag(
        "",
        "help",
        "print this help menu"
    );

    return opts;
}

pub fn parse_args(engine_options: &mut EngineOptions, args: Vec<String>) -> Option<String> {
    let opts = get_command_line_options();

    match opts.parse(&args[1..]) {
        Ok(m) => {
            if m.free.len() > 0 {
                return Some(format!("Unknown arguments: '{}'.", m.free.join(" ")));
            }

            if let Some(s) = m.opt_str("datadir") {
                match fs::canonicalize(PathBuf::from(s)) {
                    Ok(s) => {
                        let mut temp = String::from(s.to_str().expect("Should not happen"));
                        // remove UNC path prefix (Windows)
                        if temp.starts_with("\\\\") {
                            temp.drain(..2);
                            let pos = temp.find("\\").unwrap() + 1;
                            temp.drain(..pos);
                        }
                        engine_options.vanilla_data_dir = PathBuf::from(temp)
                    },
                    Err(_) => return Some(String::from("Please specify an existing datadir."))
                };
            }

            if m.opt_strs("mod").len() > 0 {
                engine_options.mods = m.opt_strs("mod");
            }

            if let Some(s) = m.opt_str("res") {
                match parse_resolution(&s) {
                    Ok(res) => {
                        engine_options.resolution = res;
                    },
                    Err(s) => return Some(s)
                }
            }

            if let Some(s) = m.opt_str("resversion") {
                match ResourceVersion::from_str(&s) {
                    Ok(resource_version) => {
                        engine_options.resource_version = resource_version
                    },
                    Err(str) => return Some(str)
                }
            }

            if m.opt_present("help") {
                engine_options.show_help = true;
            }


            if m.opt_present("unittests") {
                engine_options.run_unittests = true;
            }

            if m.opt_present("editor") {
                engine_options.run_editor = true;
            }

            if m.opt_present("fullscreen") {
                engine_options.start_in_fullscreen = true;
            }

            if m.opt_present("nosound") {
                engine_options.start_without_sound = true;
            }

            if m.opt_present("window") {
                engine_options.start_in_window = true;
            }

            if m.opt_present("debug") {
                engine_options.start_in_debug_mode = true;
            }

            return None;
        }
        Err(f) => Some(f.to_string())
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;
    use fs;

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
        assert!(!engine_options.start_in_fullscreen);
    }

    #[test]
    fn parse_args_should_be_able_to_change_fullscreen_value() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("-fullscreen"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert!(engine_options.start_in_fullscreen);
    }

    #[test]
    fn parse_args_should_be_able_to_show_help() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("-help"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert!(engine_options.show_help);
    }

    #[test]
    fn parse_args_should_continue_with_multiple_known_switches() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("-debug"), String::from("-mod"), String::from("a"), String::from("--mod"), String::from("ö"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert!(engine_options.start_in_debug_mode);
        assert_eq!(engine_options.mods, vec!["a", "ö"]);
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
        assert_eq!(engine_options.resource_version, super::ResourceVersion::RUSSIAN);
    }

    #[test]
    fn parse_args_should_return_the_correct_resversion_for_italian() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("-resversion"), String::from("ITALIAN"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert_eq!(engine_options.resource_version, super::ResourceVersion::ITALIAN);
    }

    #[test]
    fn parse_args_should_return_the_correct_resolution() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("--res"), String::from("1120x960"));
        assert_eq!(super::parse_args(&mut engine_options, input), None);
        assert_eq!(engine_options.resolution, (1120, 960));
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

        let temp = fs::canonicalize(temp_dir.path()).expect("Problem during building of reference value.");

        assert_eq!(engine_options.vanilla_data_dir, temp);
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
        assert_eq!(engine_options.vanilla_data_dir, temp_dir.path());
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
        assert_eq!(engine_options.vanilla_data_dir, temp_dir.path());
    }

    #[test]
    fn parse_args_should_fail_with_non_existing_directory() {
        let mut engine_options: super::EngineOptions = Default::default();
        let input = vec!(String::from("ja2"), String::from("--datadir"), String::from("somethingelse"));

        assert_eq!(super::parse_args(&mut engine_options, input), Some(String::from("Please specify an existing datadir.")));
    }
}
