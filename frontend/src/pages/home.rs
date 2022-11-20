use yew::prelude::*;
use stylist::{yew::styled_component, style};

use crate::{api, models::Product};

#[styled_component(Home)]
pub fn home() -> Html {
    let stylesheet = style!(r#"
    display: flex;
    justify-content: center;
    "#).unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        let products: Vec<Product> = api::get("products").await;
        log::info!("{:?}", products);
    });

    html! {
        <div class={stylesheet}>
            <h1>{ "Coming soon" }</h1>
        </div>
    }
}
