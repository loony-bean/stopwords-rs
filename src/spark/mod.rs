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

pub fn stopwords(language: Language) -> Lines<'static> {
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

    fn assert_count(language: Language, expected_len: usize) {
        let actual: Vec<_> = stopwords(language).collect();
        assert_eq!(actual.len(), expected_len);
    }

    fn assert_head(language: Language, expected: Vec<&str>) {
        let actual: Vec<_> = stopwords(language).take(5).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn danish() {
        assert_count(Language::Danish, 94);
        assert_head(Language::Danish, vec!("og", "i", "jeg", "det", "at"));
    }

    #[test]
    fn dutch() {
        assert_count(Language::Dutch, 101);
        assert_head(Language::Dutch, vec!("de", "en", "van", "ik", "te"));
    }

    #[test]
    fn english() {
        assert_count(Language::English, 181);
        assert_head(Language::English, vec!("i", "me", "my", "myself", "we"));
    }

    #[test]
    fn finnish() {
        assert_count(Language::Finnish, 235);
        assert_head(Language::Finnish, vec!("olla", "olen", "olet", "on", "olemme"));
    }

    #[test]
    fn french() {
        assert_count(Language::French, 156);
        assert_head(Language::French, vec!("au", "aux", "avec", "ce", "ces"));
    }

    #[test]
    fn german() {
        assert_count(Language::German, 231);
        assert_head(Language::German, vec!("aber", "alle", "allem", "allen", "aller"));
    }

    #[test]
    fn hungarian() {
        assert_count(Language::Hungarian, 199);
        assert_head(Language::Hungarian, vec!("a", "ahogy", "ahol", "aki", "akik"));
    }

    #[test]
    fn italian() {
        assert_count(Language::Italian, 279);
        assert_head(Language::Italian, vec!("ad", "al", "allo", "ai", "agli"));
    }

    #[test]
    fn norwegian() {
        assert_count(Language::Norwegian, 176);
        assert_head(Language::Norwegian, vec!("og", "i", "jeg", "det", "at"));
    }

    #[test]
    fn portuguese() {
        assert_count(Language::Portuguese, 203);
        assert_head(Language::Portuguese, vec!("de", "a", "o", "que", "e"));
    }

    #[test]
    fn russian() {
        assert_count(Language::Russian, 151);
        assert_head(Language::Russian, vec!("и", "в", "во", "не", "что"));
    }

    #[test]
    fn spanish() {
        assert_count(Language::Spanish, 313);
        assert_head(Language::Spanish, vec!("de", "la", "que", "el", "en"));
    }

    #[test]
    fn swedish() {
        assert_count(Language::Swedish, 114);
        assert_head(Language::Swedish, vec!("och", "det", "att", "i", "en"));
    }

    #[test]
    fn turkish() {
        assert_count(Language::Turkish, 53);
        assert_head(Language::Turkish, vec!("acaba", "ama", "aslında", "az", "bazı"));
    }
}
