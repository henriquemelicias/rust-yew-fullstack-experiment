#![allow( unused )]

use common::settings::RuntimeEnvironmentType;
use derive_getters::Getters;

use common::settings::ImportFigment;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref GENERAL: GeneralConfigs =
        GeneralConfigs::import( "./configs/backend/general.toml", "general_", None );
    pub static ref SERVER: ServerConfigs = ServerConfigs::import(
        "./configs/backend/server.toml",
        "server_",
        Some( GENERAL.default_run_env() )
    );
    pub static ref LOGGER: LoggerConfigs = LoggerConfigs::import(
        "./configs/backend/logger.toml",
        "logger_",
        Some( GENERAL.default_run_env() )
    );
}

#[derive(Debug, Deserialize, Getters)]
pub struct GeneralConfigs
{
    app_name:        String,
    about:           String,
    default_run_env: RuntimeEnvironmentType,
}

#[derive(Debug, Deserialize, Getters)]
pub struct ServerConfigs
{
    addr:       String,
    port:       u16,
    static_dir: String,
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
