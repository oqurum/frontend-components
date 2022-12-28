use common::component::popup::button::*;
use yew::prelude::*;

#[function_component(ButtonDropdownPage)]
pub fn _dropdown() -> Html {
    html! {
        <>
            <h3>{ "Drop down" }</h3>

            <ButtonWithPopup title={ ButtonTitle::Text("Click Me!".into()) } position={ ButtonPopupPosition::Bottom }>
                <div class="dropdown-item">{ "One" }</div>
                <div class="dropdown-item">{ "Two" }</div>
                <div class="dropdown-item">{ "Three" }</div>
            </ButtonWithPopup>

            <br />
            <h3>{ "Drop up" }</h3>

            <ButtonWithPopup title={ ButtonTitle::Text("Click Me!".into()) } position={ ButtonPopupPosition::Top }>
                <div class="dropdown-item">{ "One" }</div>
                <div class="dropdown-item">{ "Two" }</div>
                <div class="dropdown-item">{ "Three" }</div>
            </ButtonWithPopup>

            <br />
            <h3>{ "Drop left" }</h3>

            <ButtonWithPopup title={ ButtonTitle::Text("Click Me!".into()) } position={ ButtonPopupPosition::Left }>
                <div class="dropdown-item">{ "One" }</div>
                <div class="dropdown-item">{ "Two" }</div>
                <div class="dropdown-item">{ "Three" }</div>
            </ButtonWithPopup>

            <br />
            <h3>{ "Drop right" }</h3>

            <ButtonWithPopup title={ ButtonTitle::Text("Click Me!".into()) } position={ ButtonPopupPosition::Right }>
                <div class="dropdown-item">{ "One" }</div>
                <div class="dropdown-item">{ "Two" }</div>
                <div class="dropdown-item">{ "Three" }</div>
            </ButtonWithPopup>
        </>
    }
}
