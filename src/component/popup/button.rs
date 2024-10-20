use yew::{prelude::*, virtual_dom::AttrValue};

use super::{Popup, PopupType};

// TODO: Update on Resize

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonPopupPosition {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

#[derive(PartialEq, Eq)]
pub enum ButtonTitle {
    Text(AttrValue),
    Icon(&'static str),
}

impl Default for ButtonTitle {
    fn default() -> Self {
        Self::Icon("more_horiz")
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProperty {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub title: ButtonTitle,

    #[prop_or_default]
    pub on_close_popup: Option<Callback<()>>,

    #[prop_or_default]
    pub position: ButtonPopupPosition,

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
        Self { is_open: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ButtonMsg::TogglePopup => {
                self.is_open = !self.is_open;

                if !self.is_open {
                    if let Some(cb) = ctx.props().on_close_popup.as_ref() {
                        cb.emit(());
                    }
                }
            }

            ButtonMsg::ClosePopup => {
                self.is_open = false;

                if let Some(cb) = ctx.props().on_close_popup.as_ref() {
                    cb.emit(());
                }
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let popup_type = match ctx.props().position {
            ButtonPopupPosition::Top => "popup-top",
            ButtonPopupPosition::Bottom => "popup-bottom",
            ButtonPopupPosition::Left => "popup-left",
            ButtonPopupPosition::Right => "popup-right",
        };

        html! {
            <div class={ classes!("button-popup-group", ctx.props().class.clone()) }>
                {
                    match &ctx.props().title {
                        ButtonTitle::Text(text) => html! {
                            <button class="btn btn-secondary btn-sm" title="More Options"
                                onclick={ ctx.link().callback(|e: MouseEvent| {
                                    e.prevent_default();
                                    e.stop_propagation();

                                    ButtonMsg::TogglePopup
                                }) }
                            >{ text.clone() }</button>
                        },

                        &ButtonTitle::Icon(text) => html! {
                            <span class="material-icons" title="More Options"
                                onclick={ ctx.link().callback(|e: MouseEvent| {
                                    e.prevent_default();
                                    e.stop_propagation();

                                    ButtonMsg::TogglePopup
                                }) }
                            >{ text }</span>
                        },
                    }
                }

                {
                    if self.is_open {
                        html! {
                            <Popup
                                type_of={ PopupType::Display }
                                on_close={ ctx.link().callback(|_| ButtonMsg::ClosePopup) }
                                classes={ classes!("dropdown-menu", "dropdown-menu-dark", "show", popup_type) }
                            >
                                {
                                    for ctx.props().children
                                        .iter()
                                        .map(|v| html! {
                                            <li>{ v }</li>
                                        })
                                }
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
