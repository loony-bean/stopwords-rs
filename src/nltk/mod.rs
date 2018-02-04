use std::str::Lines;

pub enum Language {
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Hungarian,
    Italian,
    Norwegian,
    Portuguese,
    Russian,
    Spanish,
    Swedish,
    Turkish,
}

pub fn load(language: Language) -> Lines<'static> {
    let text = match language {
        Language::Danish => include_str!("data/danish.txt"),
        Language::Dutch => include_str!("data/dutch.txt"),
        Language::English => include_str!("data/english.txt"),
        Language::Finnish => include_str!("data/finnish.txt"),
        Language::French => include_str!("data/french.txt"),
        Language::German => include_str!("data/german.txt"),
        Language::Hungarian => include_str!("data/hungarian.txt"),
        Language::Italian => include_str!("data/italian.txt"),
        Language::Norwegian => include_str!("data/norwegian.txt"),
        Language::Portuguese => include_str!("data/portuguese.txt"),
        Language::Russian => include_str!("data/russian.txt"),
        Language::Spanish => include_str!("data/spanish.txt"),
        Language::Swedish => include_str!("data/swedish.txt"),
        Language::Turkish => include_str!("data/turkish.txt"),
    };

    text.lines()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn english() {
        let words: HashSet<_> = load(Language::English).take(5).collect();
        assert_eq!(words, [ "i", "me", "my", "myself", "we" ].iter().cloned().collect());
    }

    #[test]
    fn russian() {
        let words: Vec<_> = load(Language::Russian).take(5).collect();
        assert_eq!(words, vec!("и", "в", "во", "не", "что"));
    }
}
