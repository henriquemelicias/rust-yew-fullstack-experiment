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

use presentation::{components::lightbox, layout, routes};

use crate::utils::unwrap_r_abort;
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

            <lightbox::modal_view::LightboxModal />
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

#[cfg( feature = "ssr" )]
#[derive(Properties, PartialEq, Eq)]
pub struct RequestData
{
    pub url:     String,
    pub queries: Vec<( String, String )>,
}

#[cfg( feature = "ssr" )]
#[derive(Properties, PartialEq, Eq)]
pub struct ServerAppProps
{
    pub request_data: RequestData,
}

#[cfg( feature = "ssr" )]
#[function_component( ServerApp )]
pub fn server_app( props: &ServerAppProps ) -> Html
{
    let history = history::AnyHistory::from( history::MemoryHistory::new() );
    unwrap_r_abort( history.push_with_query( &*props.request_data.url, &props.request_data.queries ) );

    html! {
        <Router history={history}>
            <Layout />
        </Router>
    }
}
