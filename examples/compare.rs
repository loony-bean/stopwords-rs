extern crate stopwords;

use stopwords::{nltk, spark, sklearn};

use std::collections::HashSet;

fn main() {
    let nltk: HashSet<_> = nltk::stopwords(nltk::Language::French).collect();
    let spark: HashSet<_> = spark::stopwords(spark::Language::French).collect();
    println!("-> NLTK VS Spark for French: {:?}", nltk.symmetric_difference(&spark));

    let nltk: HashSet<_> = nltk::stopwords(nltk::Language::German).collect();
    let spark: HashSet<_> = spark::stopwords(spark::Language::German).collect();
    println!("-> NLTK VS Spark for German: {:?}", nltk.symmetric_difference(&spark));

    let nltk: HashSet<_> = nltk::stopwords(nltk::Language::English).collect();
    let spark: HashSet<_> = spark::stopwords(spark::Language::English).collect();
    println!("-> NLTK VS Spark for English: {:?}", nltk.symmetric_difference(&spark));

    let sklearn: HashSet<_> = sklearn::stopwords(sklearn::Language::English).collect();
    let spark: HashSet<_> = spark::stopwords(spark::Language::English).collect();
    println!("-> SkLearn vs Spark for English: {:?}", sklearn.symmetric_difference(&spark));
}
