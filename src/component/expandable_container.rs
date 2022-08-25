use std::fmt::Write;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::use_bool_toggle;

#[derive(PartialEq, Properties)]
pub struct ExpandableContainerProps {
    pub children: Children,

    pub max_lines: Option<usize>,
}

#[function_component(ExpandableContainerComponent)]
pub fn _expanding_text_comp(props: &ExpandableContainerProps) -> Html {
    let container_ref = use_node_ref();
    let show_expand_button = use_bool_toggle(false);
    let is_expanded = use_bool_toggle(false);

    // Handle Initial Container Size Check
    {
        let container_ref = container_ref.clone();
        let show_expand_button = show_expand_button.clone();

        use_effect_with_deps(
            move |container_ref| {
                let container = container_ref
                    .cast::<HtmlElement>()
                    .expect_throw("container_ref not attached to element");

                if container.client_height() < container.scroll_height() {
                    show_expand_button.set(true);
                } else {
                    show_expand_button.set(false);
                }

                move || {}
            },
            container_ref,
        );
    }

    let mut cont_style = String::new();

    if let Some(max_lines) = props.max_lines {
        let _ = write!(cont_style, "-webkit-line-clamp: {max_lines};");
    }

    html! {
        <div class="expandable-container">
            <div class={ classes!("expanding-container", is_expanded.then(|| "expanded")) } ref={ container_ref } style={ cont_style }>
                { for props.children.iter() }
            </div>
            {
                if *show_expand_button {
                    let is_expanded_c = is_expanded.clone();
                    let onclick = Callback::from(move |_| is_expanded_c.toggle());

                    if *is_expanded {
                        html! {
                            <span class="expanding-button" { onclick }>
                                { "Contract" }
                                <span class="material-icons">{ "arrow_drop_up" }</span>
                            </span>
                        }
                    } else {
                        html! {
                            <span class="expanding-button" { onclick }>
                                { "Expand" }
                                <span class="material-icons">{ "arrow_drop_down" }</span>
                            </span>
                        }
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
