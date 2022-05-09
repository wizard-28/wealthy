use std::borrow::Cow;
use std::collections::HashMap;

use regex::{Captures, Regex};

use crate::emoji_codes;

#[allow(dead_code)]
/// Replace emoji code in text.
fn emoji_replace<'a>(
    text: &'a str,
    default_variant: Option<&str>,
    emoji_regex: Option<Regex>,
) -> Cow<'a, str> {
    let variants = HashMap::from([("text", "\u{FE0E}"), ("emoji", "\u{FE0F}")]);

    let default_variant_code = default_variant
        .and_then(|variant| variants.get(variant))
        .copied()
        .unwrap_or("");

    emoji_regex
        .unwrap_or_else(|| Regex::new(r#"(:(\S*?)(?:(?:\-)(emoji|text))?:)"#).unwrap())
        .replace(text, |captures: &Captures| {
            let (emoji_code, emoji_name, variant) = (&captures[1], &captures[2], &captures.get(3));

            let emoji = match emoji_codes::EMOJI.get(&*emoji_name.to_lowercase()) {
                Some(emoji) => String::from(*emoji),
                None => return String::from(emoji_code),
            };
            let variant = match variant {
                Some(v) => v.as_str(),
                None => "",
            };
            let variant = variants.get(variant).unwrap_or(&default_variant_code);
            emoji + variant
        })
}

#[cfg(test)]
mod tests {
    use super::emoji_replace;

    #[test]
    fn test_replace() {
        assert_eq!(
            emoji_replace("This is an :atm_sign:", None, None),
            "This is an 🏧"
        );
    }

    #[test]
    fn test_variant() {
        assert_eq!(emoji_replace(":warning:", None, None), "⚠");
        assert_eq!(
            emoji_replace(":warning-text:", None, None),
            String::from("⚠") + "\u{FE0E}"
        );
        assert_eq!(
            emoji_replace(":warning-emoji:", None, None),
            String::from("⚠") + "\u{FE0F}"
        );
        assert_eq!(emoji_replace(":warning-foo:", None, None), ":warning-foo:");
    }
}
