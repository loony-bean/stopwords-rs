use std::str::Lines;

pub enum Language {
    English,
}

pub fn stopwords(language: Language) -> Lines<'static> {
    let text = match language {
        Language::English => include_str!("data/english.txt"),
    };

    text.lines()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn english() {
        let words: HashSet<_> = stopwords(Language::English).take(5).collect();
        assert_eq!(words, [ "a", "about", "above", "across", "after" ].iter().cloned().collect());
    }
}
