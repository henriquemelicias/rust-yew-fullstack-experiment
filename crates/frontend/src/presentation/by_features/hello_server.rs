use crate::{
    presentation::components::{lightbox::view::LightBox},
    utils::unwrap_r_abort,
};
use gloo_net::http::Request;
use yew::{html, platform::spawn_local, prelude::*};
use crate::presentation::components::custom_children_container::attrs;

#[must_use]
pub fn component() -> Html
{
    html! { <HelloServer /> }
}

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
                    <div>{"Got server response: "}{data}</div>

                    <LightBox tag="div" gallery="lightbox-text" attrs={attrs!(href="assets/images/test.jpg")}>
                        <img src="assets/images/test.jpg" alt="test img" width="500" height="400"/>
                    </LightBox>
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
