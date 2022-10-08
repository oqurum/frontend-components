use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CarouselProps {
    pub children: Children,
}

#[function_component(CarouselComponent)]
pub fn _carousel_comp(props: &CarouselProps) -> Html {
    html! {
        <div class="carousel">
            // <div class="carousel-left-control">
            //     <span class="material-icons">{ "arrow_left" }</span>
            // </div>
            <div class="carousel-items">
                { for props.children.iter() }
            </div>
            // <div class="carousel-left-control">
            //     <span class="material-icons">{ "arrow_right" }</span>
            // </div>
        </div>
    }
}
