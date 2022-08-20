use std::collections::HashMap;

use gloo_utils::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::util::{truncate_on_indices, upper_case_first_char, LoadingItem};

use super::{Popup, PopupType};

type SearchResponse<Id> = std::result::Result<HashMap<String, Vec<SearchItem<Id>>>, String>;

#[derive(Properties, PartialEq)]
pub struct Property<Id: PartialEq> {
    #[prop_or_default]
    pub classes: Classes,

    pub on_close: Callback<()>,
    pub on_select: Callback<Id>,

    // TODO: Use fn(String) -> impl Future<Output = SearchResponse>
    // Currently using a hackish way. Would be better to make it simply return the requests' future.
    pub call_search: Callback<CallSearch<Id>>,

    pub input_value: String,
}

pub enum Msg<Id> {
    BookSearchResponse(String, SearchResponse<Id>),

    SearchFor(String),

    OnChangeTab(String),
    OnSelectItem(Id),
}

pub struct PopupSearch<Id> {
    cached_posters: Option<LoadingItem<SearchResponse<Id>>>,
    input_value: String,

    selected_tab: String,

    waiting_item_resp: bool,
}

impl<Id: Clone + PartialEq + 'static> Component for PopupSearch<Id> {
    type Message = Msg<Id>;
    type Properties = Property<Id>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            cached_posters: None,
            input_value: ctx.props().input_value.clone(),

            selected_tab: String::new(),

            waiting_item_resp: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SearchFor(search) => {
                self.cached_posters = Some(LoadingItem::Loading);

                ctx.props().call_search.emit(CallSearch {
                    search: search.clone(),
                    response: ctx
                        .link()
                        .callback(move |v| Msg::BookSearchResponse(search.clone(), v)),
                });
            }

            Msg::BookSearchResponse(search, resp) => {
                if let Some(name) = resp.as_ref().ok().and_then(|v| v.keys().next().cloned()) {
                    self.selected_tab = name;
                }

                self.cached_posters = Some(LoadingItem::Loaded(resp));
                self.input_value = search;
            }

            Msg::OnSelectItem(id) => {
                if self.waiting_item_resp {
                    return false;
                }

                self.waiting_item_resp = true;

                ctx.props().on_select.emit(id);
            }

            Msg::OnChangeTab(name) => {
                self.selected_tab = name;
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.render_main(ctx)
    }
}

impl<Id: Clone + PartialEq + 'static> PopupSearch<Id> {
    fn render_main(&self, ctx: &Context<Self>) -> Html {
        let input_id = "external-book-search-input";

        html! {
            <Popup
                type_of={ PopupType::FullOverlay }
                on_close={ ctx.props().on_close.clone() }
                classes={ classes!("external-book-search-popup") }
            >
                <h1>{"Book Search"}</h1>

                <form class="row">
                    <input id={input_id} name="book_search" placeholder="Search For Title" value={ self.input_value.clone() } />
                    <button onclick={
                        ctx.link().callback(move |e: MouseEvent| {
                            e.prevent_default();

                            let input = document().get_element_by_id(input_id).unwrap_throw().unchecked_into::<HtmlInputElement>();

                            Msg::SearchFor(input.value())
                        })
                    }>{ "Search" }</button>
                </form>

                <hr />

                <div class="external-book-search-container">
                    {
                        if let Some(loading) = self.cached_posters.as_ref() {
                            match loading {
                                LoadingItem::Loaded(res) => {
                                    match res {
                                        Ok(map) => html! {
                                            <>
                                                <div class="tab-bar">
                                                {
                                                    for map.iter()
                                                        .map(|(name, values)| {
                                                            let name2 = name.clone();

                                                            html! {
                                                                <div class="tab-bar-item" onclick={ ctx.link().callback(move |_| Msg::OnChangeTab(name2.clone())) }>
                                                                    { upper_case_first_char(name.clone()) } { format!(" ({})", values.len()) }
                                                                </div>
                                                            }
                                                        })
                                                }
                                                </div>

                                                <div class="book-search-items">
                                                {
                                                    for map.get(&self.selected_tab)
                                                        .iter()
                                                        .flat_map(|values| values.iter())
                                                        .map(|item| Self::render_poster_container(&self.selected_tab, item, ctx))
                                                }
                                                </div>
                                            </>
                                        },

                                        Err(e) => html! {
                                            <h2>{ e }</h2>
                                        },
                                    }
                                },

                                LoadingItem::Loading => html! {
                                    <h2>{ "Loading..." }</h2>
                                }
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </Popup>
        }
    }

    fn render_poster_container(site: &str, item: &SearchItem<Id>, ctx: &Context<Self>) -> Html {
        let id = item.id.clone();

        html! {
            <div
                class="book-search-item"
                onclick={ ctx.link().callback(move |_| Msg::OnSelectItem(id.clone())) }
            >
                <img src={ item.thumbnail_url.clone() } alt="No Image" />
                <div class="book-info">
                    <h4 class="book-name">{ item.name.as_deref().unwrap_or("(Missing Name)") }</h4>
                    <h5>{ site }</h5>
                    <span class="book-author">{ item.author.clone().unwrap_or_default() }</span>
                    <p class="book-author">{ item.description.clone()
                            .map(|mut v| { truncate_on_indices(&mut v, 300); v })
                            .unwrap_or_default() }
                    </p>
                </div>
            </div>
        }
    }
}

#[derive(Clone)]
pub struct SearchItem<Id> {
    pub id: Id,

    pub thumbnail_url: Option<String>,

    pub name: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
}

pub struct CallSearch<Id> {
    pub search: String,
    pub response: Callback<SearchResponse<Id>>,
}
