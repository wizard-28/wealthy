#![allow(dead_code)]

/// The red, green, and blue components of a color.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ColorTriplet {
    /// Red component of the color.
    pub(crate) red: u8,
    /// Green component of the color.
    pub(crate) green: u8,
    /// Blue component of the color.
    pub(crate) blue: u8,
}

impl ColorTriplet {
    /// Instantiate a new [`ColorTriplet`].
    ///
    /// # Arguments
    ///
    /// * `red` - Red component of the color.
    /// * `green` - Green component of the color.
    /// * `blue` - Blue component of the color.
    pub(crate) const fn new(red: u8, green: u8, blue: u8) -> Self { Self { red, green, blue } }

    /// Get the color triplet in CSS style.
    pub(crate) fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }

    /// Get the color triplet in RBG format.
    pub(crate) fn rgb(&self) -> String { format!("rgb({},{},{})", self.red, self.green, self.blue) }

    /// Convert components into floats between 0 and 1.
    #[must_use]
    pub(crate) fn normalized(&self) -> (f32, f32, f32) {
        (
            f32::from(self.red) / 255.0,
            f32::from(self.green) / 255.0,
            f32::from(self.blue) / 255.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(&ColorTriplet::new(255, 255, 255), "#ffffff")]
    #[case(&ColorTriplet::new(0, 255, 0), "#00ff00")]
    fn test_hex(#[case] color_triplet: &ColorTriplet, #[case] result: &str) {
        assert_eq!(color_triplet.hex(), result);
    }

    #[rstest]
    #[case(&ColorTriplet::new(255, 255, 255), "rgb(255,255,255)")]
    #[case(&ColorTriplet::new(0, 255, 0), "rgb(0,255,0)")]
    fn test_rgb(#[case] color_triplet: &ColorTriplet, #[case] result: &str) {
        assert_eq!(color_triplet.rgb(), result);
    }

    #[rstest]
    #[case(&ColorTriplet::new(255, 255, 255), (1.0, 1.0, 1.0))]
    #[case(&ColorTriplet::new(0, 255, 0), (0.0, 1.0, 0.0))]
    fn test_normalized(#[case] color_triplet: &ColorTriplet, #[case] result: (f32, f32, f32)) {
        assert_eq!(color_triplet.normalized(), result);
    }
}
