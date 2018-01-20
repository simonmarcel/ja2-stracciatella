use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use getopts::Options;

use config::engine::EngineOptions;
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

pub struct Cli {
    args: Vec<String>
}

impl Cli {
    pub fn new(args: Vec<String>) -> Cli {
        Cli { args: args }
    }

    pub fn options() -> Options {
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

    pub fn merge_options(self: &Cli, engine_options: &mut EngineOptions) -> Result<(), String> {
        let opts = Cli::options();
        let parsed = opts.parse(&self.args[1..]).map_err(|e| e.to_string())?;

        if parsed.free.len() > 0 {
            return Err(format!("Unknown arguments: '{}'.", parsed.free.join(" ")));
        }

        if parsed.opt_present("fullscreen") && parsed.opt_present("window") {
            return Err(String::from("Cannot use fullscreen and window switches at the same time."));
        }

        if let Some(s) = parsed.opt_str("datadir") {
            let datadir = fs::canonicalize(PathBuf::from(s)).map_err(|_| String::from("Please specify an existing datadir."))?;
            let mut temp = String::from(datadir.to_str().expect("Error converting PathBuf to str when parsing cli datadir"));
            // remove UNC path prefix (Windows)
            if temp.starts_with("\\\\") {
                temp.drain(..2);
                let pos = temp.find("\\").unwrap() + 1;
                temp.drain(..pos);
            }
            engine_options.vanilla_data_dir = PathBuf::from(temp)
        }

        if parsed.opt_strs("mod").len() > 0 {
            engine_options.mods = parsed.opt_strs("mod");
        }

        if let Some(ref s) = parsed.opt_str("res") {
            engine_options.resolution = parse_resolution(&s)?;
        }

        if let Some(ref s) = parsed.opt_str("resversion") {
            engine_options.resource_version = ResourceVersion::from_str(&s)?;
        }

        if parsed.opt_present("help") {
            engine_options.show_help = true;
        }

        if parsed.opt_present("unittests") {
            engine_options.run_unittests = true;
        }

        if parsed.opt_present("editor") {
            engine_options.run_editor = true;
        }

        if parsed.opt_present("fullscreen") {
            engine_options.start_in_fullscreen = true;
        }

        if parsed.opt_present("window") {
            engine_options.start_in_fullscreen = false;
        }

        if parsed.opt_present("nosound") {
            engine_options.start_without_sound = true;
        }

        if parsed.opt_present("debug") {
            engine_options.start_in_debug_mode = true;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use super::*;

    #[test]
    fn it_should_work_without_args() {
        Cli::new(vec!());
    }

    #[test]
    fn it_should_do_nothing_without_args() {
        let mut got = EngineOptions::default();
        let expected = got.clone();
        let cli = Cli::new(vec!(String::from("ja2")));

        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    #[cfg(not(windows))]
    fn it_should_parse_datadir_option_to_canonical_data_dir_unix() {
        let temp_dir = tempdir::TempDir::new("ja2-tests").unwrap();
        let dir_path = temp_dir.path().join("foo");

        fs::create_dir_all(&dir_path).unwrap();

        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--datadir"), String::from(temp_dir.path().join("foo/../foo/../").to_str().unwrap()));
        let cli = Cli::new(input);

        expected.vanilla_data_dir = fs::canonicalize(temp_dir.path()).expect("Problem during building of reference value.");
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    #[cfg(windows)]
    fn it_should_parse_datadir_option_to_canonical_data_dir_windows() {
        let temp_dir = tempdir::TempDir::new("ja2-tests").unwrap();
        let dir_path = temp_dir.path().join("foo");

        fs::create_dir_all(dir_path).unwrap();

        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--datadir"), String::from(temp_dir.path().join("foo\\..\\foo\\..\\").to_str().unwrap()));
        let cli = Cli::new(input);

        expected.vanilla_data_dir = PathBuf::from(temp_dir.path());
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn it_should_parse_mods() {
        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--mod"), String::from("a"), String::from("--mod"), String::from("b"));
        let cli = Cli::new(input);

        expected.mods = vec!(String::from("a"), String::from("b"));
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn it_should_return_an_error_on_parsing_invalid_resolution() {
        let mut got = EngineOptions::default();
        let input = vec!(String::from("ja2"), String::from("--res"), String::from("a"));
        let cli = Cli::new(input);

        assert_eq!(cli.merge_options(&mut got), Err(String::from("Incorrect resolution format, should be WIDTHxHEIGHT.")));
    }

    #[test]
    fn it_should_parse_resolution() {
        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--res"), String::from("120x120"));
        let cli = Cli::new(input);

        expected.resolution = (120, 120);
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn it_should_return_an_error_on_parsing_invalid_resversion() {
        let mut got = EngineOptions::default();
        let input = vec!(String::from("ja2"), String::from("--resversion"), String::from("a"));
        let cli = Cli::new(input);

        assert_eq!(cli.merge_options(&mut got), Err(String::from("Resource version a is unknown")));
    }

    #[test]
    fn it_should_parse_resversion() {
        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--resversion"), String::from("RUSSIAN_GOLD"));
        let cli = Cli::new(input);

        expected.resource_version = ResourceVersion::RUSSIAN_GOLD;
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn it_should_parse_help() {
        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--help"));
        let cli = Cli::new(input);

        expected.show_help = true;
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn it_should_parse_unittests() {
        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--unittests"));
        let cli = Cli::new(input);

        expected.run_unittests = true;
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn it_should_parse_editor() {
        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--editor"));
        let cli = Cli::new(input);

        expected.run_editor = true;
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn it_should_parse_fullscreen() {
        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--fullscreen"));
        let cli = Cli::new(input);

        expected.start_in_fullscreen = true;
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn it_should_parse_window() {
        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--window"));
        let cli = Cli::new(input);

        got.start_in_fullscreen = true;
        expected.start_in_fullscreen = false;
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn it_should_throw_on_window_and_fullscreen() {
        let mut got = EngineOptions::default();
        let input = vec!(String::from("ja2"), String::from("--window"), String::from("--fullscreen"));
        let cli = Cli::new(input);

        assert_eq!(cli.merge_options(&mut got), Err(String::from("Cannot use fullscreen and window switches at the same time.")));
    }

    #[test]
    fn it_should_parse_nosound() {
        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--nosound"));
        let cli = Cli::new(input);

        expected.start_without_sound = true;
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn it_should_parse_debug() {
        let mut got = EngineOptions::default();
        let mut expected = got.clone();
        let input = vec!(String::from("ja2"), String::from("--debug"));
        let cli = Cli::new(input);

        expected.start_in_debug_mode = true;
        cli.merge_options(&mut got).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn it_should_throw_on_unknown_args() {
        let mut got = EngineOptions::default();
        let input = vec!(String::from("ja2"), String::from("aaa"));
        let cli = Cli::new(input);

        assert_eq!(cli.merge_options(&mut got), Err(String::from("Unknown arguments: 'aaa'.")));
    }
}