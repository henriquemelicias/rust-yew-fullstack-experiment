#![allow( unused )]

use derive_getters::Getters;
use settings::{ImportFigment, RuntimeEnvironmentType};

use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref GENERAL: GeneralConfigs =
        GeneralConfigs::import( "./configs/backend/general.toml", "backend_general_", None );
    pub static ref SERVER: ServerConfigs = ServerConfigs::import(
        "./configs/backend/server.toml",
        "backend_server_",
        Some( GENERAL.run_env() )
    );
    pub static ref LOGGER: LoggerConfigs = LoggerConfigs::import(
        "./configs/backend/logger.toml",
        "backend_logger_",
        Some( GENERAL.run_env() )
    );
}

#[derive(Debug, Deserialize, Getters)]
pub struct GeneralConfigs
{
    app_name: String,
    about:    String,
    run_env:  RuntimeEnvironmentType,
}

#[derive(Debug, Deserialize, Getters)]
pub struct ServerConfigs
{
    addr:       String,
    port:       u16,
    static_dir: String,
    assets_dir: String,
}

#[derive(Debug, Deserialize, Getters)]
pub struct LoggerConfigs
{
    log_level:         String,
    is_stdout_emitted: bool,
    is_file_emitted:   bool,
    files_directory:   Option<String>,
    files_prefix:      Option<String>,
}

impl ImportFigment<Self> for GeneralConfigs {}
impl ImportFigment<Self> for ServerConfigs {}
impl ImportFigment<Self> for LoggerConfigs {}
