mod components;
mod pages;
mod router;
mod api;
mod models;

use components::navigation::Navigation;
use router::{Route, switch};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navigation/>
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
