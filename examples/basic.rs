//! Minimal usage: normalize strings with the default configuration.
//!
//! Run with: `cargo run --example basic`

use std::borrow::Cow;

use persian_normalizer::normalize;

fn main() {
    // Arabic kaf (ك) and Arabic-Indic digits (١٢٣) become their Persian forms.
    let messy = "كتاب ١٢٣";
    let clean = normalize(messy);
    println!("{messy}  ->  {clean}");
    assert_eq!(clean, "کتاب ۱۲۳");

    // The result is a `Cow<str>`: text that needs changes is returned as an
    // owned `String`...
    assert!(matches!(clean, Cow::Owned(_)));

    // ...while text that is already normalized is returned borrowed, so
    // normalizing clean input costs no allocation.
    let already_clean = normalize("کتاب ۱۲۳");
    println!("already normalized  ->  {already_clean}");
    assert!(matches!(already_clean, Cow::Borrowed(_)));

    // Call `into_owned()` when you need to keep the result as a `String`.
    let owned: String = normalize(messy).into_owned();
    assert_eq!(owned, "کتاب ۱۲۳");
}
