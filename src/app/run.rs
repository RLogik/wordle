// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use crate::models::config::ConfigParams;
use crate::app::menus::basic::show_start_screen;
use crate::app::menus::basic::show_end_screen;
use crate::app::menus::root::create_main_menu;

// ----------------------------------------------------------------
// CONSTANTS
// ----------------------------------------------------------------

//

// ----------------------------------------------------------------
// Main cycle
// ----------------------------------------------------------------

pub fn main_cycle(config: &mut ConfigParams) {
    show_start_screen(&config);
    let main_menu = create_main_menu();
    main_menu.cycle(config);
    show_end_screen(&config);
}
