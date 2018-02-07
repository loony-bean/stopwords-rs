#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use std::str::FromStr;

pub mod nltk;
pub mod spark;
pub mod sklearn;

pub use nltk::NLTK;
pub use spark::Spark;
pub use sklearn::SkLearn;

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

#[derive(Fail, PartialEq, Debug)]
#[fail(display = "Language not supported")]
pub struct LanguageError;

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
            _ => Err(LanguageError)
        }
    }
}

pub trait Stopwords {
    fn stopwords(language: Language) -> Option<&'static [&'static str]>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(Language::from_str("english").ok(), Some(Language::English));
        assert_eq!(Language::from_str("en").err(), Some(LanguageError));
        assert_eq!(Language::from_str("en").ok(), None);
    }
}