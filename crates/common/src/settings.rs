use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde( rename_all = "lowercase" )]
pub enum RuntimeEnvironmentType
{
    Development,
    Production,
}

impl std::fmt::Display for RuntimeEnvironmentType
{
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result
    {
        match self
        {
            Self::Development => write!( f, "development" ),
            Self::Production => write!( f, "production" ),
        }
    }
}

impl From<&str> for RuntimeEnvironmentType
{
    fn from( env: &str ) -> Self
    {
        match env.to_lowercase().as_str()
        {
            "production" => Self::Production,
            _ => Self::Development,
        }
    }
}

pub trait ImportFigment<T: Deserialize<'static>>
{
    #[must_use]
    fn import( file_path: &str, env_prefix: &str, runtime_environment: Option<&RuntimeEnvironmentType> ) -> T
    {
        import::<T>( file_path, env_prefix, runtime_environment )
    }
}

fn import<T: Deserialize<'static>>(
    file_path: &str,
    env_prefix: &str,
    runtime_environment: Option<&RuntimeEnvironmentType>,
) -> T
{
    let mut figment = Figment::new().merge( Toml::file( file_path ).nested() );

    if let Some( run_env ) = runtime_environment
    {
        figment = figment.select( run_env.to_string() );
    }

    figment
        .merge( Env::prefixed( env_prefix ) )
        .extract::<T>()
        .expect( "Failed to load settings" )
}
