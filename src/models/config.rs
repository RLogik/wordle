// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

//

// ----------------------------------------------------------------
// Structure
// ----------------------------------------------------------------

#[derive(Debug)]
pub struct ConfigParams {
    pub version: String,
    pub title: String,
    pub url: String,
    pub notes: String,
    pub path_to_words: String,
    pub paths: Vec<String>,
    pub size_of_wordle: usize,
    pub max_display_length: usize,
    pub max_length_for_best_optimisation: usize,
    pub hard_mode: bool,
    pub anonymous_feedback: bool,
}
