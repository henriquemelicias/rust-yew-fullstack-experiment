use gloo_net::http::Request;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route
{
    #[at( "/" )]
    Home,
    #[at( "/hello-server" )]
    HelloServer,
    #[not_found]
    #[at( "/404" )]
    NotFound,
}

fn switch( routes: Route ) -> Html
{
    match routes
    {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::HelloServer => html! { <HelloServer /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component( App )]
fn app() -> Html
{
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() { yew::Renderer::<App>::new().render(); }

#[function_component( HelloServer )]
fn hello_server() -> Html
{
    let data = use_state( || None );

    // Request `/api/hello` once
    {
        let data = data.clone();
        use_effect( move || {
            if data.is_none()
            {
                spawn_local( async move {
                    let resp = Request::get( "/api/hello" ).send().await.unwrap();
                    let result = {
                        if !resp.ok()
                        {
                            Err( format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ) )
                        }
                        else
                        {
                            resp.text().await.map_err( |err| err.to_string() )
                        }
                    };
                    data.set( Some( result ) );
                } );
            }

            || {}
        } );
    }

    match data.as_ref()
    {
        None =>
        {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some( Ok( data ) ) =>
        {
            html! {
                <div>{"Got server response: "}{data}</div>
            }
        }
        Some( Err( err ) ) =>
        {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}
