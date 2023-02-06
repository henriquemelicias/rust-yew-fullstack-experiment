use backend::settings;

use clap::Parser;
use smartstring::alias::String as SmartString;

// Command line arguments interface.
#[derive(Parser, Debug)]
#[clap( name = settings::GENERAL.app_name().as_str(), about = settings::GENERAL.about().as_str() )]
struct CliArgs
{
    /// Set the listen addr.
    #[clap( short = 'a', long = "addr", default_value = settings::SERVER.addr().as_str() )]
    addr: SmartString,

    /// Set the listen port.
    #[clap( short = 'p', long = "port", default_value_t =  *settings::SERVER.port() )]
    port: u16,

    /// Set the log level.
    /// Possible values: trace, debug, info, warn, error.
    #[clap( short = 'l', long = "log-level", default_value = settings::LOGGER.log_level().as_str() )]
    log_level: SmartString,

    /// Set the static files directory
    #[clap( short = 's', long = "static-dir", default_value = settings::SERVER.static_dir().as_str() )]
    static_dir: SmartString,

    /// Set the assets files directory
    #[clap( long = "assets-dir", default_value = settings::SERVER.assets_dir().as_str() )]
    assets_dir: SmartString,
}

fn main() -> backend::Result<()>
{
    // Enable color_eyre.
    color_eyre::install()?;

    // Parse the command line arguments.
    let cli_args = CliArgs::parse();

    // Tracing logs.
    let ( _maybe_stdio_writer_guard, _maybe_file_writer_guard ) = backend::start_logs( &cli_args.log_level );

    tracing::info!( "Starting backend." );

    backend::start_server(
        &cli_args.addr,
        cli_args.port,
        &cli_args.static_dir,
        &cli_args.assets_dir,
    );

    Ok( () )
}
