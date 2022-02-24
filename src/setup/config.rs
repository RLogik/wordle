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
    pub size_of_wordle: i8,
    pub max_display_length: i8,
    pub max_length_for_best_optimisation: i64,
    pub hard_mode: bool,
}

pub static PATH_TO_CONFIG: &str = "src/setup/config.yml";

// ----------------------------------------------------------------
// Prompt confirm
// ----------------------------------------------------------------

pub fn set_config(spec: &Yaml) -> ConfigParams {
    return ConfigParams {
        size_of_wordle:
            utils::i64_to_i8(spec["settings"]["size-of-wordle"].as_i64().unwrap_or_else(|| {4}),),
        max_display_length:
            utils::i64_to_i8(spec["settings"]["max-display-length"].as_i64().unwrap_or_else(|| {100}),),
        max_length_for_best_optimisation:
            spec["settings"]["max-length-for-best-optimisation"].as_i64().unwrap_or_else(|| {500}),
        hard_mode:
            spec["settings"]["hard-mode"].as_bool().unwrap_or_else(|| {false}),
    };
}
