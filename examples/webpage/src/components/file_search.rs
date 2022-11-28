use std::{collections::HashMap, path::PathBuf};

use yew::prelude::*;

use common::component::{file_search::FileInfo, FileSearchComponent, FileSearchEvent};

#[function_component(FileSearchPage)]
pub fn _filesearch() -> Html {
    let handle = use_state(String::default);
    let handle2 = handle.clone();

    let cached_paths: HashMap<&'static str, Vec<(&'static str, bool)>> = HashMap::from([
        (
            "/",
            vec![
                ("Testing", false),
                ("Another Test", false),
                ("What is this", false),
                ("My Files", true),
            ],
        ),
        ("/Testing", vec![]),
        ("/Another Test", vec![]),
        ("/What is this", vec![]),
    ]);

    html! {
        <>
            <span>{ handle2.to_string() }</span>

            <FileSearchComponent
                init_location={ PathBuf::from("/") }
                on_event={ Callback::from(move |e| match e {
                    FileSearchEvent::Request(req) => {
                        handle.set(format!("Request: {:?}", req.path.display()));

                        if let Some(values) = cached_paths.get(req.path.to_str().unwrap()) {
                            let items = values
                                .iter()
                                .map(|&(ref title, is_file)| {
                                    let mut path = req.path.clone();
                                    path.push(title);

                                    FileInfo { title: title.to_string(), path, is_file }
                                })
                                .collect();

                            req.update.emit(items);
                        } else {
                            req.update.emit(Vec::new());
                        }
                    }

                    FileSearchEvent::Submit(path) => {
                        handle.set(format!("Submit: {:?}", path.display()));
                    }
                }) }
            />
        </>
    }
}
