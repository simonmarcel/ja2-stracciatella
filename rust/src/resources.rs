use std::fmt;
use std::str;

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

impl str::FromStr for ResourceVersion {
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

impl fmt::Display for ResourceVersion {
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
}
