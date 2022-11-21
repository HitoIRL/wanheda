use super::redirect::Redirect;
use crate::Route;

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use yew_router::prelude::use_route;

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    let route = use_route::<Route>().unwrap();
    if route == Route::NotFound {
        return html! { };
    }

    let stylesheet = style!(r#"
    float: left;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    margin: 0 50px 72px 50px;

    a {
        font-weight: bold;
        font-size: 1.3em;
        margin: 15px 0;
    }

    a:hover {
        color: var(--main-color);
    }

    img {
        width: 300px;
        margin: 50px 0 15px;
    }
    "#).unwrap();

    html! {
        <div class={stylesheet}>
            <img alt="logo" src="img/logo.png"/>
            <Redirect text="COLLECTION" route={Route::Home}/>
            <Redirect text="LOGIN" route={Route::Home}/>
            <Redirect text="REGISTER" route={Route::Home}/>
            <Redirect text="SHIPPING" route={Route::Home}/>
            <Redirect text="TERMS" route={Route::Home}/>
            <Redirect text="CONTACT" route={Route::Home}/>
        </div>
    }
}
