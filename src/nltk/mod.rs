use super::Language;
use super::Provider;

lazy_static! {
    static ref ARABIC: Vec<&'static str> = include_str!("data/arabic.txt").lines().collect();
    static ref AZERBAIJANI: Vec<&'static str> = include_str!("data/azerbaijani.txt").lines().collect();
    static ref DANISH: Vec<&'static str> = include_str!("data/danish.txt").lines().collect();
    static ref DUTCH: Vec<&'static str> = include_str!("data/dutch.txt").lines().collect();
    static ref ENGLISH: Vec<&'static str> = include_str!("data/english.txt").lines().collect();
    static ref FINNISH: Vec<&'static str> = include_str!("data/finnish.txt").lines().collect();
    static ref FRENCH: Vec<&'static str> = include_str!("data/french.txt").lines().collect();
    static ref GERMAN: Vec<&'static str> = include_str!("data/german.txt").lines().collect();
    static ref GREEK: Vec<&'static str> = include_str!("data/greek.txt").lines().collect();
    static ref HUNGARIAN: Vec<&'static str> = include_str!("data/hungarian.txt").lines().collect();
    static ref ITALIAN: Vec<&'static str> = include_str!("data/italian.txt").lines().collect();
    static ref KAZAKH: Vec<&'static str> = include_str!("data/kazakh.txt").lines().collect();
    static ref NEPALI: Vec<&'static str> = include_str!("data/nepali.txt").lines().collect();
    static ref NORWEGIAN: Vec<&'static str> = include_str!("data/norwegian.txt").lines().collect();
    static ref PORTUGUESE: Vec<&'static str> = include_str!("data/portuguese.txt").lines().collect();
    static ref ROMANIAN: Vec<&'static str> = include_str!("data/romanian.txt").lines().collect();
    static ref RUSSIAN: Vec<&'static str> = include_str!("data/russian.txt").lines().collect();
    static ref SPANISH: Vec<&'static str> = include_str!("data/spanish.txt").lines().collect();
    static ref SWEDISH: Vec<&'static str> = include_str!("data/swedish.txt").lines().collect();
    static ref TURKISH: Vec<&'static str> = include_str!("data/turkish.txt").lines().collect();
}

pub struct NLTK;

impl Provider for NLTK {
    fn stopwords(language: Language) -> Option<&'static [&'static str]> {
        match language {
            Language::Arabic => Some(&ARABIC),
            Language::Azerbaijani => Some(&AZERBAIJANI),
            Language::Danish => Some(&DANISH),
            Language::Dutch => Some(&DUTCH),
            Language::English => Some(&ENGLISH),
            Language::Finnish => Some(&FINNISH),
            Language::French => Some(&FRENCH),
            Language::German => Some(&GERMAN),
            Language::Greek => Some(&GREEK),
            Language::Hungarian => Some(&HUNGARIAN),
            Language::Italian => Some(&ITALIAN),
            Language::Kazakh => Some(&KAZAKH),
            Language::Nepali => Some(&NEPALI),
            Language::Norwegian => Some(&NORWEGIAN),
            Language::Portuguese => Some(&PORTUGUESE),
            Language::Romanian => Some(&ROMANIAN),
            Language::Russian => Some(&RUSSIAN),
            Language::Spanish => Some(&SPANISH),
            Language::Swedish => Some(&SWEDISH),
            Language::Turkish => Some(&TURKISH),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_count(language: Language, expected_len: usize) {
        let actual: Vec<_> = NLTK::stopwords(language).unwrap().iter().cloned().collect();
        assert_eq!(actual.len(), expected_len);
    }

    fn assert_head(language: Language, expected: Vec<&str>) {
        let actual: Vec<_> = NLTK::stopwords(language).unwrap().iter().cloned().take(5).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn arabic() {
        assert_count(Language::Arabic, 248);
        assert_head(Language::Arabic, vec!("إذ", "إذا", "إذما", "إذن", "أف"));
    }

    #[test]
    fn azerbaijani() {
        assert_count(Language::Azerbaijani, 165);
        assert_head(Language::Azerbaijani, vec!("a", "ad", "altı", "altmış", "amma"));
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
        assert_count(Language::English, 179);
        assert_head(Language::English, vec!("i", "me", "my", "myself", "we"));
    }

    #[test]
    fn finnish() {
        assert_count(Language::Finnish, 235);
        assert_head(Language::Finnish, vec!("olla", "olen", "olet", "on", "olemme"));
    }

    #[test]
    fn french() {
        assert_count(Language::French, 155);
        assert_head(Language::French, vec!("au", "aux", "avec", "ce", "ces"));
    }

    #[test]
    fn german() {
        assert_count(Language::German, 231);
        assert_head(Language::German, vec!("aber", "alle", "allem", "allen", "aller"));
    }

    #[test]
    fn greek() {
        assert_count(Language::Greek, 265);
        assert_head(Language::Greek, vec!("αλλα", "αν", "αντι", "απο", "αυτα"));
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
    fn kazakh() {
        assert_count(Language::Kazakh, 380);
        assert_head(Language::Kazakh, vec!("ах", "ох", "эх", "ай", "эй"));
    }

    #[test]
    fn nepali() {
        assert_count(Language::Nepali, 255);
        assert_head(Language::Nepali, vec!("छ", "र", "पनि", "छन्", "लागि"));
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
    fn romanian() {
        assert_count(Language::Romanian, 356);
        assert_head(Language::Romanian, vec!("a", "abia", "acea", "aceasta", "această"));
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
