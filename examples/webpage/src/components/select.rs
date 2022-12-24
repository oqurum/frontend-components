use common::component::select::*;
use yew::prelude::*;
use yew_hooks::use_list;

#[derive(Debug, Clone, PartialEq)]
enum SelectType {
    Name,
    Address,
    Time,
    Car,
}

#[function_component(SelectPage)]
pub fn _select() -> Html {
    let on_event = Callback::from(move |v| {
        log::debug!("{v:?}");
    });

    html! {
        <>
            <h3>{ "Viewing Only" }</h3>

            <SelectModule<SelectType> class="form-select" disabled=true onselect={on_event.clone()}>
                <SelectItem<SelectType> value={ SelectType::Name } name="Name" />
                <SelectItem<SelectType> value={ SelectType::Address } name="Address" />
                <SelectItem<SelectType> value={ SelectType::Time } name="Time" />
                <SelectItem<SelectType> value={ SelectType::Car } name="Car" />
            </SelectModule<SelectType>>

            <br />
            <h3>{ "Editing" }</h3>

            <SelectModule<SelectType> class="form-select" onselect={on_event.clone()}>
                <SelectItem<SelectType> value={ SelectType::Name } name="Name" />
                <SelectItem<SelectType> value={ SelectType::Address } name="Address" />
                <SelectItem<SelectType> value={ SelectType::Time } name="Time" />
                <SelectItem<SelectType> value={ SelectType::Car } name="Car" />
            </SelectModule<SelectType>>

            <br />
            <h3>{ "Default" }</h3>

            <SelectModule<SelectType> class="form-select" default={ SelectType::Address } onselect={on_event}>
                <SelectItem<SelectType> value={ SelectType::Name } name="Name" />
                <SelectItem<SelectType> value={ SelectType::Address } name="Address" />
                <SelectItem<SelectType> value={ SelectType::Time } name="Time" />
                <SelectItem<SelectType> value={ SelectType::Car } name="Car" />
            </SelectModule<SelectType>>
        </>
    }
}
