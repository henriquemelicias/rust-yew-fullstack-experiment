fn main()
{
    #[cfg( target_arch = "wasm32" )]
    {
        #[cfg( feature = "csr" )]
        yew::Renderer::<frontend::App>::new().render();

        #[cfg( feature = "ssr" )]
        yew::Renderer::<frontend::App>::new().hydrate();
    }
}
