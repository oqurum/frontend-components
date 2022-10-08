use yew::prelude::*;

use common::component::CarouselComponent;

#[function_component(CarouselPage)]
pub fn _carousel() -> Html {
    html! {
        <CarouselComponent>
            { for (0..20).map(gene) }
        </CarouselComponent>
    }
}


fn gene(index: usize) -> Html {
    let book = format!("Book Title {index}");
    let author = format!("Author Title {index}");

    html! {
        <a href="/" class="book-list-item">
            <div class="poster">
                <img src="https://via.placeholder.com/400x600" />
            </div>
            <div title="Reading Chapter 5/73" class="progress">
                <div style="width: 5%;" class="prog-bar"></div>
            </div>
            <div class="info">
                <div title={ book.clone() } class="title">{ book }</div>
                <div title={ author.clone() } class="author">{ author }</div>
            </div>
        </a>
    }
}