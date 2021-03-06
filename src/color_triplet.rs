/// The red, green, and blue components of a color.
///
/// # Fields
///
/// * `red` - Red component of the color.
/// * `green` - Green component of the color.
/// * `blue` - Blue component of the color.
pub struct ColorTriplet {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl ColorTriplet {
    /// Get the color triplet in CSS style.
    ///
    /// # Returns
    ///
    /// A `String` containing the color triplet in CSS style.
    ///
    /// # Examples
    ///
    /// ```
    /// use wealthy::color_triplet::ColorTriplet;
    ///
    /// let color_triplet = ColorTriplet {
    ///     red: 100,
    ///     green: 150,
    ///     blue: 200,
    /// };
    ///
    /// assert_eq!(color_triplet.hex(), "#6496c8");
    /// ```
    #[must_use]
    pub fn hex(&self) -> String { format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue) }

    /// Get the color triplet in RBG format.
    ///
    /// # Returns
    ///
    /// A `String` containing the color triplet in RGB format.
    ///
    /// # Examples
    ///
    /// ```
    /// use wealthy::color_triplet::ColorTriplet;
    ///
    /// let color_triplet = ColorTriplet {
    ///     red: 100,
    ///     green: 150,
    ///     blue: 200,
    /// };
    ///
    /// assert_eq!(color_triplet.rgb(), "rgb(100,150,200)");
    /// ```
    #[must_use]
    pub fn rgb(&self) -> String { format!("rgb({},{},{})", self.red, self.green, self.blue) }

    /// Convert components into floats between 0 and 1.
    ///
    /// # Returns
    ///
    /// A `(f32, f32, f32)` of the color triplet components normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use wealthy::color_triplet::ColorTriplet;
    ///
    /// let color_triplet = ColorTriplet {
    ///     red: 100,
    ///     green: 150,
    ///     blue: 200,
    /// };
    ///
    /// assert_eq!(
    ///     color_triplet.normalized(),
    ///     (0.39215687, 0.5882353, 0.78431374)
    /// );
    /// ```
    #[must_use]
    pub fn normalized(&self) -> (f32, f32, f32) {
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
    #[case(&ColorTriplet { red: 255, green: 255, blue: 255}, "#ffffff")]
    #[case(&ColorTriplet { red: 0, green: 255, blue: 0}, "#00ff00")]
    fn test_hex(#[case] color_triplet: &ColorTriplet, #[case] result: &str) {
        assert_eq!(color_triplet.hex(), result);
    }

    #[rstest]
    #[case(&ColorTriplet { red: 255, green: 255, blue: 255}, "rgb(255,255,255)")]
    #[case(&ColorTriplet { red: 0, green: 255, blue: 0}, "rgb(0,255,0)")]
    fn test_rgb(#[case] color_triplet: &ColorTriplet, #[case] result: &str) {
        assert_eq!(color_triplet.rgb(), result);
    }

    #[rstest]
    #[case(&ColorTriplet { red: 255, green: 255, blue: 255}, (1.0, 1.0, 1.0))]
    #[case(&ColorTriplet { red: 0, green: 255, blue: 0}, (0.0, 1.0, 0.0))]
    fn test_normalized(#[case] color_triplet: &ColorTriplet, #[case] result: (f32, f32, f32)) {
        assert_eq!(color_triplet.normalized(), result);
    }
}
