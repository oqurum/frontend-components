use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::use_event;

#[derive(Properties, PartialEq)]
pub struct InfiniteScrollProps {
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    pub event: Callback<InfiniteScrollEvent>,

    pub r#ref: Option<NodeRef>,
}

#[function_component(InfiniteScroll)]
pub fn _component_fn(props: &InfiniteScrollProps) -> Html {
    let node = if let Some(v) = props.r#ref.clone() {
        v
    } else {
        use_node_ref()
    };

    let event = props.event.clone();
    use_event(node.clone(), "scroll", move |e: Event| {
        let el = e.target_unchecked_into::<HtmlElement>();
        event.emit(InfiniteScrollEvent {
            scroll_pos: el.client_height() + el.scroll_top(),
            scroll_height: el.scroll_height(),
        });
    });

    let event = props.event.clone();
    let enode = node.clone();
    // TODO: Can cause infinite recursion if event Callback updates parent Component.
    use_effect(move || {
        let el = enode.cast::<HtmlElement>().unwrap_throw();

        event.emit(InfiniteScrollEvent {
            scroll_pos: el.client_height() + el.scroll_top(),
            scroll_height: el.scroll_height(),
        });

        || {}
    });

    html! {
        <div ref={ node } class={ props.class.clone() }>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Clone)]
pub struct InfiniteScrollEvent {
    pub scroll_pos: i32,
    pub scroll_height: i32,
}
