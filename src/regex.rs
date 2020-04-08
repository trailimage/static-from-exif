//! Common regular expressions
use lazy_static::*;
use regex::{Regex, RegexBuilder};

lazy_static! {
    pub static ref CURLY_QUOTE: Regex = Regex::new("[“”]").unwrap();
    pub static ref OPEN_QUOTE: Regex = Regex::new(r"^\s*“").unwrap();
    pub static ref NON_WORD: Regex = Regex::new(r"\W").unwrap();

    pub static ref LINE_BREAK: Regex = Regex::new(r"\r*\n").unwrap();

    pub static ref NEW_LINE: Regex = Regex::new(r"(\r\n|\n|\r)").unwrap();

    /// Capture footnoted word and superscript. Match superscripts but don't
    /// match atomic numbers.
    pub static ref FOOTNOTE_NUMBER: Regex =
        Regex::new(r"([^/\s])([⁰¹²³⁴⁵⁶⁷⁸⁹]+)\B").unwrap();

    /// Footnote text preceded by three underscores
    pub static ref FOOTNOTE_TEXT: Regex =
        Regex::new(r"(^|[\r\n]+)_{3}[\r\n]*(?P<notes>[\s\S]+)$").unwrap();

    /// Long quote followed by line break or end of text
    pub static ref BLOCK_QUOTE: Regex =
        Regex::new(r"(\r\n|\r|\n|^)\s*(?P<quote>“[^”]{200,}”[⁰¹²³⁴⁵⁶⁷⁸⁹]*)\s*(\r\n|\r|\n|$)")
            .unwrap();

    pub static ref TRAILING_SPACE: Regex = Regex::new(r"[\r\n\s]*$").unwrap();

    pub static ref EMPTY_P_TAG: Regex = Regex::new(r"<p[^>]*></p>").unwrap();

    /// Capture URL and label
    pub static ref ANCHOR_TAG: Regex = Regex::new(r#"<a href=["'](?P<url>[^"']+)['"][^>]*>(?P<label>[^<]+)</a>"#).unwrap();

    /// Whether text contains a poem. Exclude dialog by negating comma or
    /// question mark before closing quote unless it's footnoted. Capture
    /// leading space and poem body.
    ///
    /// Match any character but new lines:
    /// ```
    /// [^\r\n]
    /// ```
    ///
    /// Do not match punctuation followed by closing quote (negative look-
    /// ahead) unless the quote mark is followed by a superscript number:
    /// ```
    /// (?![\.,!?]”[^⁰¹²³⁴⁵⁶⁷⁸⁹])
    /// ```
    ///
    /// Match stops at end of text (`$`) or when there are one or more
    /// new-lines (`\r\n`):
    /// ```
    /// ([\r\n]+|$)
    /// ```
    //pub static ref POETRY: Regex = Regex::new(r"(^|[\r\n]+)((([^\r\n](?![\.,!?]”[^⁰¹²³⁴⁵⁶⁷⁸⁹])){4,80}([\r\n]+|$)){3,})").unwrap();
    pub static ref POETRY: Regex = Regex::new(r"(^|[\r\n]+)(([^\r\n]{4,80}([\r\n]+|$)){3,})").unwrap();

    // TODO: this was needed because Flickr collapsed spaces -- validate
    pub static ref POEM_INDENT: Regex = Regex::new(r"· · ").unwrap();

    /// Whether text is entirely a poem. Uses dashes above and below to set off
    /// full poem — hacky but haven't figured out better way.
    pub static ref ALL_POEM: Regex = Regex::new(r"^\-[\r\n]*(([^\r\n]){3,100}([\r\n])+){3,}\-[\r\n]*$").unwrap();

    /// Full poems have single dash above and below
    pub static ref POEM_DELIMITER: Regex = Regex::new(r"(^|[\r\n]+)-([\r\n]+|$)").unwrap();

    /// Match the first HTML paragraph if it's short and contains a quote
    pub static ref QUIP: Regex = Regex::new(r"(<p>)(“[^<]{4,80}</p>)").unwrap();

    /// Text begins with three lines 5–100 characters long
    pub static ref BEGINS_WITH_HAIKU: Regex = RegexBuilder::new(r"^([ \w]{5,100})[\r\n]+([ \w]{5,100})[\r\n]+([ \w]{5,100})([\r\n]{2}|$)+")
        .size_limit(50_000_000)
        .build()
        .unwrap();

    /// Text is three lines 5–100 characters long
    pub static ref HAIKU: Regex = RegexBuilder::new(r"^([ \w]{5,100})[\r\n]+([ \w]{5,100})[\r\n]+([ \w]{5,100})$")
        .size_limit(50_000_000)
        .build()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::BLOCK_QUOTE;

    const NEW_LINE: &str = "\r\n";
    const LIPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    #[test]
    fn block_quote_test() {
        let quote =
            format!("“{txt}{cr}“{txt}{cr}”{cr}", txt = LIPSUM, cr = NEW_LINE);
        assert!(BLOCK_QUOTE.is_match(&quote));

        // interrupted block quote should not match
        let quote = format!(
            "“{txt},” he said, “{txt}{cr}",
            txt = LIPSUM,
            cr = NEW_LINE
        );
        assert!(!BLOCK_QUOTE.is_match(&quote));
    }
}
