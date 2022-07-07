use crate::util::upper_case_first_char;




pub static LANGUAGES: [&str; 1] = [
    "english",
];



pub fn get_language_id(value: &str) -> Option<usize> {
    let value = value.to_lowercase();
    LANGUAGES.iter().position(|v| *v == value)
}


pub fn get_language_name(value: u16) -> Option<String> {
    LANGUAGES.get(value as usize).map(|v| upper_case_first_char(v.to_string()))
}