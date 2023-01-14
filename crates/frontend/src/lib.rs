#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod presentation;
pub mod settings;

use presentation::{
    layout::{LayoutFooter, LayoutHeader},
    routes,
};

use std::collections::HashMap;
use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History, MemoryHistory},
    prelude::*,
};


#[function_component( App )]
pub fn app() -> Html
{
    html! {
        <BrowserRouter>
            <LayoutHeader />

            <main>
                <Switch<routes::Route> render={routes::switch} /> // must be child of <BrowserRouter>
            </main>

            <LayoutFooter />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps
{
    pub url:     AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component( ServerApp )]
pub fn server_app( props: &ServerAppProps ) -> Html
{
    let history = AnyHistory::from( MemoryHistory::new() );
    history.push_with_query( &*props.url, &props.queries ).unwrap();

    html! {
        <Router history={history}>
            <LayoutHeader />

            <main>
                <Switch<routes::Route> render={routes::switch} /> // must be child of <BrowserRouter>
            </main>

            <LayoutFooter />
        </Router>
    }
}
