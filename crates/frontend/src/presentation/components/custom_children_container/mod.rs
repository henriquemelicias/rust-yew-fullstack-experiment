use crate::utils::unwrap_r_abort;
use gloo::utils::document;
use indexmap::IndexMap;
use web_sys::{Element, Node};
use yew::{
    create_portal, function_component, html, use_memo, html::NodeRef, AttrValue, Children, Html, Properties, Callback
};


pub fn attrs( entries: &[( &str, &str )] ) -> IndexMap<AttrValue, AttrValue>
{
    let mut attributes = IndexMap::with_capacity( entries.len() );

    for ( key, value ) in entries.iter()
    {
        attributes.insert( AttrValue::from( key.to_string() ), AttrValue::from( value.to_string() ) );
    }

    attributes
}

#[derive(Properties, PartialEq)]
pub struct CustomChildrenContainerProps
{
    #[prop_or( AttrValue::from( "a" ) )]
    pub tag:      AttrValue,
    #[prop_or( IndexMap::new() )]
    pub attrs:    IndexMap<AttrValue, AttrValue>,
    pub children: Children,
    #[prop_or( Callback::noop() )]
    pub on_container_element: Callback<Element>
}

#[function_component( CustomChildrenContainer )]
pub fn custom_children_container( props: &CustomChildrenContainerProps ) -> Html
{
    let container = use_memo(
        |_| {
            let element: Element = unwrap_r_abort( document().create_element( props.tag.clone().as_str() ) );

            for ( key, value ) in props.attrs.iter()
            {
                element.set_attribute( key, value );
            }

            let node: Node = element.clone().into();

            props.on_container_element.emit( element.clone() );

            ( element, Html::VRef( node ) )
        },
        ( props.tag.clone(), props.attrs.clone() ),
    );

    let ( container_element, container_html ) = (*container).clone();

    let container_portal = create_portal(
        html! {
            <>
                {props.children.clone()}
            </>
        },
        container_element,
    );

    html! {
        <>
            {container_html}
            {container_portal}
        </>
    }
}

