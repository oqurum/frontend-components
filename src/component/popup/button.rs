use yew::prelude::*;

use super::{Popup, PopupType};


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
    is_open: bool,
}

impl Component for ButtonWithPopup {
    type Message = ButtonMsg;
    type Properties = ButtonProperty;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_open: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ButtonMsg::TogglePopup => {
                self.is_open = !self.is_open;
            }

            ButtonMsg::ClosePopup => {
                self.is_open = false;
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={ classes!("button-popup-group", ctx.props().class.clone()) }>
                <span
                    class="button material-icons"
                    title="More Options"
                    onclick={ ctx.link().callback(|_| ButtonMsg::TogglePopup) }
                >{ "more_horiz" }</span>

                {
                    if self.is_open {
                        html! {
                            <Popup
                                type_of={ PopupType::Display }
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