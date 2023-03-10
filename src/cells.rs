#![allow(dead_code)]
use std::num::NonZeroUsize;
use std::sync::Mutex;

use cached::proc_macro::cached;
use lazy_static::lazy_static;
use lru::LruCache;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

use crate::cell_widths::CELL_WIDTHS;

lazy_static! {
    static ref CACHE: Mutex<LruCache<String, u32>> =
        Mutex::new(LruCache::new(NonZeroUsize::new(4096).unwrap()));
}

/// Get number of cells required to display text.
///
/// # Arguments
///
/// * `text` - Text to display.
///
/// # Returns
///
/// The number of cells required to display text.
///
/// # Examples
///
/// ```
/// use wealthy::cells::cell_len;
///
/// assert_eq!(cell_len("abc"), 3);
/// assert_eq!(cell_len("愛"), 2);
/// assert_eq!(cell_len("👪"), 2);
/// ```
pub fn cell_len(text: &str) -> u32 {
    let mut cache = CACHE.lock().expect("Unable to lock LRUCache!");
    let cached_result = cache.get(text);

    if let Some(cached_result) = cached_result {
        return *cached_result;
    }

    let total_size = text.chars().map(get_character_cell_size).sum();

    if UnicodeSegmentation::graphemes(text, true).count() <= 512 {
        cache.put(text.into(), total_size);
    }

    total_size
}

/// Get cell size of a character.
///
/// # Arguments
///
/// * `character` - A single character
///
/// # Returns
///
/// Number of cells (0, 1 or 2) occupied by that character.
///
/// # Examples
///
/// ```
/// use wealthy::cells::get_character_cell_size;
///
/// assert_eq!(get_character_cell_size('a'), 1);
/// assert_eq!(get_character_cell_size('愛'), 2);
/// assert_eq!(get_character_cell_size('👪'), 2);
/// ```
#[cached(size = 4096)]
pub fn get_character_cell_size(character: char) -> u32 { get_codepoint_cell_size(character as u32) }

/// Get cell size of a codepoint.
///
/// # Arguments
///
/// * `codepoint` - A codepoint.
///
/// # Returns
///
/// Number of cells (0, 1 or 2) occupied by that codepoint.
#[cached(size = 4096)]
fn get_codepoint_cell_size(codepoint: u32) -> u32 {
    let table = CELL_WIDTHS;
    let mut lower_bound = 0;
    let mut upper_bound = table.len() - 1;

    let mut index = (lower_bound + upper_bound) / 2;

    loop {
        let (start, end, width) = table[index];

        if codepoint < start {
            upper_bound = index - 1;
        } else if codepoint > end {
            lower_bound = index + 1;
        } else {
            return width.unwrap_or(0);
        }

        if upper_bound < lower_bound {
            break 1;
        }

        index = (lower_bound + upper_bound) / 2;
    }
}

/// Set the length of a string to fit within given number of cells.
#[allow(clippy::missing_panics_doc)]
pub fn set_cell_size(text: String, total: u32) -> String {
    // Regex to match sequence of the most common character ranges.
    if Regex::new(r#"^[\u0020-\u006f\u00a0\u02ff\u0370-\u0482]*$"#)
        .unwrap()
        .is_match(&text)
    {
        let size: u32 = UnicodeSegmentation::graphemes(text.as_str(), true)
            .count()
            .try_into()
            .unwrap();

        if size < total {
            return text + &" ".repeat((total - size) as usize);
        }

        return text[..total as usize].into();
    }

    if total == 0 {
        return String::new();
    }

    let cell_size = cell_len(&text);

    if cell_size == total {
        return text;
    }

    if cell_size < total {
        return text + &" ".repeat((total - cell_size) as usize);
    }

    let mut start = 0;
    let mut end = UnicodeSegmentation::graphemes(text.as_str(), true).count();

    loop {
        let pos = (start + end) / 2;
        let before = &UnicodeSegmentation::graphemes(text.as_str(), true)
            .take(pos + 1)
            .collect::<String>();
        let before_len = cell_len(before);

        let graphemes = UnicodeSegmentation::graphemes(before.as_str(), true);

        if before_len == total + 1 && cell_len(graphemes.clone().last().unwrap()) == 2 {
            return format!(
                "{} ",
                &graphemes
                    .clone()
                    .take(&graphemes.count() - 1)
                    .collect::<String>()
            );
        }
        if before_len == total {
            return before.into();
        }

        if before_len > total {
            end = pos;
        } else {
            start = pos;
        }
    }
}

/// Break text in equal (cell) length strings, returning the characters in
/// reverse order.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::missing_panics_doc)]
pub fn chop_cells(text: &str, max_size: u32, position: Option<u32>) -> Vec<String> {
    let characters = text
        .chars()
        .map(|character| (character, get_character_cell_size(character)));

    let mut lines: Vec<Vec<char>> = vec![vec![]];
    let mut total_size = position.unwrap_or(0);

    for (character, size) in characters.rev() {
        if total_size + size > max_size {
            lines.push(vec![character]);
            total_size = size;
        } else {
            total_size += size;
            lines.last_mut().unwrap().push(character);
        }
    }

    lines
        .into_iter()
        .map(|line| line.into_iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_pass_by_value)]

    use rstest::rstest;

    use super::*;

    fn test_cell_len_long_string() { assert_eq!(cell_len(&"abc".repeat(200)), 3 * 200) }

    #[rstest]
    #[case("foo", 0, "")]
    #[case("f", 0, "")]
    #[case("", 0, "")]
    #[case("😽😽", 0, "")]
    #[case("foo", 2, "fo")]
    #[case("foo", 3, "foo")]
    #[case("foo", 4, "foo ")]
    #[case("😽😽", 4, "😽😽")]
    #[case("😽😽", 3, "😽 ")]
    #[case("😽😽", 2, "😽")]
    #[case("😽😽", 1, " ")]
    #[case("😽😽", 5, "😽😽 ")]
    fn test_set_cell_size(#[case] test_case: &str, #[case] size: u32, #[case] result: &str) {
        assert_eq!(
            set_cell_size(test_case.into(), size),
            result,
            "set_cell_size({test_case}, {size})"
        );
    }
    #[test]
    fn test_set_cell_size_infinite() {
        for size in 0..38 {
            assert_eq!(
                cell_len(&set_cell_size(
                    "เป็นเกมที่ต้องมีความอดทนมากที่สุดตั้งเเต่เคยเล่นมา".into(),
                    size
                )),
                size
            );
        }
    }

    #[test]
    fn test_chop_cells() {
        assert_eq!(
            chop_cells(
                "这是对亚洲语言支持的测试。面对模棱两可的想法，拒绝猜测的诱惑。",
                8,
                None
            ),
            vec![
                "。惑诱的",
                "测猜绝拒",
                "，法想的",
                "可两棱模",
                "对面。试",
                "测的持支",
                "言语洲亚",
                "对是这"
            ]
        );
    }
}
