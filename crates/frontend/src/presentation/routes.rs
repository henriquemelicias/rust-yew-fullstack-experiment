use yew::{Html, html};
use yew_router::prelude::*;
use crate::presentation::by_features;

#[derive(Clone, Routable, PartialEq)]
pub enum Route
{
    #[at( "/" )]
    Home,
    #[at( "/hello-server" )]
    HelloServer,
    #[not_found]
    #[at( "/404" )]
    NotFound,
}

pub fn switch( routes: Route ) -> Html
{
    match routes
    {
        Route::Home => html! {
            <>
            <h1>{ "Home" }</h1>
            <Link<Route> to={Route::HelloServer}>{ "click here to go home" }</Link<Route>>
            </>
        },
        Route::HelloServer => by_features::hello_server::component(),
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

