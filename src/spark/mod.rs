use super::Language;
use super::Provider;

lazy_static! {
    static ref DANISH: Vec<&'static str> = include_str!("data/danish.txt").lines().collect();
    static ref DUTCH: Vec<&'static str> = include_str!("data/dutch.txt").lines().collect();
    static ref ENGLISH: Vec<&'static str> = include_str!("data/english.txt").lines().collect();
    static ref FINNISH: Vec<&'static str> = include_str!("data/finnish.txt").lines().collect();
    static ref FRENCH: Vec<&'static str> = include_str!("data/french.txt").lines().collect();
    static ref GERMAN: Vec<&'static str> = include_str!("data/german.txt").lines().collect();
    static ref HUNGARIAN: Vec<&'static str> = include_str!("data/hungarian.txt").lines().collect();
    static ref ITALIAN: Vec<&'static str> = include_str!("data/italian.txt").lines().collect();
    static ref NORWEGIAN: Vec<&'static str> = include_str!("data/norwegian.txt").lines().collect();
    static ref PORTUGUESE: Vec<&'static str> = include_str!("data/portuguese.txt").lines().collect();
    static ref RUSSIAN: Vec<&'static str> = include_str!("data/russian.txt").lines().collect();
    static ref SPANISH: Vec<&'static str> = include_str!("data/spanish.txt").lines().collect();
    static ref SWEDISH: Vec<&'static str> = include_str!("data/swedish.txt").lines().collect();
    static ref TURKISH: Vec<&'static str> = include_str!("data/turkish.txt").lines().collect();
}

pub struct Spark;

impl Provider for Spark {
    fn stopwords(language: Language) -> Option<&'static [&'static str]> {
        match language {
            Language::Danish => Some(&DANISH),
            Language::Dutch => Some(&DUTCH),
            Language::English => Some(&ENGLISH),
            Language::Finnish => Some(&FINNISH),
            Language::French => Some(&FRENCH),
            Language::German => Some(&GERMAN),
            Language::Hungarian => Some(&HUNGARIAN),
            Language::Italian => Some(&ITALIAN),
            Language::Norwegian => Some(&NORWEGIAN),
            Language::Portuguese => Some(&PORTUGUESE),
            Language::Russian => Some(&RUSSIAN),
            Language::Spanish => Some(&SPANISH),
            Language::Swedish => Some(&SWEDISH),
            Language::Turkish => Some(&TURKISH),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_count(language: Language, expected_len: usize) {
        let actual: Vec<_> = Spark::stopwords(language).unwrap().iter().cloned().collect();
        assert_eq!(actual.len(), expected_len);
    }

    fn assert_head(language: Language, expected: Vec<&str>) {
        let actual: Vec<_> = Spark::stopwords(language).unwrap().iter().cloned().take(5).collect();
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
