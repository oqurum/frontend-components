use yew::prelude::*;
use yew_router::{prelude::Link, Routable};

use crate::components::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,

    #[at("/multiselect")]
    MultiSelect,
}

pub fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <Home /> },

        AppRoute::MultiSelect => html! { <MultiSelectPage /> },
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <ul>
            <li><Link<AppRoute> to={ AppRoute::MultiSelect }>{ "MultiSelect" }</Link<AppRoute>></li>
        </ul>
    }
}
