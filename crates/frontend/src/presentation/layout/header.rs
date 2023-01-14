use yew::html;
use yew::prelude::*;

#[function_component( LayoutHeader )]
pub fn header() -> Html
{
    html!
    {
        <header>
            <p1> { "Header" } </p1>
        </header>
    }
}