extern crate stopwords;

use std::collections::HashSet;
use stopwords::{Spark, Language, Stopwords};

fn main() {
    let stops: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().collect();
    let mut tokens = vec!("brocolli", "is", "good", "to", "eat");
    tokens.retain(|s| !stops.contains(s));
    println!("{:?}", tokens);
    assert_eq!(tokens, vec!("brocolli", "good", "eat"));
}
