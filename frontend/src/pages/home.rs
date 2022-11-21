use yew::prelude::*;
use stylist::{yew::styled_component, style};

use crate::{api, models::Product};

#[styled_component(Home)]
pub fn home() -> Html {
    let products_state = use_state(|| Vec::<Product>::new());

    if products_state.is_empty() {
        let products_state_clone = products_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let products: Vec<Product> = api::get("products").await;
            products_state_clone.set(products);
        });
    }

    let products = products_state.iter().map(|product| html! {
        <div class="product">
            <img alt="product image" src={ api::resource(&product.image) }/>
            <p key={product.id}>{ format!("{} | {}pln", product.title, product.price) }</p> // TODO: split title and price
        </div>
    }).collect::<Html>();

    let stylesheet = style!(r#"
    display: grid;
    grid-template-columns: repeat(auto-fit, 300px);
    gap: 10px;
    justify-content: center;

    margin-left: 50px;
    margin-right: 50px;

    .product {
        display: flex;
        flex-direction: column;

        width: 300px;
    }
    "#).unwrap();

    html! {
        <div class={ stylesheet }>
            { products }
        </div>
    }
}
