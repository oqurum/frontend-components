use web_sys::HtmlElement;
use yew::prelude::*;

use super::{Popup, PopupType};

// TODO: Update on Resize

// TODO: Implement.
// #[derive(Clone, Copy)]
// pub enum ButtonPopupPosition {
//     Top,
//     Bottom,
//     Left,
//     Right,
// }

#[derive(Properties, PartialEq)]
pub struct ButtonProperty {
    #[prop_or_default]
    pub class: Classes,

    pub children: Children,
}

pub enum ButtonMsg {
    TogglePopup,
    ClosePopup,
}

pub struct ButtonWithPopup {
    open_pos: Option<(i32, i32)>,
    but_ref: NodeRef,
}

impl Component for ButtonWithPopup {
    type Message = ButtonMsg;
    type Properties = ButtonProperty;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            open_pos: None,
            but_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ButtonMsg::TogglePopup => {
                if self.open_pos.is_none() {
                    let element = self.but_ref.cast::<HtmlElement>().unwrap();
                    let bb = element.get_bounding_client_rect();

                    self.open_pos = Some(((bb.x() + bb.width()) as i32, bb.y() as i32));
                } else {
                    self.open_pos = None;
                }
            }

            ButtonMsg::ClosePopup => {
                self.open_pos = None;
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={ classes!("button-popup-group", ctx.props().class.clone()) }>
                <span
                    ref={ self.but_ref.clone() }
                    class="button material-icons"
                    title="More Options"
                    onclick={ ctx.link().callback(|e: MouseEvent| {
                        e.prevent_default();
                        e.stop_immediate_propagation();
                        e.stop_propagation();

                        ButtonMsg::TogglePopup
                    }) }
                >{ "more_horiz" }</span>

                {
                    if let Some((x, y)) = self.open_pos {
                        html! {
                            <Popup
                                type_of={ PopupType::AtPoint(x, y) }
                                on_close={ ctx.link().callback(|_| ButtonMsg::ClosePopup) }
                                classes="menu-list"
                            >
                                { for ctx.props().children.iter() }
                            </Popup>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }
}
