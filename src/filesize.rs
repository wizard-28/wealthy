#![allow(dead_code)]

use separator::Separatable;

/// Convert a filesize in to a string (powers of 1000, SI prefixes).
///
/// In this convention, `1000 B = 1 kB`.
///
/// This is typically the format used to advertise the storage
/// capacity of USB flash drives and the like (*256 MB* meaning
/// actually a storage capacity of more than *256 000 000 B*),
/// or used by **Mac OS X** since v10.6 to report file sizes.
///
/// # Arguments
///
/// * `size` - A file size.
/// * `precision` -  The number of decimal places to include (default = 1).
/// * `separator` - The string to separate the value from the units (default = "
///   ").
///
/// # Returns
///
/// A string containing a abbreviated file size and units.
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub(crate) fn decimal(size: f32, precision: Option<usize>, separator: Option<&str>) -> String {
    let base = 1000.0;

    if (size - 1.0).abs() < f32::EPSILON {
        return String::from("1 byte");
    } else if size < base {
        return format!("{} bytes", size.separated_string());
    }

    let suffixes = ["kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut unit = 0.0;
    let mut evaluated_suffix = " ";
    for (i, suffix) in suffixes.into_iter().enumerate() {
        unit = base.powi(i as i32 + 2);
        evaluated_suffix = suffix;
        if size < unit {
            break;
        }
    }
    let precision = precision.unwrap_or(1);
    let separator = separator.unwrap_or(" ");

    let magnitude = format!("{:.precision$}", base * size / unit)
        .parse::<f32>()
        .unwrap();

    let mut magnitude_separated_string = magnitude.separated_string();
    // HACK: Workaround for `separator` crate's float truncating behavior.
    if magnitude.fract() == 0.0 && precision != 0 {
        magnitude_separated_string.push_str(".0");
    }

    format!("{magnitude_separated_string}{separator}{evaluated_suffix}")
}

/// Pick a unit and suffix for the given size.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn pick_unit_and_suffix(size: u32, suffixes: &[&str], base: u32) -> (u32, String) {
    let mut unit = 0_u32;
    let mut evaluated_suffix = "";
    for (i, suffix) in suffixes.iter().enumerate() {
        unit = base.pow(i as u32);
        evaluated_suffix = suffix;

        if size < unit * base {
            break;
        }
    }

    (unit, String::from(evaluated_suffix))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0.0, None, None, "0 bytes")]
    #[case(1.0, None, None, "1 byte")]
    #[case(2.0, None, None, "2 bytes")]
    #[case(1000.0, None, None, "1.0 kB")]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    #[case(1.5 * (1000 * 1000) as f32, None, None, "1.5 MB")]
    #[case(0.0, Some(2), None, "0 bytes")]
    #[case(1111.0, Some(0), None, "1 kB")]
    #[case(1111.0, Some(1), None, "1.1 kB")]
    #[case(1111.0, Some(2), None, "1.11 kB")]
    #[case(1111.0, None, Some(""), "1.1kB")]
    fn test_decimal(
        #[case] size: f32,
        #[case] precision: Option<usize>,
        #[case] separator: Option<&str>,
        #[case] result: &str,
    ) {
        assert_eq!(decimal(size, precision, separator), result);
    }

    #[rstest]
    #[case(50, 1024, &(1, "bytes".to_owned()))]
    #[case(2048, 1024, &(1024, "KB".to_owned()))]
    fn test_pick_unit_and_suffix(
        #[case] size: u32,
        #[case] base: u32,
        #[case] result: &(u32, String),
    ) {
        let suffixes = ["bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
        assert_eq!(pick_unit_and_suffix(size, &suffixes, base), *result);
    }
}
