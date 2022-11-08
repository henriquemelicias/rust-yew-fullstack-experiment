use backend::Result;

use tracing::{event, Level};

fn main() -> Result<()>
{
    // Enable color_eyre.
    color_eyre::install()?;

    event!( Level::INFO, "Starting backend." );
    Ok( () )
}
