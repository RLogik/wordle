// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate wordle;
extern crate alloc;

use wordle::app;
use wordle::setup;


// ----------------------------------------------------------------
// MAIN METHOD
// ----------------------------------------------------------------

fn main() {
    // let args = cli::args::construct_arg_parser();
    // let spec = utils::read_yaml(PATH_TO_CONFIG);
    let words = setup::assets::get_data()
        .unwrap_or_else(|err| panic!("{}", err));
    let spec = setup::assets::get_config()
        .unwrap_or_else(|err| panic!("{}", err));
    let config = setup::config::set_config(&spec);
    let result = app::menus::main_menu(&config, &words);
    match result {
        Ok(_word) => { },
        Err(_) => { }
    }
}
