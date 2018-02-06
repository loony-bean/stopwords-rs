#[macro_use]
extern crate lazy_static;

pub mod nltk;
pub mod spark;
pub mod sklearn;

pub use nltk::NLTK;
pub use spark::Spark;
pub use sklearn::SkLearn;

#[derive(Clone, Copy)]
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

pub trait Stopwords {
    fn stopwords(language: Language) -> Option<&'static [&'static str]>;
}
