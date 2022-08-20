use common::component::multi_select::*;
use yew::prelude::*;
use yew_hooks::use_list;

#[function_component(MultiSelectPage)]
pub fn _multi_select() -> Html {
    let selected_hook = use_list(vec![0, 3]);

    let selected_hook_event = selected_hook.clone();

    let items = vec![
        (0, "Zero"),
        (1, "One"),
        (2, "Two"),
        (3, "Three"),
        (4, "Four"),
    ];

    let on_event = Callback::from(move |v| match v {
        MultiSelectEvent::Toggle { toggle, id } => {
            if toggle {
                selected_hook_event.push(id);
            } else if let Some(index) = {
                let v = selected_hook_event.current().iter().position(|v| *v == id);
                v
            } {
                // Wrapped to prevent panic.
                selected_hook_event.remove(index);
            }
        }

        MultiSelectEvent::Create(_) | MultiSelectEvent::Input { .. } => (),
    });

    html! {
        <>
            <h3>{ "Viewing Only" }</h3>
            <br />

            // CHECK: on_event shouldn't do anything.
            <MultiSelectModule<usize> editing=false on_event={on_event.clone()}>
            {
                for items.iter().map(|&(id, name)| html_nested! {
                    <MultiSelectItem<usize> {id} {name} selected={ selected_hook.current().iter().any(move |v| *v == id) } />
                })
            }
            </MultiSelectModule<usize>>


            <h3>{ "Editing" }</h3>
            <br />

            <MultiSelectModule<usize> editing=true on_event={on_event.clone()}>
            {
                for items.iter().map(|&(id, name)| html_nested! {
                    <MultiSelectItem<usize> {id} {name} selected={ selected_hook.current().iter().any(move |v| *v == id) } />
                })
            }
            </MultiSelectModule<usize>>


            <h3>{ "Create Disabled" }</h3>
            <br />

            <MultiSelectModule<usize> editing=true create_new=false {on_event}>
            {
                for items.iter().map(|&(id, name)| html_nested! {
                    <MultiSelectItem<usize> {id} {name} selected={ selected_hook.current().iter().any(move |v| *v == id) } />
                })
            }
            </MultiSelectModule<usize>>
        </>
    }
}
