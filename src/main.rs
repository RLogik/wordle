// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate wordle;

use wordle::app;
use wordle::setup;

// ----------------------------------------------------------------
// MAIN METHOD
// ----------------------------------------------------------------

fn main() {
    // get assets
    let version = setup::assets::get_version();
    let words = setup::assets::get_data()
        .unwrap_or_else(|err| panic!("{}", err));
    let spec = setup::assets::get_config()
        .unwrap_or_else(|err| panic!("{}", err));
    // set config
    let config = setup::config::set_config(&spec, &version);
    // run methods
    app::menus::show_start_screen(&config);
    app::menus::main_menu(&config, &words);
    app::menus::show_end_screen(&config);
}
