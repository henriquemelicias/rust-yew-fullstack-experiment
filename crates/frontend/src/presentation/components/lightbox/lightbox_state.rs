use std::{cell::RefCell, collections::HashMap, rc::Rc};
use yew::{AttrValue, NodeRef};
use yewdux::prelude::Store;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct LightboxState
{
    pub is_open:                 bool,
    pub data_sources_by_gallery: Rc<RefCell<HashMap<String, Vec<String>>>>,
}

impl LightboxState
{
    pub fn add_data_source( &mut self, gallery: &AttrValue, data_src: &AttrValue )
    {
        let gallery = gallery.as_str();
        let data_src = data_src.as_str();

        let mut data_sources_by_gallery = self.data_sources_by_gallery.borrow_mut();
        let data_sources = data_sources_by_gallery
            .entry( gallery.to_string() )
            .or_insert( Vec::new() );

        if !data_sources.contains( &data_src.to_string() )
        {
            data_sources.push( data_src.to_string() );
        }
    }

    pub fn remove_data_source( &mut self, gallery: &AttrValue, data_src: &AttrValue )
    {
        let gallery = gallery.as_str();
        let data_src = data_src.as_str();

        let mut data_sources_by_gallery = self.data_sources_by_gallery.borrow_mut();

        if let Some( data_sources ) = data_sources_by_gallery.get_mut( gallery )
        {
            data_sources.retain( |data_source| data_source != data_src );
        }
    }

    pub fn has_gallery( &self, gallery: &AttrValue ) -> bool
    {
        let gallery = gallery.as_str();

        let data_sources_by_gallery = self.data_sources_by_gallery.borrow();
        data_sources_by_gallery.contains_key( gallery )
    }

    pub fn remove_gallery_if_empty( &mut self, gallery: &AttrValue )
    {
        let gallery = gallery.as_str();

        let mut data_sources_by_gallery = self.data_sources_by_gallery.borrow_mut();

        if let Some( data_sources ) = data_sources_by_gallery.get( gallery )
        {
            if data_sources.is_empty()
            {
                data_sources_by_gallery.remove( gallery );
            }
        }
    }
}
