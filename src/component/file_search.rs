use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::{Popup, PopupType};


#[derive(Clone, Deserialize, Serialize)]
pub struct FileInfo {
    pub title: String,
    pub path: PathBuf,
    pub is_file: bool,
}


pub struct FileSearchRequest {
    pub path: PathBuf,
    pub update: Callback<(Option<PathBuf>, Vec<FileInfo>)>,
}


pub enum FileSearchEvent {
    Request(FileSearchRequest),

    Submit(PathBuf),
}


#[derive(PartialEq, Properties)]
pub struct FileSearchProps {
    #[prop_or_default]
    pub is_popup: bool,

    #[prop_or_default]
    pub show_files: bool,

    pub init_location: PathBuf,

    pub on_event: Callback<FileSearchEvent>,
}


pub enum Msg {
    OpenPath(PathBuf),
    // ( current_location, Files )
    OpenResponse((Option<PathBuf>, Vec<FileInfo>)),

    OnChange(String),

    TogglePopup,
    Submit,
}


pub struct FileSearchComponent {
    // Only used to inform us if we need to update the component.
    cached_init_location: PathBuf,

    // Current location we're in.
    current_location: PathBuf,

    // The new, set (submitted) location
    set_location: Option<PathBuf>,

    files: Vec<FileInfo>,

    show_popup: bool,

    // Have we done the initial directory call? Used to Convert "/" -> "C:/"
    initial_call: bool,
}

impl Component for FileSearchComponent {
    type Message = Msg;

    type Properties = FileSearchProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::OpenPath(ctx.props().init_location.clone()));

        Self {
            cached_init_location: ctx.props().init_location.clone(),
            current_location: ctx.props().init_location.clone(),
            set_location: None,
            files: Vec::new(),
            show_popup: false,

            initial_call: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="input-grouping">
                    <input
                        type="text"
                        readonly=false
                        value={ self.set_location.as_ref().unwrap_or(&self.cached_init_location).display().to_string() }
                        onchange={ ctx.link().callback(|e: Event| Msg::OnChange(e.target().unwrap().unchecked_into::<HtmlInputElement>().value())) }
                    />

                    <button onclick={ ctx.link().callback(|_| Msg::TogglePopup) }>{ "Open" }</button>
                </div>

                {
                    if self.show_popup {
                        html! {
                            <Popup
                                classes="file-search"
                                type_of={ PopupType::FullOverlay }
                                on_close={ ctx.link().callback(|_| Msg::TogglePopup) }
                            >
                                <div class="location">{ self.current_location.display() }</div>
                                <div class="files">
                                    {
                                        if self.current_location.parent().is_some() {
                                            let mut prev_path = self.current_location.clone();
                                            prev_path.pop();

                                            Self::create_button(FileInfo { title: String::from(".. [Back]"), path: prev_path, is_file: false }, ctx)
                                        } else {
                                            html! {}
                                        }
                                    }

                                    { for self.files.iter().cloned().map(|info| Self::create_button(info, ctx)) }
                                </div>

                                <div class="footer">
                                    <button class="red" onclick={ ctx.link().callback(|_| Msg::TogglePopup) }>{ "Cancel" }</button>
                                    <button class="green" onclick={ ctx.link().callback(|_| Msg::Submit) }>{ "Submit" }</button>
                                </div>
                            </Popup>
                        }
                    } else {
                        html! {}
                    }
                }
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnChange(value) => {
                let new_loc = Some(value.trim().to_string()).filter(|v| !v.is_empty()).and_then(|v| PathBuf::from_str(&v).ok());

                if new_loc.is_some() {
                    self.set_location = new_loc;
                }
            }

            Msg::OpenPath(path) => {
                let scope = ctx.link().clone();

                self.current_location = path.clone();
                ctx.props().on_event.emit(FileSearchEvent::Request(FileSearchRequest {
                    path,
                    update: Callback::from(move |v| scope.send_message(Msg::OpenResponse(v))),
                }));

                return false;
            }

            Msg::OpenResponse((fixed_set_location, resp)) => {
                self.files = resp;

                if let Some(location) = fixed_set_location {
                    if self.initial_call {
                        self.current_location = location;
                    } else {
                        self.set_location = Some(location);
                        self.initial_call = true;
                    }
                }
            }

            Msg::TogglePopup => {
                self.show_popup = !self.show_popup;

                if self.show_popup {
                    // Resets location to last saved location.
                    let new_location = self.set_location.as_ref().unwrap_or(&self.cached_init_location).clone();

                    if new_location != self.current_location {
                        self.files.clear();
                        ctx.link().send_message(Msg::OpenPath(new_location.clone()));
                    }

                    self.current_location = new_location;
                }
            }

            Msg::Submit => {
                self.show_popup = !self.show_popup;
                self.set_location = Some(self.current_location.clone());

                ctx.props().on_event.emit(FileSearchEvent::Submit(self.current_location.clone()));
            }
        }

        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.cached_init_location != ctx.props().init_location {
            self.set_location = None;
            self.cached_init_location = ctx.props().init_location.clone();
            self.current_location = ctx.props().init_location.clone();
            ctx.link().send_message(Msg::OpenPath(self.current_location.clone()));

            true
        } else {
            false
        }
    }
}

impl FileSearchComponent {
    fn create_button(info: FileInfo, ctx: &Context<Self>) -> Html {
        if info.is_file {
            html! {}
        } else {
            html! {
                <span
                    class="file"
                    onclick={ ctx.link().callback(move |_| Msg::OpenPath(info.path.clone())) }
                >{ info.title }</span>
            }
        }
    }
}

