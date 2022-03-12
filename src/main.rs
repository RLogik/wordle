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
    // get assets +  config
    let version = setup::assets::get_version();
    let spec = setup::assets::get_config()
        .unwrap_or_else(|err| panic!("{}", err));
    let config = setup::config::set_config(&spec, &version);
    let words = setup::assets::get_data(&config)
        .unwrap_or_else(|err| panic!("{}", err));

    // run methods
    app::menus::show_start_screen(&config);
    app::menus::main_menu(&config, &words);
    app::menus::show_end_screen(&config);
}
