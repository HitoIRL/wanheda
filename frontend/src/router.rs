use super::pages::{home::Home, not_found::NotFound};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/not_found")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home/> },
        Route::NotFound => html! { <NotFound/> },
    }
}
