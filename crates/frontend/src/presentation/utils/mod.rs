use indexmap::IndexMap;
use yew::AttrValue;

pub fn attrs( entries: &[( &str, &str )] ) -> IndexMap<AttrValue, AttrValue>
{
    let mut attributes = IndexMap::with_capacity( entries.len() );

    for ( key, value ) in entries.iter()
    {
        attributes.insert( AttrValue::from( key.to_string() ), AttrValue::from( value.to_string() ) );
    }

    attributes
}
