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
            <div class="info-row">
                <p>{ format!("{}", product.title) }</p>
                <p>{ format!("{}pln", product.price) }</p>
            </div>
        </div>
    }).collect::<Html>();

    let stylesheet = style!(r#"
    display: grid;
    grid-template-columns: repeat(auto-fill, 430px);
    gap: 10px;
    margin: 10px 10px;

    .product {
        display: flex;
        flex-direction: column;
        
        border: 1px solid black;
    }

    .info-row {
        display: flex;
        justify-content: space-between;
        margin: 0 100px;
    }
    "#).unwrap();

    html! {
        <div class={ stylesheet }>
            { products }
        </div>
    }
}
