use std::str::Lines;

pub enum Language {
    English,
    // Danish,
    // Dutch,
    // Finnish,
    // French,
    // German,
    // Hungarian,
    // Italian,
    // Norwegian,
    // Portuguese,
    Russian,
    // Spanish,
    // Swedish,
    // Turkish,
}

pub fn nltk(language: Language) -> Lines<'static> {
    let text = match language {
        Language::English => include_str!("data/nltk/english.txt"),
        Language::Russian => include_str!("data/nltk/russian.txt"),
    };

    text.lines()
}

pub fn sklearn(language: Language) -> Lines<'static> {
    let text = match language {
        Language::English => include_str!("data/sklearn/english.txt"),
        Language::Russian => include_str!("data/sklearn/russian.txt"),
    };

    text.lines()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn sklearn_english() {
        let words: HashSet<_> = sklearn(Language::English).take(5).collect();
        assert_eq!(words, [ "a", "about", "above", "across", "after" ].iter().cloned().collect());
    }

    #[test]
    fn nltk_english() {
        let words: HashSet<_> = nltk(Language::English).take(5).collect();
        assert_eq!(words, [ "i", "me", "my", "myself", "we" ].iter().cloned().collect());
    }

    #[test]
    fn nltk_russian() {
        let words: Vec<_> = nltk(Language::Russian).take(5).collect();
        assert_eq!(words, vec!("и", "в", "во", "не", "что"));
    }
}
