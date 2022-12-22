use common::component::expandable_container::*;
use yew::prelude::*;

#[function_component(ExpandableContainerPage)]
pub fn _expandable_cont() -> Html {
    html! {
        <div style="max-width: 300px">
            <h3>{ "Basic Overflow" }</h3>
            <br />

            <ExpandableContainerComponent>
                { "The brilliant‚ breathtaking conclusion to J.K. Rowling’s spellbinding series. " }
                { "The heart of Book 7 is a hero’s mission — not just in Harry’s quest for the Horcruxes‚ but in his journey from boy to man — and Harry faces more danger than that found in all six books combined‚ from the direct threat of the Death Eaters and you-know-who‚ to the subtle perils of losing faith in himself. " }
                { "What will be the outcome of the final battle between Voldemort and Harry?" }
                { "\n\nSource: gale.com" }
            </ExpandableContainerComponent>

            <h3>{ "Basic Underflow" }</h3>
            <br />

            <ExpandableContainerComponent>
                { "The brilliant‚ breathtaking conclusion to J.K. Rowling’s spellbinding series. " }
                { "What will be the outcome of the final battle between Voldemort and Harry?" }
            </ExpandableContainerComponent>

            <h3>{ "Specific Contracted Line Count" }</h3>
            <br />

            <ExpandableContainerComponent max_contracted_lines={ 3 }>
                { "The brilliant‚ breathtaking conclusion to J.K. Rowling’s spellbinding series. " }
                { "What will be the outcome of the final battle between Voldemort and Harry?" }
            </ExpandableContainerComponent>
        </div>
    }
}
