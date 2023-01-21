use gloo::utils::document;
use indexmap::IndexMap;
use std::{ops::Deref, rc::Rc};
use web_sys::*;
use yew::{
    html::IntoPropValue,
    prelude::*,
    virtual_dom::{ApplyAttributeAs, Attributes, VNode},
};

use super::viewmodel::ViewModel;
use crate::{
    features::base_component::{BasePluginComponent, SettingValue, Settings},
    presentation::components::custom_children_container::CustomChildrenContainer,
    utils::{unwrap_abort, unwrap_r_abort},
};
use gloo::events::EventListener;

pub enum LightboxMsg
{
    Closing,
}

#[derive(Properties, PartialEq)]
pub struct LightboxProps
{
    #[prop_or( AttrValue::from( "a" ) )]
    pub tag:      AttrValue,
    #[prop_or( IndexMap::new() )]
    pub attrs:    IndexMap<AttrValue, AttrValue>,
    pub children: Children,
    #[prop_or( AttrValue::from( "gallery" ) )]
    pub gallery:  AttrValue,
}

pub struct LightBox
{
    base_component: BasePluginComponent,
    view_model:     ViewModel,
    host_element:   Element,
}

impl Component for LightBox
{
    type Message = LightboxMsg;
    type Properties = LightboxProps;

    fn create( _ctx: &Context<Self> ) -> Self
    {
        Self {
            base_component: BasePluginComponent::new( None ),
            view_model:     ViewModel::new(),
            host_element:   LightBox::get_host_element(),
        }
    }

    fn view( &self, ctx: &Context<Self> ) -> Html
    {
        let props = ctx.props();

        let lightbox_container_portal = create_portal(
            html! {
                <div class={"lightbox__container"}>
                </div>
            },
            self.host_element.clone(),
        );

        let mut children_container_attrs = props.attrs.clone();
        children_container_attrs.insert( AttrValue::from( "test" ), AttrValue::from( "" ) );

        let on_container_element = Callback::from( move |element: Element| {

            let on_click_listener = Callback::from( move |_: Event| {
                gloo_console::log!("click");
            } );


            // Create a Closure from a Box<dyn Fn> - this has to be 'static
            let _listener = EventListener::new(
                &element,
                "click",
                move |e| on_click_listener.emit(e.clone())
            );
        } );

        html! {
            <>
                {lightbox_container_portal}
                <CustomChildrenContainer {on_container_element} tag={props.tag.clone()} attrs={children_container_attrs}>
                    {props.children.clone()}
                </CustomChildrenContainer>
            </>
        }
    }
}

impl LightBox
{
    fn get_host_element() -> Element { unwrap_abort( document().body() ).into() }
}
