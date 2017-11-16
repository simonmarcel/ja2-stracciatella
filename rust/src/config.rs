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
