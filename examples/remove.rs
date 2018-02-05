extern crate stopwords;

use std::collections::HashSet;

use stopwords::spark::Spark;
use stopwords::{Language, Provider};

fn main() {
    let stops: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().collect();
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
