#[macro_export]
macro_rules! attrs
{
    () => { indexmap::IndexMap::<yew::AttrValue, yew::AttrValue>::new() };
    ( $($key:ident $(= $value:literal)?),+ ) => {{

            let mut index_map = indexmap::IndexMap::<yew::AttrValue, yew::AttrValue>::new();

            // Per entry repetition.
            $(
                let mut _value: &str = "";

                // Optional value per entry.
                $(
                    _value = concat!( $value );
                )?

                // Insert key and (optional) value to index map.
                index_map.insert( stringify!($key).into(), _value.into() );
            )*

            index_map
    }};
}

#[cfg( test )]
#[allow( non_snake_case )]
mod tests
{

    #[test]
    fn macro_attrs__no_args__empty_index_map()
    {
        assert_eq!( attrs!(), indexmap::IndexMap::<yew::AttrValue, yew::AttrValue>::new() );
    }

    #[test]
    fn macro_attrs__one_key__index_map_with_one_entry()
    {
        let mut index_map = indexmap::IndexMap::<yew::AttrValue, yew::AttrValue>::new();
        index_map.insert( "key".into(), "".into() );

        assert_eq!( attrs!( key ), index_map );
    }

    #[test]
    fn macro_attrs__one_key_value__index_map_with_one_entry()
    {
        let mut index_map = indexmap::IndexMap::<yew::AttrValue, yew::AttrValue>::new();
        index_map.insert( "key".into(), "value".into() );

        assert_eq!( attrs!( key = "value" ), index_map );
    }

    #[test]
    fn macro_attrs__multiple_entries__index_map_with_multiple_entries()
    {
        let mut index_map = indexmap::IndexMap::<yew::AttrValue, yew::AttrValue>::new();
        index_map.insert( "key1".into(), "value".into() );
        index_map.insert( "key2".into(), "".into() );
        index_map.insert( "key3".into(), "1234".into() );
        index_map.insert( "key4".into(), "true".into() );

        assert_eq!( attrs!( key1 = "value", key2, key3 = 1234, key4 = true ), index_map );
    }

    #[test]
    fn macro_attrs__two_entries_with_same_key__index_map_with_one_entry_containing_last_value()
    {
        let mut index_map = indexmap::IndexMap::<yew::AttrValue, yew::AttrValue>::new();
        index_map.insert( "key".into(), "value2".into() );

        assert_eq!( attrs!( key = "value", key = "value2" ), index_map );
    }
}
