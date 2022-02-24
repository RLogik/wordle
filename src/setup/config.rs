// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate yaml_rust;

use self::yaml_rust::Yaml;

use crate::core::utils;

// ----------------------------------------------------------------
// Structure
// ----------------------------------------------------------------

pub struct ConfigParams {
    pub version: String,
    pub title: String,
    pub url: String,
    pub size_of_wordle: i8,
    pub max_display_length: i8,
    pub max_length_for_best_optimisation: i64,
    pub hard_mode: bool,
}

pub static PATH_TO_CONFIG: &str = "src/setup/config.yml";

// ----------------------------------------------------------------
// Prompt confirm
// ----------------------------------------------------------------

pub fn set_config(spec: &Yaml, version: &String) -> ConfigParams {
    return ConfigParams {
        version: version.clone(),
        title:
            utils::attribute_or_default(spec["info"]["title"].as_str(), "<app name missing>").to_string(),
        url:
            utils::attribute_or_default(spec["info"]["url"].as_str(), "<url missing>").to_string(),
        size_of_wordle:
            utils::i64_to_i8(utils::attribute_or_default(spec["settings"]["size-of-wordle"].as_i64(), 4)),
        max_display_length:
            utils::i64_to_i8(utils::attribute_or_default(spec["settings"]["max-display-length"].as_i64(), 100)),
        max_length_for_best_optimisation:
            utils::attribute_or_default(spec["settings"]["max-length-for-best-optimisation"].as_i64(), 500),
        hard_mode:
            utils::attribute_or_default(spec["settings"]["hard-mode"].as_bool(), false),
    };
}
