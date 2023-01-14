use frontend::presentation::{
    layout::{LayoutFooter, LayoutHeader},
    routes,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component( App )]
fn app() -> Html
{
    html! {
        <BrowserRouter>
            <LayoutHeader />
            <Switch<routes::Route> render={routes::switch} /> // must be child of <BrowserRouter>
            <LayoutFooter />
        </BrowserRouter>
    }
}

fn main() { yew::Renderer::<App>::new().render(); }
