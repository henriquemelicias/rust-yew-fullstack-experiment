use std::env;

fn main()
{
    #[cfg( target_arch = "wasm32" )]
    {
        yew::Renderer::<frontend::App>::new().hydrate();
    }
}
