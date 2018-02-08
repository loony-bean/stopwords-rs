//! This library provides stopwords datasets from popular text processing engines.
//!
//!
//! This could help reproducing results of text analysis pipelines written using different languages and tools.
//!
//! # Usage
//! ```toml
//! [dependencies]
//! stopwords = "0.1.0"
//! ```
//!
//! ```rust
//! extern crate stopwords;
//!
//! use std::collections::HashSet;
//! use stopwords::{Spark, Language, Stopwords};
//!
//! fn main() {
//!     let stops: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().collect();
//!     let mut tokens = vec!("brocolli", "is", "good", "to", "eat");
//!     tokens.retain(|s| !stops.contains(s));
//!     assert_eq!(tokens, vec!("brocolli", "good", "eat"));
//! }
//! ```
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use std::str::FromStr;

mod nltk;
mod spark;
mod sklearn;

pub use nltk::NLTK;
pub use spark::Spark;
pub use sklearn::SkLearn;

/// Supported languages. Each provider supports only a subset of this list.
///
/// You can parse lowercase English name of the language to construct enum variants.
///
/// ```rust
/// use std::str::FromStr;
/// use stopwords::Language;
///
/// assert_eq!(Language::from_str("english").ok(), Some(Language::English));
/// assert_eq!(Language::from_str("nepali").ok(), Some(Language::Nepali));
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Language {
    Arabic,
    Azerbaijani,
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Greek,
    Hungarian,
    Italian,
    Kazakh,
    Nepali,
    Norwegian,
    Portuguese,
    Romanian,
    Russian,
    Spanish,
    Swedish,
    Turkish,
}

/// Language parse error.
#[derive(Fail, PartialEq, Debug)]
#[fail(display = "Language '{}' is not supported", _0)]
pub struct LanguageError(String);

impl FromStr for Language {
    type Err = LanguageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "arabic" => Ok(Language::Arabic),
            "azerbaijani" => Ok(Language::Azerbaijani),
            "danish" => Ok(Language::Danish),
            "dutch" => Ok(Language::Dutch),
            "english" => Ok(Language::English),
            "finnish" => Ok(Language::Finnish),
            "french" => Ok(Language::French),
            "german" => Ok(Language::German),
            "greek" => Ok(Language::Greek),
            "hungarian" => Ok(Language::Hungarian),
            "italian" => Ok(Language::Italian),
            "kazakh" => Ok(Language::Kazakh),
            "nepali" => Ok(Language::Nepali),
            "norwegian" => Ok(Language::Norwegian),
            "portuguese" => Ok(Language::Portuguese),
            "romanian" => Ok(Language::Romanian),
            "russian" => Ok(Language::Russian),
            "spanish" => Ok(Language::Spanish),
            "swedish" => Ok(Language::Swedish),
            "turkish" => Ok(Language::Turkish),
            _ => Err(LanguageError(s.to_owned()))
        }
    }
}

/// Interface for getting stopwords from different providers.
pub trait Stopwords {
    fn stopwords(language: Language) -> Option<&'static [&'static str]>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(Language::from_str("english").ok(), Some(Language::English));
        assert_eq!(Language::from_str("en"), Err(LanguageError("en".to_owned())));
        assert_eq!(Language::from_str("en").ok(), None);
    }
}
