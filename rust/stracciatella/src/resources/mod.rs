use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum ResourceVersion {
    DUTCH,
    ENGLISH,
    FRENCH,
    GERMAN,
    ITALIAN,
    POLISH,
    RUSSIAN,
    RUSSIAN_GOLD,
}

impl FromStr for ResourceVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DUTCH" => Ok(ResourceVersion::DUTCH),
            "ENGLISH" => Ok(ResourceVersion::ENGLISH),
            "FRENCH" => Ok(ResourceVersion::FRENCH),
            "GERMAN" => Ok(ResourceVersion::GERMAN),
            "ITALIAN" => Ok(ResourceVersion::ITALIAN),
            "POLISH" => Ok(ResourceVersion::POLISH),
            "RUSSIAN" => Ok(ResourceVersion::RUSSIAN),
            "RUSSIAN_GOLD" => Ok(ResourceVersion::RUSSIAN_GOLD),
            _ => Err(format!("Resource version {} is unknown", s))
        }
    }
}

impl Display for ResourceVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &ResourceVersion::DUTCH => "DUTCH",
            &ResourceVersion::ENGLISH => "ENGLISH",
            &ResourceVersion::FRENCH => "FRENCH",
            &ResourceVersion::GERMAN => "GERMAN",
            &ResourceVersion::ITALIAN => "ITALIAN",
            &ResourceVersion::POLISH => "POLISH",
            &ResourceVersion::RUSSIAN => "RUSSIAN",
            &ResourceVersion::RUSSIAN_GOLD => "RUSSIAN_GOLD",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_resource_version_from_string_correctly() {
        assert_eq!(ResourceVersion::from_str("bla"), Err(String::from("Resource version bla is unknown")));
        assert_eq!(ResourceVersion::from_str("DUTCH"), Ok(ResourceVersion::DUTCH));
        assert_eq!(ResourceVersion::from_str("ENGLISH"), Ok(ResourceVersion::ENGLISH));
        assert_eq!(ResourceVersion::from_str("FRENCH"), Ok(ResourceVersion::FRENCH));
        assert_eq!(ResourceVersion::from_str("GERMAN"), Ok(ResourceVersion::GERMAN));
        assert_eq!(ResourceVersion::from_str("ITALIAN"), Ok(ResourceVersion::ITALIAN));
        assert_eq!(ResourceVersion::from_str("POLISH"), Ok(ResourceVersion::POLISH));
        assert_eq!(ResourceVersion::from_str("RUSSIAN"), Ok(ResourceVersion::RUSSIAN));
        assert_eq!(ResourceVersion::from_str("RUSSIAN_GOLD"), Ok(ResourceVersion::RUSSIAN_GOLD));
    }

    #[test]
    fn it_displays_resource_version_correctly() {
        assert_eq!(format!("{}", ResourceVersion::DUTCH), "DUTCH");
        assert_eq!(format!("{}", ResourceVersion::ENGLISH), "ENGLISH");
        assert_eq!(format!("{}", ResourceVersion::FRENCH), "FRENCH");
        assert_eq!(format!("{}", ResourceVersion::GERMAN), "GERMAN");
        assert_eq!(format!("{}", ResourceVersion::ITALIAN), "ITALIAN");
        assert_eq!(format!("{}", ResourceVersion::POLISH), "POLISH");
        assert_eq!(format!("{}", ResourceVersion::RUSSIAN), "RUSSIAN");
        assert_eq!(format!("{}", ResourceVersion::RUSSIAN_GOLD), "RUSSIAN_GOLD");
    }
}
