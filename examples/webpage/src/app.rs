use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, AppRoute};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            // TODO: Navbar
            <div class="outer-view-container">
                <div class="view-container">
                    <Switch<AppRoute> render={ Switch::render(switch) } />
                </div>
            </div>
        </BrowserRouter>
    }
}