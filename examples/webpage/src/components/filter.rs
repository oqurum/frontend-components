use std::borrow::Cow;

use common::component::filter::*;
use gloo_utils::window;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::UrlSearchParams;
use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum FilterItem {
    #[default]
    Abc,
    OneTwoThree,
    Baby,
}

impl FilterItem {
    pub fn get_from_params_or_default(params: &UrlSearchParams) -> Self {
        if params.has("abc") {
            Self::Abc
        } else if params.has("123") {
            Self::OneTwoThree
        } else if params.has("baby") {
            Self::Baby
        } else {
            Self::default()
        }
    }
}

impl FilterItemType for FilterItem {
    fn title(self) -> Cow<'static, str> {
        match self {
            FilterItem::Abc => Cow::Borrowed("Testing Abc Item"),
            FilterItem::OneTwoThree => Cow::Borrowed("Testing OneTwoThree Item"),
            FilterItem::Baby => Cow::Borrowed("Testing Baby Item"),
        }
    }
}

#[function_component(FilterPage)]
pub fn _filter_cont() -> Html {
    let comp_value = use_state_eq(|| {
        let loc = window().location();

        let search = loc.search().unwrap_throw();

        let params = if search.is_empty() {
            UrlSearchParams::new().unwrap_throw()
        } else {
            UrlSearchParams::new_with_str(&search[1..]).unwrap_throw()
        };

        FilterItem::get_from_params_or_default(&params)
    });

    let comp_value_set = comp_value.setter();

    html! {
        <div style="max-width: 300px">
            <h3>{ "Filter Dropdown" }</h3>
            <br />

            <FilterContainerComponent<FilterItem>
                overwrite_query=true
                value={ *comp_value }
                on_click={ Callback::from(move |v| comp_value_set.set(v)) }
            >
                <FilterItemDropdown<FilterItem> title="ABC">
                    <FilterItemRedirect<FilterItem> type_of={ FilterItem::Abc } search={ (AttrValue::Static("abc"), AttrValue::Static("def")) } />
                </FilterItemDropdown<FilterItem>>

                <FilterItemDropdown<FilterItem> title="123">
                    <FilterItemRedirect<FilterItem> type_of={ FilterItem::OneTwoThree } search={ (AttrValue::Static("123"), AttrValue::Static("456")) } />
                </FilterItemDropdown<FilterItem>>

                <FilterItemRedirect<FilterItem> type_of={ FilterItem::Baby } search={ (AttrValue::Static("baby"), AttrValue::Static("You and Me")) } />
            </FilterContainerComponent<FilterItem>>
        </div>
    }
}
