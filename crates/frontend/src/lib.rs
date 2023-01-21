#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

#[cfg( target_arch = "wasm32" )]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg( target_arch = "wasm32" )]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> = LockedAllocator::new( FreeListAllocator::new() );

#[macro_use]
mod macros;

pub mod domain;
pub mod features;
pub mod infrastructure;
pub mod presentation;
pub mod utils;

use presentation::{layout, routes};

use yew::prelude::*;
use yew_router::{history, history::History, prelude::*};

#[function_component( Layout )]
pub fn layout() -> Html
{
    html!(
        <>
            <layout::Header />

            <main>
                <Switch<routes::Route> render={routes::switch} /> // must be child of <BrowserRouter>
            </main>

            <layout::Footer />
        </>
    )
}

#[function_component( App )]
pub fn app() -> Html
{
    html! {
        <BrowserRouter>
            <Layout />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct ServerAppProps
{
    pub url: AttrValue,
}

#[function_component( ServerApp )]
pub fn server_app( props: &ServerAppProps ) -> Html
{
    let history = history::AnyHistory::from( history::MemoryHistory::new() );
    history.push( &*props.url );

    html! {
        <Router history={history}>
            <Layout />
        </Router>
    }
}
