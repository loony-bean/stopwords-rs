extern crate stopwords;

use std::collections::HashSet;

fn main() {
    let stops: HashSet<_> = stopwords::nltk::load(stopwords::nltk::Language::English).collect();
    let text = "Brocolli is good to eat. My brother likes to eat good brocolli, but not my mother.";
    let words: HashSet<_> = text
        .to_lowercase()
        .split(' ')
        .filter(|s| !stops.contains(s))
        .map(|s| s
            .to_string()
            .replace(".", "")
            .replace(",", "")
            )
        .collect();
    println!("{:?}", words);
}
