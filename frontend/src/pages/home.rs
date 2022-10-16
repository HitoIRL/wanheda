use yew::prelude::*;
use stylist::{yew::styled_component, style};

#[styled_component(Home)]
pub fn home() -> Html {
    let stylesheet = style!(r#"
    display: flex;
    justify-content: center;
    "#).unwrap();

    html! {
        <div class={stylesheet}>
            <h1>{ "Coming soon" }</h1>
        </div>
    }
}
