use super::redirect::Redirect;
use crate::Route;

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use yew_router::prelude::use_route;

#[styled_component(Header)]
pub fn header() -> Html {
    let route = use_route::<Route>().unwrap();
    if route == Route::NotFound {
        return html! { };
    }

    let stylesheet = style!(r#"
    display: flex;
    justify-content: center;
    align-items: center;

    height: 170px;
    margin: 0 50px 72px 50px;

    
    border-height: 2px;
    border-bottom-style: solid;
    border-image: radial-gradient(circle, var(--main-color) 30%, rgba(255,0,0,0) 90%) 1;
    

    a {
        font-weight: bold;
        font-size: 1.3em;

        margin: 0 10px 0 10px;
    }

    img {
        height: 90px;
        padding: 30px;
    }
    "#).unwrap();

    html! {
        <div class={stylesheet}>
            <Redirect text="Home" route={Route::Home}/>
            <Redirect text="XYZ" route={Route::Home}/>
            <img src="img/logo.png" alt="logo image"/>
            <Redirect text="Login" route={Route::Home}/>
            <Redirect text="Register" route={Route::Home}/>
        </div>
    }
}
