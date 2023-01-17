use std::process;

#[inline]
pub fn unwrap_abort<T>( o: Option<T> ) -> T
{
    match o
    {
        Some( t ) => t,
        None => process::abort(),
    }
}

#[inline]
pub fn unwrap_r_abort<T, E>( r: Result<T, E> ) -> T
{
    match r
    {
        Ok( t ) => t,
        Err( _ ) => process::abort(),
    }
}
