use std::vec;

use heraclitus_compiler::prelude::*;
use crate::utils::metadata::ParserMetadata;
use crate::modules::expression::expr::Expr;

pub mod bool;
pub mod number;
pub mod text;
pub mod void;
pub mod command;

pub fn parse_interpolated_region(meta: &mut ParserMetadata, letter: char) -> Result<(Vec<String>, Vec<Expr>), ErrorDetails> {
    let mut strings = vec![];
    let mut interps = vec![];
    // Handle full string
    if let Ok(word) = token_by(meta, |word| word.starts_with(letter) && word.ends_with(letter) && word.len() > 1) {
        let stripped = word.chars().take(word.len() - 1).skip(1).collect::<String>();
        strings.push(stripped);
        Ok((strings, interps))
    }
    else {
        let mut is_interp = false;
        // Initialize string
        strings.push(token_by(meta, |word| word.starts_with(letter))?);
        // Factor rest of the interpolation
        while let Some(token) = meta.get_current_token() {
            // Track interpolations
            match token.word.as_str() {
                "{" => is_interp = true,
                "}" => is_interp = false,
                // Manage inserting strings and intrpolations
                _ => if is_interp {
                    let mut expr = Expr::new();
                    syntax(meta, &mut expr)?;
                    interps.push(expr);
                    // TODO: [H50] In the next release of Heraclitus
                    // Change this line to `meta.offset_index(-1)`
                    meta.set_index(meta.get_index() - 1);
                }
                else {
                    strings.push(token.word.clone());
                    if token.word.ends_with(letter) {
                        meta.increment_index();
                        return Ok((strings, interps))
                    }
                }
            }
            meta.increment_index();
        }
        Err(ErrorDetails::from_metadata(meta))
    }
}