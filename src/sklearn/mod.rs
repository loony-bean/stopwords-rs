use super::Language;
use super::Provider;

lazy_static! {
    static ref ENGLISH: Vec<&'static str> = include_str!("data/english.txt").lines().collect();
}

pub struct SkLearn;

impl Provider for SkLearn {
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
