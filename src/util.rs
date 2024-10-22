use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Truncate string based off of char indices instead of bytes.
pub fn truncate_on_indices(s: &mut String, max_chars: usize) {
    if let Some((new_len, _)) = s.char_indices().nth(max_chars) {
        s.truncate(new_len);
    }
}

pub fn upper_case_first_char(mut value: String) -> String {
    // Get the first char
    if let Some(v) = value.chars().next() {
        // Uppercase first char
        let first = v.to_uppercase().to_string();

        // Replace first char with uppercase one.
        value.replace_range(0..v.len_utf8(), &first);
    }

    value
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoadingItem<V> {
    Loading,
    Loaded(V),
}

#[cfg(feature = "frontend")]
pub use frontend::*;

#[cfg(feature = "frontend")]
mod frontend {
    use web_sys::Element;

    pub fn does_parent_contain_class(element: &Element, value: &str) -> bool {
        if element.class_list().contains(value) {
            true
        } else if let Some(element) = element.parent_element() {
            does_parent_contain_class(&element, value)
        } else {
            false
        }
    }

    pub fn does_parent_contain_attribute(element: &Element, value: &str) -> bool {
        if element.has_attribute(value) {
            true
        } else if let Some(element) = element.parent_element() {
            does_parent_contain_attribute(&element, value)
        } else {
            false
        }
    }
}

// DateTime

pub fn serialize_datetime<S>(value: &DateTime<Utc>, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_i64(value.timestamp_millis())
}

pub fn serialize_datetime_opt<S>(
    value: &Option<DateTime<Utc>>,
    s: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => s.serialize_i64(v.timestamp_millis()),
        None => s.serialize_none(),
    }
}

pub fn serialize_datetime_opt_opt<S>(
    value: &Option<Option<DateTime<Utc>>>,
    s: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.map(|v| v.map(|v| v.timestamp_millis())).serialize(s)
}

pub fn deserialize_datetime<'de, D>(value: D) -> std::result::Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Utc.timestamp_millis_opt(i64::deserialize(value)?).unwrap())
}

pub fn deserialize_datetime_opt<'de, D>(
    value: D,
) -> std::result::Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Some(v) = Option::<i64>::deserialize(value)? {
        Ok(Some(Utc.timestamp_millis_opt(v).unwrap()))
    } else {
        Ok(None)
    }
}

pub fn deserialize_datetime_opt_opt<'de, D>(
    value: D,
) -> std::result::Result<Option<Option<DateTime<Utc>>>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Some(v) = Option::<Option<i64>>::deserialize(value)? {
        Ok(Some(v.map(|v| Utc.timestamp_millis_opt(v).unwrap())))
    } else {
        Ok(None)
    }
}

// Date

pub fn serialize_date<S>(value: &NaiveDate, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_i64(value.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp())
}

pub fn serialize_naivedate_opt<S>(
    value: &Option<NaiveDate>,
    s: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => s.serialize_i64(v.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp()),
        None => s.serialize_none(),
    }
}

pub fn deserialize_date<'de, D>(value: D) -> std::result::Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Utc
        .timestamp_opt(i64::deserialize(value)?, 0)
        .unwrap()
        .date_naive())
}

pub fn deserialize_naivedate_opt<'de, D>(
    value: D,
) -> std::result::Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Some(v) = Option::<i64>::deserialize(value)? {
        Ok(Some(Utc.timestamp_opt(v, 0).unwrap().date_naive()))
    } else {
        Ok(None)
    }
}
