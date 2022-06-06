// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate yaml_rust;

use self::yaml_rust::Yaml;

use crate::core::utils;
use crate::models::config::ConfigParams;

// ----------------------------------------------------------------
// CONSTANTS
// ----------------------------------------------------------------

pub static PATH_TO_CONFIG: &str = "src/setup/config.yml";

// ----------------------------------------------------------------
// Prompt confirm
// ----------------------------------------------------------------

pub fn set_config(spec: &Yaml, version: &String) -> ConfigParams {
    let paths_empty = Vec::<Yaml>::new();
    let paths = spec["settings"]["paths"]
        .as_vec()
        .unwrap_or_else(|| &paths_empty)
        .iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect::<Vec<String>>();
    return ConfigParams {
        version: version.clone(),
        title:
            utils::attribute_or_default(spec["info"]["title"].as_str(), "<app name missing>").to_string(),
        url:
            utils::attribute_or_default(spec["info"]["url"].as_str(), "<url missing>").to_string(),
        notes:
            utils::attribute_or_default(spec["info"]["notes"].as_str(), "").to_string(),
        paths: paths,
        path_to_words:
            utils::attribute_or_default(spec["settings"]["path-to-words"].as_str(), "words.txt").to_string(),
        size_of_wordle:
            utils::i64_to_usize(utils::attribute_or_default(spec["settings"]["size-of-wordle"].as_i64(), 4)),
        max_display_length:
            utils::i64_to_usize(utils::attribute_or_default(spec["settings"]["max-display-length"].as_i64(), 100)),
        max_length_for_best_optimisation:
            utils::i64_to_usize(utils::attribute_or_default(spec["settings"]["max-length-for-best-optimisation"].as_i64(), 500)),
        hard_mode:
            utils::attribute_or_default(spec["settings"]["hard-mode"].as_bool(), false),
        anonymous_feedback:
            utils::attribute_or_default(spec["settings"]["hard-mode"].as_bool(), true),
    };
}
