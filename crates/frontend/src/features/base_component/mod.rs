use std::collections::HashMap;
use yew::prelude::*;

macro_rules! struct_to_hashmap {
    ( $struct_name:ident { $( $field_name:ident : $field_type:ty ),* $(,)? } ) => {
        impl Settings for $struct_name
        {
            fn to_hashmap( &self, key: &str ) -> SettingsHashmap
            {
                let mut map = SettingsHashmap::new();
                map.insert( key.to_string(), self.clone().into() );
                map
            }
        }
    };
}

pub trait Settings
{
    fn to_hashmap( &self ) -> SettingsHashmap;
}

pub type SettingsHashmap = HashMap<&'static str, SettingValue<'static>>;

pub enum SettingValue<'a>
{
    U32( u32 ),
    String( &'a str ),
    Bool( bool ),
    HashMap( HashMap<String, SettingValue<'a>> ),
}

pub struct BasePluginComponent {}

impl BasePluginComponent
{
    pub(crate) fn new( settings: Option<Box<dyn Settings>> ) -> Self { Self {} }

    // fn on( &self, event_name: &str, callback: Callback<()> ) -> Self
    // {
    //     event_name.split_whitespace().for_each( |event_name| {
    //         unimplemented!();
    //     } );
    //
    //     Self
    // }
    //
    // fn once( &self, event_name: &str, callback: Callback<()> ) -> Self
    // {
    //     Self
    // }
    //
    // fn off( &self, event_name: &str, callback: Callback<()> ) -> Self
    // {
    //     Self
    // }
}

impl From<u32> for SettingValue<'static>
{
    fn from( value: u32 ) -> Self { Self::U32( value ) }
}
