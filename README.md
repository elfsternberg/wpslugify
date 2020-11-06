![Language: Rust](https://img.shields.io/badge/language-Rust-green.svg)
![Topic: Web](https://img.shields.io/badge/topic-Web-red.svg)
![Topic: Slugs](https://img.shields.io/badge/topic-slugs-red.svg)
![Library Status: Complete](https://img.shields.io/badge/status-Library_Complete-green.svg)

# WordPress's Slugify Function, in Rust.

[Slugs](https://en.wikipedia.org/wiki/Clean_URL#Slug) are the parts of a
URL generated from a hand-written or human-friendly title.  In order to
be both readable and search-engine optimized, slugs are generally
stripped of all frivolous punctuation and, since they're meant to be
used in URLs, all dangerous HTML and HTML entities are removed.

This algorithm closely matches the one used in Wordpress's
`formatting.php` file, `sanitize_title_with_dashes()`, although by
leveraging Rust's more powerful Regex library and Rust's native Unicode
features, I was able to get it done in a slightly smaller space.  It's
not spectacularly efficient, but it works.

In the library you'll find a series of tests that show off what it does.

## Reasons

Mostly, I needed a slugification function and the ones I found on
crates.io didn't thrill me.  URLs and Database are UTF-8 aware these
days, and the most popular ones either use
[`deunicode`](https://docs.rs/deunicode/1.1.1/deunicode/) or do other
sorts of mangling.

There are two functions in the library: one does the sanitization and
returns a Vec of the words after sanitization; the other uses hyphens to
join them together into a slug.  I needed the Vec available as I'm using
this library to create a trie of titles in a document store to support
autosuggest and autoreferencing features.

And the repetition of sanitizing stages gave me an excuse to reboot some
of my `macro_rules!` knowledge, since I hadn't used them much recently
and was starting to need them for my other project.

## LICENSE 

- WordPress™ is a trademark of Automattic, Inc.

This slugification library is Copyright [Elf
M. Sternberg](https://elfsternberg.com) (c) 2019, and licensed with the
Mozilla Public License vers. 2.0.  A copy of the license file is
included in the root folder.


