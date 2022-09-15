pub static LANGUAGES: [&str; 1] = ["english"];

pub fn get_language_id(value: &str) -> u16 {
    let value = value.to_lowercase();
    LANGUAGES.iter().position(|v| *v == value).unwrap_or_default() as u16
}

pub fn get_language_name(value: u16) -> &'static str {
    LANGUAGES
        .get(value as usize)
        .map(|v| *v)
        .unwrap_or(LANGUAGES[0])
}
