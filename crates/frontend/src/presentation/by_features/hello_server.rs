use crate::{
    presentation::{components::lightbox::item_view::LightboxItem, utils::attrs},
    utils::unwrap_r_abort,
};
use gloo_net::http::Request;
use yew::{html, platform::spawn_local, prelude::*};

#[must_use]
pub fn component() -> Html
{
    html! { <HelloServer /> }
}

#[function_component( HelloServer )]
fn hello_server() -> Html
{
    let data = use_state( || None );
    let href = use_state( || "assets/images/test.jpg" );

    // Request `/api/hello` once
    {
        let data = data.clone();
        use_effect( move || {
            if data.is_none()
            {
                spawn_local( async move {
                    let resp = unwrap_r_abort( Request::get( "/api/hello" ).send().await );
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

    let onclick = {
        let href = href.clone();
        Callback::from( move |_: MouseEvent| {
            href.set( "assets/images/404.jpg" );
        } )
    };

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
                <>
                    <div class="bg-sky-700 px-4 py-2 text-white hover:bg-sky-800 sm:px-8 sm:py-3">{"Got server response: "}{data}</div>

                    <LightboxItem data_src={href.to_string()} gallery="lightbox-test" class={classes!( "container" )}>
                        <img src="assets/images/test.webp" alt="test img" width="500" height="400" decoding="async"/>
                    </LightboxItem>
                    <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 16 16"><path d="M6.12 6.75H4.87v7.05l-2.05-3-1 .71 2.67 3.93a1.29 1.29 0 0 0 1 .59 1.29 1.29 0 0 0 1-.59l2.67-3.93-1-.71-2.06 3zM9.45.59 6.78 4.52l1 .71 2.06-3v7.02h1.25v-7l2 3 1-.71L11.55.59a1.23 1.23 0 0 0-2.1 0z"/></svg>

                    <button {onclick}>{"Click me!!!!"}</button>
                </>
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
