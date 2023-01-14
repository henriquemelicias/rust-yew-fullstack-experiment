use yew::prelude::*;
use yew_router::prelude::*;
use frontend::presentation::routes;
use frontend::presentation::layout::{LayoutHeader, LayoutFooter};

#[function_component( App )]
fn app() -> Html
{
    html!
    {
        <BrowserRouter>
            <LayoutHeader />
            <Switch<routes::Route> render={routes::switch} /> // must be child of <BrowserRouter>
            <LayoutFooter />
        </BrowserRouter>
    }
}

fn main() { yew::Renderer::<App>::new().render(); }
