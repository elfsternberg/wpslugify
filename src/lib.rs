// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # Wordpress-style slugification
//!
//! This library provides a simple slugification algorithm that is a
//! close-as-possible direct port of the WordPress
//! [`sanitize_title_with_dashes()`](https://developer.wordpress.org/reference/functions/sanitize_title_with_dashes/)
//! function that WordPress uses to generate
//! [slugs](https://en.wikipedia.org/wiki/Clean_URL#Slug).
//!
//! Examples:
//!
//! ```
//! # use wpslugify::slugify;
//! assert_eq!(slugify("This is a test."), "this-is-a-test");
//! assert_eq!(slugify("This is a <script>alertslugify('!')</script> test"), "this-is-a-test");
//! assert_eq!(slugify("Excellent!!!1!1"), "excellent-1-1");
//! assert_eq!(slugify("user@example.com"), "user-example-com");
//! ```
//!
//! This slugification feature leaves UTF-8 that is within the Unicode
//! `{Alphabetic}` properties class intact.  Yes, there's really a
//! band with this name:
//!
//! ```
//! # use wpslugify::slugify;
//! assert_eq!(slugify("Töxic Tësticle Färm?"), "töxic-tësticle-färm");
//! ```

use lazy_static::lazy_static;
use regex::Regex;

// Rustfmt really wants to put some of my comments at the end of
// lines.
#[rustfmt::skip]
const TO_STRIP: [char; 28] = [
    '\u{00ad}', 
	// &iexcl and &iquest.
    '\u{00a1}', '\u{00bf}', 
	// Angle quotes.
    '\u{00ab}', '\u{00bb}', '\u{2039}', '\u{203a}', 
	// Curly quotes.
    '\u{2018}', '\u{2019}', '\u{201a}', '\u{201b}', '\u{201c}', '\u{201d}', '\u{201e}', '\u{201f}', '\u{2022}',
    // &copy, &reg, &deg, &hellip, and &trade.
    '\u{00a9}', '\u{00ae}', '\u{00b0}', '\u{2026}', '\u{2122}', 
	// Acute accents.
    '\u{00b4}', '\u{02ca}', '\u{0301}', '\u{0341}', 
	// Grave accent, macron, caron.
    '\u{0300}', '\u{0304}', '\u{030c}',
];

const TO_REWRITE: [&str; 10] = [
    "&nbsp;", "&#160;", "&ndash;", "&8211;", "&mdash;", "&#8212;", "\u{00a0}", "\u{2013}", "\u{2014}", "-",
];

macro_rules! big_collection {
    ( $ty:ident, $fnn:ident ) => {
        #[inline]
        fn $fnn() -> String {
            r"(".to_string()
                + &($ty
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(r"|"))
                + &r")+".to_string()
        }
    };
}

macro_rules! mk_workspace {
	( $v:ident, $( $tag:ident, $rep:literal, )* ) => {
		$(let $v = $tag.replace_all(&$v, $rep); )*
	}
}

macro_rules! extra_lazy {
	( $( $y:ident, $r:expr, )* ) => {
		lazy_static! {
			$(static ref $y: Regex = Regex::new($r).unwrap();)*
		}
	}
}

big_collection!(TO_STRIP, mk_strip);
big_collection!(TO_REWRITE, mk_rewrite);

const SCRIPT_AND_STYLE: &str = r"(<script[^>]*?>.*?</script>|<style[^>]*?>.*?</style>)";

/// Sanitize a string and return an array of the atomized words, all
/// lowercased.  This function is here because there are other uses
/// for slugified titles than just as slugs, and clients may want to
/// limit the length of a slug, remove stopwords or just "a|an|the"
/// language articles, or other modifications.
pub fn sanitize_and_split(title: &str) -> Vec<String> {
    #[rustfmt::skip]
    extra_lazy! {
        STRIP_DANGEROUS_TAGS, SCRIPT_AND_STYLE,
        REMOVE_TAGS, r"<[^>]*?>",
        REMOVE_SOFT_PUNCT, &mk_strip(),
        REWRITE_SOFT_PUNCT, &mk_rewrite(),
        REMOVE_REMAINING_ENTITIES, r"&.+?;",
		REWRITE_ACCEPTABLE_PUNCT, r"[\.\?!;:_@\r\n]+",
        REMOVE_REMAINING_PUNCT, r"[^%\p{Alphabetic}0-9 -]+",
    }

    let workspace = title.to_string().to_lowercase();

    #[rustfmt::skip]
    mk_workspace!(
		workspace,
        STRIP_DANGEROUS_TAGS, "",
        REMOVE_TAGS, "",
        REMOVE_SOFT_PUNCT, "",
        REWRITE_SOFT_PUNCT, "-",
        REMOVE_REMAINING_ENTITIES, "",
        REWRITE_ACCEPTABLE_PUNCT, "-",
		REMOVE_REMAINING_PUNCT, "",
    );

    workspace
        .split(|c| c == ' ' || c == '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Sanitize a string and return the string lowercased with a single
/// hyphen between the words.
pub fn slugify(title: &str) -> String {
    sanitize_and_split(title).join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES: [(&str, &str); 10] = [
        ("This is a test.", "this-is-a-test"),
        ("This is a <script>alert('!')</script> test", "this-is-a-test"),
        ("this is a <em>test</em>", "this-is-a-test"),
        ("        this    is --- a       <em>test</em>        ", "this-is-a-test"),
        ("Excellent!!!1!1", "excellent-1-1"),
        ("make\nit   work?", "make-it-work"),
        ("Töxic Tësticle Färm?", "töxic-tësticle-färm"), // Yes, that's a real band.
        ("  ----You--and--_-_me", "you-and-me"),
        ("Boys & Girls & Those Elsewhere", "boys-girls-those-elsewhere"),
        ("user@example.com", "user-example-com"),
    ];

    #[test]
    fn basic_checks() {
        for sample in SAMPLES.iter() {
            assert_eq!(slugify(sample.0), sample.1);
        }
    }
}
