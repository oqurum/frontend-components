use super::popup::button::{ButtonTitle, ButtonWithPopup};
use gloo_utils::window;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::UrlSearchParams;
use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Clone, PartialEq)]
pub struct FilterContainerContext<V: PartialEq + Clone + Copy + Default + 'static> {
    on_click_dropdown: Callback<ChildrenWithProps<FilterItemRedirect<V>>>,
}

#[derive(PartialEq, Properties)]
pub struct FilterContainerProps<V: PartialEq + Clone + Copy + Default + 'static> {
    pub children: Children,

    pub on_click: Callback<V>,

    #[prop_or_default]
    pub value: V,
}

#[function_component(FilterContainerComponent)]
pub fn _filter_comp<V: PartialEq + Clone + Copy + Default + 'static>(
    props: &FilterContainerProps<V>,
) -> Html {
    let display_drop_children =
        use_state_eq(|| Option::<ChildrenWithProps<FilterItemRedirect<V>>>::None);

    // TODO: Currently selected.

    let context = {
        let setter = display_drop_children.setter();

        FilterContainerContext {
            on_click_dropdown: Callback::from(move |items| {
                setter.set(Some(items));
            }),
        }
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
            title={ ButtonTitle::Text(AttrValue::Static("Asdf")) }
            {on_close_popup}
        >
            <ContextProvider<FilterContainerContext<V>> {context}>
                {
                    if let Some(children) = display_drop_children.as_ref() {
                        html! {
                            <>
                                <div class="menu-item" onclick={on_back}>
                                    { "Back" }
                                </div>

                                { for children.iter() }
                            </>
                        }
                    } else {
                        html! {{ for props.children.iter() }}
                    }
                }
            </ContextProvider<FilterContainerContext<V>>>
        </ButtonWithPopup>
    }
}

// ==========================
// Redirect
// ==========================

#[derive(PartialEq, Eq, Clone, Properties)]
pub struct FilterItemRedirectProps<V: PartialEq + Clone + Copy + Default + 'static> {
    pub title: String,

    /// Location Search (?query=value)
    pub search: (AttrValue, AttrValue),

    pub type_of: V,
}

#[function_component(FilterItemRedirect)]
pub fn _filter_item_redirect_comp<V: PartialEq + Clone + Copy + Default + 'static>(
    props: &FilterItemRedirectProps<V>,
) -> Html {
    let (query, value) = props.search.clone();

    let loc = window().location();

    let search = loc.search().unwrap_throw();
    let params = if !search.is_empty() {
        UrlSearchParams::new_with_str(&search[1..]).unwrap_throw()
    } else {
        UrlSearchParams::new().unwrap_throw()
    };

    params.set(&query, &value);

    html! {
        <div class="menu-item">
            <a href={ format!(
                "{}?{}",
                loc.pathname().unwrap_throw(),
                params.to_string().as_string().unwrap_throw()
            ) }>{ props.title.clone() }</a>
        </div>
    }
}

// ==========================
// Dropdown
// ==========================

#[derive(PartialEq, Clone, Properties)]
pub struct FilterItemDropdownProps<V: PartialEq + Clone + Copy + Default + 'static> {
    pub children: ChildrenWithProps<FilterItemRedirect<V>>,

    pub title: String,
}

#[function_component(FilterItemDropdown)]
pub fn _filter_item_dropdown_comp<V: PartialEq + Clone + Copy + Default + 'static>(
    props: &FilterItemDropdownProps<V>,
) -> Html {
    let state = use_context::<FilterContainerContext<V>>().unwrap_throw();

    let children = props.children.clone();

    html! {
        <div class="menu-item"
            onclick={ Callback::from(move |e: MouseEvent| {
                e.stop_propagation();

                state.on_click_dropdown.emit(children.clone());
            }) }
        >{ props.title.clone() }</div>
    }
}
