use std::rc::Rc;
use yew::prelude::*;

use super::viewmodel::ViewModel;
use crate::features::base_component::{BasePluginComponent, SettingValue, Settings};

enum LightboxMsg
{
    Init,
    Ready,
    Closing,
    Destroy
}

#[derive(Properties, PartialEq)]
struct LightboxProps
{
    pub container_class: Classes,
    pub children:        Children,
}

struct LightBox
{
    base_component: BasePluginComponent,
    view_model: ViewModel,
}

impl Component for LightBox
{
    type Message = LightboxMsg;
    type Properties = LightboxProps;

    fn create( _ctx: &Context<Self> ) -> Self
    {
        Self {
            base_component: BasePluginComponent::new(None),
            view_model: ViewModel::new(),
        }
    }

    fn view( &self, ctx: &Context<Self> ) -> Html
    {
        let props = ctx.props();
        html! {
            <div class={ props.container_class.to_string() }>
                { props.children.clone() }
            </div>
        }
    }
}
