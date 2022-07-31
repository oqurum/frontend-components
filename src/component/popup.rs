use std::{rc::Rc, cell::Cell};

use gloo_utils::body;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{MouseEvent, Element};
use yew::prelude::*;

use crate::util::{does_parent_contain_class, does_parent_contain_attribute};


pub mod compare;
pub mod button;
pub mod search;

static YEW_CLOSE_POPUP: &str = "yew_close_popup";

#[derive(Clone, Copy, PartialEq)]
pub enum PopupType {
    /// Full foreground overlay
    FullOverlay,
    /// Places the popover at the specified point and attempts to keep it there while staying readable.
    AtPoint(i32, i32),
    /// Displays the popup
    Display,
}

impl PopupType {
    pub fn should_exit(self, element: Element) -> bool {
        match self {
            // If we clicked .popup
            Self::FullOverlay if element.class_list().contains("popup") => true,
            // If we didn't click inside of the container
            Self::AtPoint(_, _) if !does_parent_contain_class(&element, "popup-at-point") => true,
            // Otherwise just check for a "data-close-popup" attribute
            _ => does_parent_contain_attribute(&element, YEW_CLOSE_POPUP)
        }
    }
}



#[derive(Properties, PartialEq)]
pub struct Property {
    #[prop_or_default]
    pub classes: Classes,

    pub children: Children,
    pub type_of: PopupType,

    pub on_close: Callback<()>
}


pub struct Popup {
    node_ref: NodeRef,
    #[allow(clippy::type_complexity)]
    closure: Rc<Cell<Option<Closure<dyn FnMut(MouseEvent)>>>>,
}

impl Component for Popup {
    type Message = ();
    type Properties = Property;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            closure: Rc::new(Cell::default()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match ctx.props().type_of {
            PopupType::FullOverlay => html! {
                <div ref={self.node_ref.clone()} class="popup">
                    <div class={classes!("popup-container", ctx.props().classes.clone())}>
                        { for ctx.props().children.iter() }
                    </div>
                </div>
            },

            PopupType::AtPoint(pos_x, pos_y) => {
                let styling = format!("left: {}px; top: {}px;", pos_x, pos_y);

                html! {
                    <div ref={self.node_ref.clone()} class={classes!("popup-at-point", ctx.props().classes.clone())} style={ styling }>
                        { for ctx.props().children.iter() }
                    </div>
                }
            },

            PopupType::Display => {
                html! {
                    <div ref={self.node_ref.clone()} class={classes!("popup-display", ctx.props().classes.clone())}>
                        { for ctx.props().children.iter() }
                    </div>
                }
            },
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        // TODO: On render check dimensions of and adjust "AtPoint"

        // FIX: rendered would be called again if we clicked an element containing another onclick event.
        // Resulted in our previous event being overwritten but not removed from the listener.
        if let Some(func) = self.closure.take() {
            let _ = body().remove_event_listener_with_callback("click", func.as_ref().unchecked_ref());
        }

        let viewing = ctx.props().type_of;
        let exit_fn = ctx.props().on_close.clone();

        let on_click = Closure::wrap(Box::new(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if viewing.should_exit(target.unchecked_into()) {
                    exit_fn.emit(());
                }
            }
        }) as Box<dyn FnMut(MouseEvent)>);

        let _ = body().add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref());

        self.closure = Rc::new(Cell::new(Some(on_click)));
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(func) = self.closure.take() {
            let _ = body().remove_event_listener_with_callback("click", func.as_ref().unchecked_ref());
        }
    }
}


#[derive(PartialEq, Properties)]
pub struct PopupCloseProps {
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
    pub title: Option<String>,

    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(PopupClose)]
pub fn _close(props: &PopupCloseProps) -> Html {
    html! {
        <div
            title={ props.title.clone() }
            class={ props.class.clone() }
            { YEW_CLOSE_POPUP }
            onclick={ props.onclick.clone() }
        >
            { for props.children.iter() }
        </div>
    }
}