use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub static MISSING_THUMB_PATH: &str = "/images/missingthumbnail.jpg";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageType {
    Book = 0,
    Person,
}

impl ImageType {
    pub fn as_num(self) -> u8 {
        self as u8
    }

    pub fn from_number(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::Book,
            1 => Self::Person,

            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThumbnailStore {
    Path(String),
    None,
}

impl ThumbnailStore {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn as_value(&self) -> Option<&str> {
        match self {
            Self::Path(v) => Some(v.as_str()),
            Self::None => None,
        }
    }

    pub fn into_value(self) -> Option<String> {
        match self {
            Self::Path(v) => Some(v),
            Self::None => None,
        }
    }
}

impl From<&str> for ThumbnailStore {
    fn from(value: &str) -> Self {
        Self::Path(value.to_string())
    }
}

impl From<String> for ThumbnailStore {
    fn from(value: String) -> Self {
        Self::Path(value)
    }
}

impl From<Option<String>> for ThumbnailStore {
    fn from(value: Option<String>) -> Self {
        value.map(|v| v.into()).unwrap_or(Self::None)
    }
}

impl From<Option<&str>> for ThumbnailStore {
    fn from(value: Option<&str>) -> Self {
        value.map(|v| v.into()).unwrap_or(Self::None)
    }
}

impl<'de> Deserialize<'de> for ThumbnailStore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Option::<String>::deserialize(deserializer)?.into())
    }
}

impl Serialize for ThumbnailStore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_value().serialize(serializer)
    }
}
