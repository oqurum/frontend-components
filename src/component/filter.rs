use std::{borrow::Cow, rc::Rc};

use super::popup::button::{ButtonTitle, ButtonWithPopup};
use gloo_utils::window;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::UrlSearchParams;
use yew::{prelude::*, virtual_dom::AttrValue};

pub trait FilterItemType {
    fn title(self) -> Cow<'static, str>;
}

#[derive(Clone, PartialEq)]
pub struct FilterContainerContext<V: PartialEq + Clone + Copy + Default + FilterItemType + 'static>
{
    on_click_dropdown: Callback<ChildrenWithProps<FilterItemRedirect<V>>>,

    overwrite_query: bool,
}

#[derive(PartialEq, Properties)]
pub struct FilterContainerProps<V: PartialEq + Clone + Copy + Default + FilterItemType + 'static> {
    pub children: Children,

    pub on_click: Callback<V>,

    pub value: V,

    pub overwrite_query: bool,
}

#[function_component(FilterContainerComponent)]
pub fn _filter_comp<V: PartialEq + Clone + Copy + Default + FilterItemType + 'static>(
    props: &FilterContainerProps<V>,
) -> Html {
    // keeps track of what you have selected instead of relying on property.
    let inner_selection = {
        let selected = props.value;
        use_state_eq(move || selected)
    };

    let display_drop_children =
        use_state_eq(|| Option::<ChildrenWithProps<FilterItemRedirect<V>>>::None);

    let context = {
        let setter_dropdown = display_drop_children.setter();

        Rc::new(FilterContainerContext {
            overwrite_query: props.overwrite_query,

            on_click_dropdown: Callback::from(move |items| {
                setter_dropdown.set(Some(items));
            }),
        })
    };

    let on_close_popup = {
        let setter = display_drop_children.setter();
        Callback::from(move |_| setter.set(None))
    };

    let on_back = {
        let setter = display_drop_children.setter();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            setter.set(None);
        })
    };

    html! {
        <ButtonWithPopup
            title={ ButtonTitle::Text(AttrValue::from(inner_selection.title())) }
            {on_close_popup}
        >
            <ContextProvider<Rc<FilterContainerContext<V>>> {context}>
                {
                    if let Some(children) = display_drop_children.as_ref() {
                        html! {
                            <>
                                <div class="dropdown-item" onclick={on_back}>
                                    { "Back" }
                                </div>

                                { for children.iter() }
                            </>
                        }
                    } else {
                        html! {{ for props.children.iter() }}
                    }
                }
            </ContextProvider<Rc<FilterContainerContext<V>>>>
        </ButtonWithPopup>
    }
}

// ==========================
// Redirect
// ==========================

#[derive(PartialEq, Eq, Clone, Properties)]
pub struct FilterItemRedirectProps<V: PartialEq + Clone + Copy + Default + FilterItemType + 'static>
{
    // TODO: Improve. Should be able to have multiple queries
    /// Location Search (?query=value)
    pub search: (AttrValue, AttrValue),

    pub type_of: V,
}

#[function_component(FilterItemRedirect)]
pub fn _filter_item_redirect_comp<
    V: PartialEq + Clone + Copy + Default + FilterItemType + 'static,
>(
    props: &FilterItemRedirectProps<V>,
) -> Html {
    let state = use_context::<Rc<FilterContainerContext<V>>>().unwrap_throw();

    let (query, value) = props.search.clone();

    let loc = window().location();

    let search = loc.search().unwrap_throw();
    let params = if state.overwrite_query || search.is_empty() {
        UrlSearchParams::new().unwrap_throw()
    } else {
        UrlSearchParams::new_with_str(&search[1..]).unwrap_throw()
    };

    params.set(&query, &value);

    html! {
        <div class="dropdown-item">
            <a href={ format!(
                "{}?{}",
                loc.pathname().unwrap_throw(),
                params.to_string().as_string().unwrap_throw()
            ) }>{ props.type_of.title() }</a>
        </div>
    }
}

// ==========================
// Dropdown
// ==========================

#[derive(PartialEq, Clone, Properties)]
pub struct FilterItemDropdownProps<V: PartialEq + Clone + Copy + Default + FilterItemType + 'static>
{
    pub children: ChildrenWithProps<FilterItemRedirect<V>>,

    pub title: String,
}

#[function_component(FilterItemDropdown)]
pub fn _filter_item_dropdown_comp<
    V: PartialEq + Clone + Copy + Default + FilterItemType + 'static,
>(
    props: &FilterItemDropdownProps<V>,
) -> Html {
    let state = use_context::<Rc<FilterContainerContext<V>>>().unwrap_throw();

    let children = props.children.clone();

    html! {
        <div class="dropdown-item"
            onclick={ Callback::from(move |e: MouseEvent| {
                e.stop_propagation();

                state.on_click_dropdown.emit(children.clone());
            }) }
        >{ props.title.clone() }</div>
    }
}
