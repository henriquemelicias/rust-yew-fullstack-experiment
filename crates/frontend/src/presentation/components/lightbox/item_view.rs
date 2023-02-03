use gloo::utils::document;
use indexmap::IndexMap;
use std::{ops::Deref, rc::Rc};
use web_sys::*;
use yew::{
    html::IntoPropValue,
    prelude::*,
    virtual_dom::{ApplyAttributeAs, Attributes, VNode},
};

use super::{lightbox_state::LightboxState, modal_view::LightboxModal};

use crate::{
    features::base_component::{BasePluginComponent, SettingValue, Settings},
    presentation::components::custom_children_container::CustomChildrenContainer,
    utils::{unwrap_abort, unwrap_r_abort},
};
use gloo::events::EventListener;
use gloo_console::info;
use yew::html::Scope;
use yewdux::prelude::Dispatch;

#[derive(Properties, PartialEq)]
pub struct LightboxItemProps
{
    // General props.
    #[prop_or( AttrValue::from( "a" ) )]
    pub tag: AttrValue,

    #[prop_or_default]
    pub id:    Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<Classes>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Option<IndexMap<AttrValue, AttrValue>>,

    #[prop_or_default]
    pub children: Children,

    // Specific props.
    pub data_src: AttrValue, // path to the image to be displayed in the lightbox.
    #[prop_or( AttrValue::from( "gallery" ) )]
    pub gallery:  AttrValue, // name of the gallery in which the lightbox item is included.
    #[prop_or_default]
    pub caption:  Option<AttrValue>, // caption for the lightbox item.
}

pub enum LightboxItemMsg
{
    State( Rc<LightboxState> ),
    DontUpdate,
    OpenLightbox,
}

pub struct LightboxItem
{
    state:    Rc<LightboxState>,
    dispatch: Dispatch<LightboxState>,
    node_ref: NodeRef,
}

impl Component for LightboxItem
{
    type Message = LightboxItemMsg;
    type Properties = LightboxItemProps;

    fn create( ctx: &Context<Self> ) -> Self
    {
        let props = ctx.props();
        let link = ctx.link();
        let dispatch = Dispatch::<LightboxState>::subscribe( link.callback( LightboxItemMsg::State ) );

        // Add the data source to the corresponding gallery on the lightbox state.
        dispatch.reduce_mut( |state| state.add_data_source( &props.gallery, &props.data_src ) );

        Self {
            state: dispatch.get(),
            dispatch,
            node_ref: NodeRef::default(),
        }
    }

    fn update( &mut self, _ctx: &Context<Self>, msg: Self::Message ) -> bool
    {
        match msg
        {
            LightboxItemMsg::State( state ) =>
            {
                self.state = state;
                true
            }
            LightboxItemMsg::DontUpdate => false,
            LightboxItemMsg::OpenLightbox =>
            {
                self.dispatch.reduce_mut( |state| state.is_open = true );
                false
            }
        }
    }

    fn changed( &mut self, ctx: &Context<Self>, old_props: &Self::Properties ) -> bool
    {
        let props = ctx.props();

        if props.gallery != old_props.gallery || props.data_src != old_props.data_src
        {
            // Update the data source to the corresponding gallery on the lightbox state.
            self.dispatch.reduce_mut( |state| {
                state.remove_data_source( &old_props.gallery, &old_props.data_src );
                state.remove_gallery_if_empty( &old_props.gallery );
                state.add_data_source( &props.gallery, &props.data_src );
            } );

            true
        }
        else
        {
            false
        }
    }

    fn view( &self, ctx: &Context<Self> ) -> Html
    {
        let props = ctx.props();
        let link = ctx.link();

        let onclick = LightboxItem::on_click( &link );

        html! {
            <>
                <@{props.tag.clone().to_string()}
                    ref={self.node_ref.clone()}
                    id={props.id.clone()}
                    class={props.class.clone()}
                    style={props.style.clone()}

                    {onclick}
                >
                    {props.children.clone()}
                </@>
            </>
        }
    }

    fn rendered( &mut self, ctx: &Context<Self>, _first_render: bool )
    {
        let props = ctx.props();

        // Set attrs props to node_ref.
        if let Some( element ) = self.node_ref.cast::<HtmlElement>()
        {
            let mut has_href = false;

            // Check attrs property.
            if let Some( attrs ) = props.attrs.as_ref()
            {
                for ( key, value ) in attrs.iter()
                {
                    // Check if href is set.
                    if key.as_str() == "href"
                    {
                        has_href = true;
                    }

                    element.set_attribute( key, value );
                }
            }

            // If tag is "a" and href is not set, set it to data_src.
            if props.tag.as_str() == "a" && !has_href
            {
                element.set_attribute( "href", &props.data_src );
            }
        }
    }
}

impl LightboxItem
{
    fn on_click( link: &Scope<LightboxItem> ) -> Callback<MouseEvent>
    {
        link.callback( |event: MouseEvent| {
            // Allow disabling default behaviour.
            if event.default_prevented()
                // Ignore non left click events.
                || event.button() != 0
                // Ignore clicks with modifier keys.
                || event.ctrl_key()
                || event.meta_key()
                || event.shift_key()
            {
                return LightboxItemMsg::DontUpdate;
            }

            // Prevent default behaviour.
            event.prevent_default();

            LightboxItemMsg::OpenLightbox
        } )
    }
}
