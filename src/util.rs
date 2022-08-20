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
