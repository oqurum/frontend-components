use std::{fmt, borrow::Cow, ops::Deref};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::Error;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Agent(Cow<'static, str>);

impl Agent {
    pub fn new_static(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }

    pub fn new_owned(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl Deref for Agent {
    type Target = Cow<'static, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub agent: Agent,
    pub value: String,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.agent, self.value)
    }
}

impl TryFrom<&str> for Source {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (source, value) = value.split_once(':').ok_or(Error::SourceSplit)?;

        Ok(Self {
            agent: Agent(Cow::Owned(source.to_owned())),
            value: value.to_owned(),
        })
    }
}

impl TryFrom<String> for Source {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (source, value) = value.split_once(':').ok_or(Error::SourceSplit)?;

        Ok(Self {
            agent: Agent(Cow::Owned(source.to_owned())),
            value: value.to_owned(),
        })
    }
}

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let resp = String::deserialize(deserializer)?;
        Self::try_from(resp)
            .map_err(|_| serde::de::Error::custom("Unable to convert String to Source"))
    }
}

impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
