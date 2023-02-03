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
                    <div class="teste">{"Got server response: "}{data}</div>

                    <LightboxItem data_src={href.to_string()} gallery="lightbox-test" class={classes!( "container" )}>
                        <img src="assets/images/test.jpg" alt="test img" width="500" height="400"/>
                    </LightboxItem>

                    <button {onclick}>{"Click me!"}</button>
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
