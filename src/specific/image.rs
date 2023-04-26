use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "backend")]
use sqlx::{Decode, Encode, encode::IsNull, error::BoxDynError, database::{Database, HasValueRef, HasArguments}};

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

impl From<i64> for ImageType {
    fn from(value: i64) -> Self {
        Self::from_number(value as u8).unwrap()
    }
}


#[cfg(feature = "backend")]
impl<'q, DB: Database> Encode<'q, DB> for ImageType
where
    u8: Encode<'q, DB>,
{
    fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        <&u8 as Encode<DB>>::encode(&self.as_num(), buf)
    }
}

#[cfg(feature = "backend")]
impl<'r, DB: Database> Decode<'r, DB> for ImageType
where
    u8: Decode<'r, DB>,
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        Ok(Self::from_number(<u8 as Decode<DB>>::decode(value)?).unwrap())
    }
}

#[cfg(feature = "backend")]
impl<DB: Database> sqlx::Type<DB> for ImageType
where
    u8: sqlx::Type<DB>
{
    fn type_info() -> DB::TypeInfo {
        <u8 as sqlx::Type<DB>>::type_info()
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

#[cfg(feature = "backend")]
impl<'q, DB: Database> Encode<'q, DB> for ThumbnailStore
where
    Option<String>: Encode<'q, DB>,
{
    fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        <&Option<String> as Encode<DB>>::encode(&self.as_value().map(|v| v.to_string()), buf)
    }
}

#[cfg(feature = "backend")]
impl<'r, DB: Database> Decode<'r, DB> for ThumbnailStore
where
    Option<String>: Decode<'r, DB>,
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        Ok(Self::from(<Option<String> as Decode<DB>>::decode(value)?))
    }
}

#[cfg(feature = "backend")]
impl<DB: Database> sqlx::Type<DB> for ThumbnailStore
where
    Option<String>: sqlx::Type<DB>
{
    fn type_info() -> DB::TypeInfo {
        <Option<String> as sqlx::Type<DB>>::type_info()
    }
}