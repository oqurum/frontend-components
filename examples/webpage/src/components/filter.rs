use common::component::filter::*;
use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum FilterType {
    #[default]
    Abc,
    OneTwoThree,
    BabyYouAndMe,
}

#[function_component(FilterPage)]
pub fn _filter_cont() -> Html {
    let comp_value = use_state_eq(FilterType::default);
    let comp_value_set = comp_value.setter();

    html! {
        <div style="max-width: 300px">
            <h3>{ "Filter Dropdown" }</h3>
            <br />

            <FilterContainerComponent<FilterType>
                value={ *comp_value }
                on_click={ Callback::from(move |v| comp_value_set.set(v)) }
            >
                <FilterItemDropdown<FilterType> title="ABC">
                    <FilterItemRedirect<FilterType> title="Testing 1" type_of={ FilterType::Abc } search={ (AttrValue::Static("abc"), AttrValue::Static("def")) } />
                </FilterItemDropdown<FilterType>>

                <FilterItemDropdown<FilterType> title="123">
                    <FilterItemRedirect<FilterType> title="Testing 2" type_of={ FilterType::OneTwoThree } search={ (AttrValue::Static("123"), AttrValue::Static("456")) } />
                </FilterItemDropdown<FilterType>>

                <FilterItemRedirect<FilterType> title="Baby You and Me" type_of={ FilterType::BabyYouAndMe } search={ (AttrValue::Static("baby"), AttrValue::Static("You and Me")) } />
            </FilterContainerComponent<FilterType>>
        </div>
    }
}
