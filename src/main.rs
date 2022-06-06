// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

pub mod app;
pub mod cli;
pub mod core;
pub mod display;
pub mod models;
pub mod setup;

use setup::assets;
use setup::config;
use app::run::main_cycle;

// ----------------------------------------------------------------
// MAIN METHOD
// ----------------------------------------------------------------

/// get assets +  config and run application
fn main() {
    let version = assets::get_version();
    let spec = assets::get_config()
        .unwrap_or_else(|err| panic!("{}", err));
    let mut config = config::set_config(&spec, &version);
    main_cycle(&mut config);
}
