use crate::presentation::by_features;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Copy, Clone, Routable, PartialEq, Eq)]
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

#[must_use]
pub fn switch( routes: Route ) -> Html
{
    match routes
    {
        Route::Home => html! {
            <>
            <h1 class="text-9xl font-bold underline">{ "Home" }</h1>
            <Link<Route> to={Route::HelloServer}>{ "click here to go to hello-server" }</Link<Route>>
            </>
        },
        Route::HelloServer => by_features::hello_server::component(),
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
