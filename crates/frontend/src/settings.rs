#![allow( clippy::ref_option_ref )]
#![allow( unused )]

use common::settings::ImportFigment;
pub use common::settings::RuntimeEnvironmentType;
use derive_getters::Getters;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref GENERAL: GeneralConfigs = GeneralConfigs::import( "./logs/frontend/general.toml", "general_", None );
    pub static ref LOGGER: LoggerConfigs = LoggerConfigs::import(
        "./logs/frontend/logger.toml",
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
pub struct LoggerConfigs
{
    log_level:               &'static str,
    is_file_emitted:         bool,
    is_stdout_emitted:       bool,
    is_wasm_console_emitted: bool,

    log_file_path: Option<&'static str>,
}

impl ImportFigment<Self> for GeneralConfigs {}
impl ImportFigment<Self> for LoggerConfigs {}
