use std::path::PathBuf;

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
    extern crate regex;

    use super::*;
    use std::env;

    #[test]
    #[cfg(not(windows))]
    fn find_stracciatella_home_should_find_the_correct_stracciatella_home_path_on_unixlike() {
        let stracciatella_home = super::find_stracciatella_home().unwrap();

        assert_eq!(stracciatella_home, PathBuf::from(format!("{}/.ja2", env::var("HOME").unwrap())));
    }

    #[test]
    #[cfg(windows)]
    fn find_stracciatella_home_should_find_the_correct_stracciatella_home_path_on_windows() {
        use self::regex::Regex;

        let stracciatella_home = super::find_stracciatella_home().unwrap();
        let regex = Regex::new(r"^[A-Z]:\\(.*)+\\JA2").unwrap();

        assert!(regex.is_match(stracciatella_home.to_str().unwrap()), "{:?} is not a valid home dir for windows", stracciatella_home);
    }
}