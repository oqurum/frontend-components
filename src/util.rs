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