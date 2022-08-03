use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, AppRoute};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="navbar-module">
                <div class="left-content">
                    <Link<AppRoute> to={ AppRoute::Home }>{ "Home" }</Link<AppRoute>>
                </div>
                <div class="center-content"></div>
                <div class="right-content"></div>
            </div>
            <div class="outer-view-container">
                <div class="view-container">
                    <Switch<AppRoute> render={ Switch::render(switch) } />
                </div>
            </div>
        </BrowserRouter>
    }
}