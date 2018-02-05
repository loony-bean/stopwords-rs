#[macro_use]
extern crate lazy_static;

pub mod nltk;
pub mod spark;
pub mod sklearn;

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

pub trait Provider {
    fn stopwords(language: Language) -> Option<&'static [&'static str]>;
}
