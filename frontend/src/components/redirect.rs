// link wrapper
use crate::Route;

use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub text: String,
    pub route: Route,
}

#[styled_component(Redirect)]
pub fn redirect(props: &Props) -> Html {
    let stylesheet = style!(r#"
    color: var(--text-color);
    text-decoration: none;
    "#).unwrap();

    html! {
        <Link<Route> to={props.route.clone()} classes={classes!(stylesheet)}>{ &props.text }</Link<Route>>
    }
}
