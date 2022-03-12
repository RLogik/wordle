// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate rust_embed;
extern crate yaml_rust;

use std::io;

use self::rust_embed::RustEmbed;
use self::yaml_rust::Yaml;

use crate::core::utils;
use crate::setup::config::ConfigParams;

// ----------------------------------------------------------------
// Assets
// ----------------------------------------------------------------

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "data.txt"]
struct AssetsData;

#[derive(RustEmbed)]
#[folder = "src/setup"]
#[include = "*.yml"]
struct AssetsConfig;

#[derive(RustEmbed)]
#[folder = "dist"]
#[include = "VERSION"]
struct AssetsVersion;



// ----------------------------------------------------------------
// Methods - get data
// ----------------------------------------------------------------

pub fn get_data(config: &ConfigParams) -> Result<Vec<String>, io::Error> {
    let file_path = config.path_to_words.as_str();
    return utils::read_from_embedded_file(AssetsData::get(file_path).unwrap())
        .and_then(|contents| Ok(read_words(&contents)));
}

pub fn read_words(contents: &String) -> Vec<String> {
    let lines = utils::read_contents_to_lines(&contents, true);
    let matcher = utils::construct_regex(r"^\w+$");
    let mut words = Vec::<String>::new();
    for line in lines {
        let word = line.trim();
        if matcher.is_match(&word) {
            words.push(word.to_string());
        }
    }
    return words;
}

// ----------------------------------------------------------------
// Methods - get config
// ----------------------------------------------------------------

pub fn get_config() -> Result<Yaml, io::Error> {
    return utils::read_from_embedded_file(AssetsConfig::get("config.yml").unwrap())
        .and_then(|source| {
            return utils::read_contents_to_yaml(&source);
        });
}

// ----------------------------------------------------------------
// Methods - get config
// ----------------------------------------------------------------

pub fn get_version() -> String {
    return utils::read_from_embedded_file(AssetsVersion::get("VERSION").unwrap())
        .map(|contents| contents.trim().to_string())
        .unwrap_or_else(|_| "x.y.z".to_string());
}
