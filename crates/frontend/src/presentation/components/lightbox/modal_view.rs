use gloo::utils::document;
use indexmap::IndexMap;
use std::{ops::Deref, rc::Rc};
use web_sys::*;
use yew::{
    html::IntoPropValue,
    prelude::*,
    virtual_dom::{ApplyAttributeAs, Attributes, VNode},
};

use super::lightbox_state::LightboxState;
use crate::{
    features::base_component::{BasePluginComponent, SettingValue, Settings},
    presentation::components::custom_children_container::CustomChildrenContainer,
    utils::{unwrap_abort, unwrap_r_abort},
};
use gloo::events::EventListener;
use yew_router::prelude::RouterScopeExt;
use yewdux::prelude::Dispatch;

#[derive(Properties, PartialEq)]
pub struct LightboxModalProps {}

pub enum LightboxModalMsg
{
    State( Rc<LightboxState> ),
}

pub struct LightboxModal
{
    state:    Rc<LightboxState>,
    dispatch: Dispatch<LightboxState>,
}

impl Component for LightboxModal
{
    type Message = LightboxModalMsg;
    type Properties = LightboxModalProps;

    fn create( ctx: &Context<Self> ) -> Self
    {
        let link = ctx.link();
        let dispatch = Dispatch::<LightboxState>::subscribe( link.callback( LightboxModalMsg::State ) );

        Self {
            state: dispatch.get(),
            dispatch,
        }
    }

    fn update( &mut self, _ctx: &Context<Self>, msg: Self::Message ) -> bool
    {
        match msg
        {
            LightboxModalMsg::State( state ) =>
            {
                self.state = state;
                true
            }
        }
    }

    fn view( &self, ctx: &Context<Self> ) -> Html
    {
        html! {
            if self.state.is_open
            {
                <div class={"lightbox-modal__container"}>
                </div>
            }
            else
            {
                <></>
            }
        }
    }
}
