//! Normalizing text for search: index time and query time must agree.
//!
//! A tiny in-memory inverted index shows why normalization matters. The same
//! word typed with an Arabic keyboard layout, a Persian one, or with different
//! digits would otherwise never match.
//!
//! Run with: `cargo run --example search_indexing`

use std::collections::HashMap;

use persian_normalizer::{DigitStyle, Normalizer};

fn main() {
    // Use one normalizer for both indexing and querying. Latin digits are a
    // common choice for search because they are the easiest to type.
    let normalizer = Normalizer::builder().digits(DigitStyle::Latin).build();

    // The same book written three ways: Arabic letters and digits, Persian
    // letters with Latin digits, and an unrelated document.
    let documents = [
        ("doc-1", "كتاب شماره ١٢٣"),
        ("doc-2", "کتاب شماره 123"),
        ("doc-3", "دفتر شماره ۴۵۶"),
    ];

    // Build the index: normalized token -> ids of the documents containing it.
    let mut index: HashMap<String, Vec<&'static str>> = HashMap::new();
    for (id, text) in documents {
        let canonical = normalizer.normalize(text);
        for token in canonical.split_whitespace() {
            index.entry(token.to_owned()).or_default().push(id);
        }
    }

    // Search: normalize the query exactly like the documents.
    let search = |query: &str| -> Vec<&'static str> {
        let canonical = normalizer.normalize(query);
        index.get(&*canonical).cloned().unwrap_or_default()
    };

    // An Arabic-kaf query finds both the Arabic and the Persian spelling.
    let hits = search("كتاب");
    println!("كتاب  ->  {hits:?}");
    assert_eq!(hits, ["doc-1", "doc-2"]);

    // A query with Persian digits finds documents written with Arabic-Indic
    // and Latin digits.
    let hits = search("۱۲۳");
    println!("۱۲۳   ->  {hits:?}");
    assert_eq!(hits, ["doc-1", "doc-2"]);

    // Unknown words simply return nothing.
    let hits = search("خودکار");
    println!("خودکار  ->  {hits:?}");
    assert_eq!(hits, Vec::<&str>::new());
}
