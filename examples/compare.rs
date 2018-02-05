extern crate stopwords;

use stopwords::nltk::NLTK;
use stopwords::spark::Spark;
use stopwords::sklearn::SkLearn;

use stopwords::{Language, Provider};

use std::collections::HashSet;

fn main() {
    let nltk: HashSet<_> = NLTK::stopwords(Language::French).unwrap().iter().cloned().collect();
    let spark: HashSet<_> = Spark::stopwords(Language::French).unwrap().iter().cloned().collect();
    println!("-> NLTK VS Spark for French: {:?}", nltk.symmetric_difference(&spark));

    let nltk: HashSet<_> = NLTK::stopwords(Language::German).unwrap().iter().cloned().collect();
    let spark: HashSet<_> = Spark::stopwords(Language::German).unwrap().iter().cloned().collect();
    println!("-> NLTK VS Spark for German: {:?}", nltk.symmetric_difference(&spark));

    let nltk: HashSet<_> = NLTK::stopwords(Language::English).unwrap().iter().cloned().collect();
    let spark: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().cloned().collect();
    println!("-> NLTK VS Spark for English: {:?}", nltk.symmetric_difference(&spark));

    let sklearn: HashSet<_> = SkLearn::stopwords(Language::English).unwrap().iter().cloned().collect();
    let spark: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().cloned().collect();
    println!("-> SkLearn vs Spark for English: {:?}", sklearn.symmetric_difference(&spark));
}
