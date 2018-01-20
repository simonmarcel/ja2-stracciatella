extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate getopts;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate user32;
#[cfg(windows)]
extern crate shell32;


pub mod config;
pub mod resources;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
