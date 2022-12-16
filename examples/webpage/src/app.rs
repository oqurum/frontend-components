use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, AppRoute};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav class="navbar navbar-expand-lg text-bg-dark">
                <div class="container-fluid">
                    <ul class="navbar-nav">
                        <li class="nav-item"><Link<AppRoute> to={ AppRoute::Home }>{ "Home" }</Link<AppRoute>></li>
                    </ul>
                </div>
            </nav>
            <div class="outer-view-container d-flex flex-column">
                <div class="view-container">
                    <Switch<AppRoute> render={ switch } />
                </div>
            </div>
        </BrowserRouter>
    }
}
