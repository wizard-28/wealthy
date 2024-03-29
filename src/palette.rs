#![allow(dead_code)]

use std::ops::Index;

use ordered_float::OrderedFloat;

use crate::color_triplet::ColorTriplet;

/// A palette of available colors.
#[derive(Debug)]
pub(crate) struct Palette<const N: usize> {
    pub(crate) colors: [ColorTriplet; N],
}

// TODO: Port __rich__ when `Table`'s ported.
impl<const N: usize> Palette<N> {
    /// Instantiate a new [`Palette`].
    pub(crate) fn new(colors: [(u8, u8, u8); N]) -> Self {
        Self {
            colors: colors.map(|color| ColorTriplet::new(color.0, color.1, color.2)),
        }
    }

    // NOTE: This was originally named `match` in Rich.
    // NOTE: This is inefficient, and needs caching but [`cached`] doesn't support
    // methods.
    /// Find a color from a palette that most closely matches a given color.
    ///
    /// # Arguments
    ///
    /// * `color` - The color triplet to compare with.
    ///
    /// # Returns
    ///
    /// Index of the closest matching color if found.
    pub(crate) fn closest(&self, color: (u8, u8, u8)) -> Option<usize> {
        let (red1, green1, blue1) = color;

        let get_color_distance = |index: &usize| -> OrderedFloat<f32> {
            let ColorTriplet {
                red: red2,
                green: green2,
                blue: blue2,
            } = self.colors[*index];

            let red_mean = i16::from((red1 + red2) / 2);
            let red = i16::from(red1) - i16::from(red2);
            let green = i16::from(green1) - i16::from(green2);
            let blue = i16::from(blue1) - i16::from(blue2);

            OrderedFloat(
                f32::from(
                    (((512 + red_mean) * red * red) >> 8)
                        + 4 * green * green
                        + (((767 - red_mean) * blue * blue) >> 8),
                )
                .sqrt(),
            )
        };

        (0..self.colors.len()).min_by_key(get_color_distance)
    }
}

impl<const N: usize> Index<usize> for Palette<N> {
    type Output = ColorTriplet;

    fn index(&self, index: usize) -> &Self::Output { &self.colors[index] }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, 0, 0, 0)]
    #[case(0, 0, 1, 0)]
    #[case(0, 0, 2, 1)]
    #[case(0, 1, 0, 0)]
    #[case(0, 1, 1, 1)]
    #[case(0, 1, 2, 1)]
    #[case(0, 2, 0, 1)]
    #[case(0, 2, 1, 1)]
    #[case(0, 2, 2, 1)]
    #[case(1, 0, 0, 0)]
    #[case(1, 0, 1, 0)]
    #[case(1, 0, 2, 1)]
    #[case(1, 1, 0, 1)]
    #[case(1, 1, 1, 1)]
    #[case(1, 1, 2, 1)]
    #[case(1, 2, 0, 1)]
    #[case(1, 2, 1, 1)]
    #[case(1, 2, 2, 2)]
    #[case(2, 0, 0, 0)]
    #[case(2, 0, 1, 1)]
    #[case(2, 0, 2, 1)]
    #[case(2, 1, 0, 1)]
    #[case(2, 1, 1, 1)]
    #[case(2, 1, 2, 1)]
    #[case(2, 2, 0, 1)]
    #[case(2, 2, 1, 2)]
    #[case(2, 2, 2, 2)]
    fn test_name(#[case] red: u8, #[case] green: u8, #[case] blue: u8, #[case] result: usize) {
        let palette = Palette::new([(0, 0, 0), (1, 1, 1), (2, 2, 2)]);

        assert_eq!(palette.closest((red, green, blue)).unwrap(), result);
    }
}
