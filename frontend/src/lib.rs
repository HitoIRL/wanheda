mod components;
mod pages;
mod router;

use components::header::Header;
use router::{Route, switch};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header/>
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}