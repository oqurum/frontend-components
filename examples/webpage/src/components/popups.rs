use common::component::popup::{
    button::{ButtonTitle, ButtonWithPopup},
    *,
};
use yew::{prelude::*, virtual_dom::AttrValue};
use yew_hooks::use_bool_toggle;

#[function_component(PopupsPage)]
pub fn _popups_cont() -> Html {
    let showing_popup_overlay = use_bool_toggle(false);
    let showing_at_point_overlay = use_bool_toggle(false);

    let popup_overlay_cb = {
        let showing_popup_overlay = showing_popup_overlay.clone();

        Callback::from(move |_| showing_popup_overlay.toggle())
    };

    let popup_at_point_cb = {
        let showing_at_point_overlay = showing_at_point_overlay.clone();

        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            showing_at_point_overlay.toggle()
        })
    };

    html! {
        <div>
            <h3>{ "Button Popup List" }</h3>
            <br />

            <ButtonWithPopup title={ ButtonTitle::Text(AttrValue::Static("Button Popup List")) }>
                <div class="dropdown-item">{ "First Item" }</div>
                <div class="dropdown-item">{ "Second Item" }</div>
                <div class="dropdown-item">{ "Third Item" }</div>
            </ButtonWithPopup>

            <h3>{ "Popup Overlay" }</h3>
            <br />

            <button class="btn btn-secondary" onclick={ popup_overlay_cb }>{ "Show Overlay" }</button>

            {
                if *showing_popup_overlay {
                    html! {
                        <Popup type_of={ PopupType::FullOverlay } on_close={ Callback::from(move |_| showing_popup_overlay.toggle()) }>
                            <div class="modal-header">
                                <h5 class="modal-title">{ "Overlay" }</h5>
                            </div>
                            <div class="modal-body">
                                { "Body" }
                            </div>
                        </Popup>
                    }
                } else {
                    html! {}
                }
            }

            <h3>{ "Popup At Point" }</h3>
            <br />

            <button class="btn btn-secondary" onclick={ popup_at_point_cb }>{ "Show Overlay" }</button>

            {
                if *showing_at_point_overlay {
                    html! {
                        <Popup type_of={ PopupType::AtPoint(40, 80) } on_close={ Callback::from(move |_| showing_at_point_overlay.toggle()) }>
                            { "At Point Overlay: x40, y80" }
                        </Popup>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
