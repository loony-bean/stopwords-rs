use super::Language;
use super::Stopwords;

lazy_static! {
    static ref ENGLISH: Vec<&'static str> = include_str!("data/english.txt").lines().collect();
}

/// Data from [scikit-learn](http://scikit-learn.org) - Python machine learning library.
pub struct SkLearn;

impl Stopwords for SkLearn {
    /// Stopwords provided by vectorizers (`TfidfVectorizer`, `HashingVectorizer`, etc).
    ///
    /// The only language available is English.
    fn stopwords(language: Language) -> Option<&'static [&'static str]> {
        match language {
            Language::English => Some(&ENGLISH),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn english() {
        let words: HashSet<_> = SkLearn::stopwords(Language::English).unwrap().iter().take(5).collect();
        assert_eq!(words, [ "a", "about", "above", "across", "after" ].iter().collect());
    }
}
