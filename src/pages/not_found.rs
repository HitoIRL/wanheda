use crate::{components::redirect::Redirect, Route};

use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[styled_component(NotFound)]
pub fn not_found() -> Html {
    let stylesheet = style!(r#"
    height: 100vh;

    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;

    h1 {
        margin: 0;
    }
    "#).unwrap();

    html! {
        <div class={stylesheet}>
            <h1>{ "404" }</h1>
            <h3>{ "Looks like you got lost" }</h3>
            <Redirect text="Head back home" route={Route::Home}/>
        </div>
    }
}
