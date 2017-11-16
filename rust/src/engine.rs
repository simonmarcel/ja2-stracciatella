use std::path::PathBuf;
use serde;
use serde::Deserializer;
use serde::Deserialize;
use serde::Serializer;
use serde::Serialize;

use cli::parse_resolution;
use resources::ResourceVersion;


fn deserialize_resolution<'de, D>(deserializer: D) -> Result<(u16, u16), D::Error>
where
    D: Deserializer<'de>,
{
    let res = String::deserialize(deserializer)?;
    parse_resolution(&res).map_err(|s| serde::de::Error::custom(s))
}

fn serialize_resolution<S>(&(x, y): &(u16, u16), serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    String::serialize(&format!("{}x{}", x, y), serializer)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EngineOptions {
    #[serde(skip)]
    pub stracciatella_home: PathBuf,
    #[serde(rename = "data_dir")]
    pub vanilla_data_dir: PathBuf,
    pub mods: Vec<String>,
    #[serde(rename ="res", serialize_with = "serialize_resolution", deserialize_with = "deserialize_resolution")]
    pub resolution: (u16, u16),
    #[serde(rename = "resversion")]
    pub resource_version: ResourceVersion,
    #[serde(skip)]
    pub show_help: bool,
    #[serde(skip)]
    pub run_unittests: bool,
    #[serde(skip)]
    pub run_editor: bool,
    #[serde(rename = "fullscreen")]
    pub start_in_fullscreen: bool,
    #[serde(skip)]
    pub start_in_window: bool,
    #[serde(rename = "debug")]
    pub start_in_debug_mode: bool,
    #[serde(rename = "nosound")]
    pub start_without_sound: bool,
}

impl Default for EngineOptions {
    fn default() -> EngineOptions {
        EngineOptions {
            stracciatella_home: PathBuf::from(""),
            vanilla_data_dir: PathBuf::from(""),
            mods: vec!(),
            resolution: (640, 480),
            resource_version: ResourceVersion::ENGLISH,
            show_help: false,
            run_unittests: false,
            run_editor: false,
            start_in_fullscreen: false,
            start_in_window: true,
            start_in_debug_mode: false,
            start_without_sound: false,
        }
    }
}
