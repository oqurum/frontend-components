use std::{
    fmt::{self, Display},
    num::ParseIntError,
    ops::Deref,
    str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "backend")]
use sqlx::{Decode, Encode, encode::IsNull, error::BoxDynError, database::{Database, HasValueRef, HasArguments}};

use crate::ImageType;

#[macro_use]
#[cfg(feature = "backend")]
mod macros {
    #[macro_export]
    macro_rules! create_single_id {
        ($name:ident) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type)]
            #[sqlx(transparent)]
            #[repr(transparent)]
            pub struct $name(i64);

            impl $name {
                pub fn none() -> Self {
                    Self(0)
                }

                pub fn is_none(self) -> bool {
                    self.0 == 0
                }
            }

            impl<'de> Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    Ok(Self(i64::deserialize(deserializer)?))
                }
            }

            impl Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    i64::serialize(&self.0, serializer)
                }
            }

            impl Deref for $name {
                type Target = i64;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    i64::fmt(&self.0, f)
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    Self::none()
                }
            }

            impl PartialEq<i64> for $name {
                fn eq(&self, other: &i64) -> bool {
                    self.0 == *other
                }
            }

            impl From<i64> for $name {
                fn from(value: i64) -> Self {
                    Self(value)
                }
            }

            impl FromStr for $name {
                type Err = ParseIntError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    i64::from_str(s).map(Self)
                }
            }
        };
    }
}

#[macro_use]
#[cfg(not(feature = "backend"))]
mod macros {
    #[macro_export]
    macro_rules! create_single_id {
        ($name:ident) => {
            #[repr(transparent)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name(i64);

            impl $name {
                pub fn none() -> Self {
                    Self(0)
                }

                pub fn is_none(self) -> bool {
                    self.0 == 0
                }
            }

            impl<'de> Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    Ok(Self(i64::deserialize(deserializer)?))
                }
            }

            impl Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    i64::serialize(&self.0, serializer)
                }
            }

            impl Deref for $name {
                type Target = i64;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    i64::fmt(&self.0, f)
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    Self::none()
                }
            }

            impl PartialEq<i64> for $name {
                fn eq(&self, other: &i64) -> bool {
                    self.0 == *other
                }
            }

            impl From<i64> for $name {
                fn from(value: i64) -> Self {
                    Self(value)
                }
            }

            impl FromStr for $name {
                type Err = ParseIntError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    i64::from_str(s).map(Self)
                }
            }
        };
    }
}

create_single_id!(BookPersonId);
create_single_id!(BookTagId);
create_single_id!(BookId);

create_single_id!(ImageId);

create_single_id!(MemberId);
create_single_id!(ClientId);

create_single_id!(PersonId);

create_single_id!(TagId);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageIdType {
    pub id: i64,
    pub type_of: ImageType, // We don't use the full u8. We use up 4 bits.
}

impl ImageIdType {
    pub fn new_book(value: BookId) -> Self {
        Self {
            id: *value,
            type_of: ImageType::Book,
        }
    }

    pub fn new_person(value: PersonId) -> Self {
        Self {
            id: *value,
            type_of: ImageType::Person,
        }
    }

    fn as_string(&self) -> String {
        format!("{}-{}", self.id, self.type_of.as_num())
    }
}

impl<'de> Deserialize<'de> for ImageIdType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::from_str(&value)
            .map_err(|_| serde::de::Error::custom("Unable to convert String to ImageIdType"))
    }
}

impl Serialize for ImageIdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&self.as_string(), serializer)
    }
}

#[cfg(feature = "backend")]
impl<'r, DB: Database> Decode<'r, DB> for ImageIdType
where
    &'r str: Decode<'r, DB>,
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        Ok(Self::from_str(<&str as Decode<DB>>::decode(value)?).unwrap())
    }
}

#[cfg(feature = "backend")]
impl<'q, DB: Database> Encode<'q, DB> for ImageIdType
where
    String: Encode<'q, DB>,
{
    fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        <&String as Encode<DB>>::encode(&self.as_string(), buf)
    }
}

impl Display for ImageIdType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_string().fmt(f)
    }
}

impl FromStr for ImageIdType {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Error handling.
        let split = s
            .split_once('-')
            .and_then(|(l, r)| Some((l.parse().ok()?, ImageType::from_number(r.parse().ok()?)?)));

        Ok(if let Some((id, type_of)) = split {
            Self { id, type_of }
        } else {
            // TODO: Remove.
            Self {
                id: 0,
                type_of: ImageType::Book,
            }
        })
    }
}
