// TODO: Port `measure_renderables` when `Console`, `ConsoleOptions` and
// `RederableType` are ported.

/// Stores the minimum and maximum widths (in characters) required to render an
/// object.
#[derive(Debug, Eq, PartialEq)]
pub struct Measurement {
    /// Minimum number of cells required to render.
    pub minimum: u32,
    /// Maximum number of cells required to render.
    pub maximum: u32,
}

// TODO: Port `get` when `Console`, `ConsoleOptions`, `RenderableType` are
// ported.
impl Measurement {
    pub const fn new(minimum: u32, maximum: u32) -> Self { Self { minimum, maximum } }

    /// Get difference between maximum and minimum.
    ///
    /// # Examples
    ///
    /// ```
    /// use wealthy::measure::Measurement;
    ///
    /// let measure = Measurement::new(10, 100);
    ///
    /// assert_eq!(measure.span(), 90);
    /// ```
    pub fn span(&self) -> u32 { self.maximum - self.minimum }

    /// Get [`Measurement`] that ensures that minimum <= maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// use wealthy::measure::Measurement;
    ///
    /// let measure = Measurement::new(100, 10);
    ///
    /// assert_eq!(measure.normalize(), Measurement::new(10, 10));
    /// ```
    #[must_use]
    pub fn normalize(&self) -> Self {
        let (mut minimum, maximum) = (self.minimum, self.maximum);
        minimum = minimum.min(maximum);

        Self {
            minimum,
            maximum: minimum.max(maximum),
        }
    }

    /// Get a [`Measurement`] where the widths are <= width.
    ///
    /// # Arguments
    ///
    /// * `width` - Maximum desired width.
    ///
    /// # Examples
    ///
    /// ```
    /// use wealthy::measure::Measurement;
    ///
    /// let measure = Measurement::new(100, 10);
    ///
    /// assert_eq!(measure.with_maximum(1), Measurement::new(1, 1));
    /// ```
    #[must_use]
    pub fn with_maximum(&self, width: u32) -> Self {
        let (minimum, maximum) = (self.minimum, self.maximum);

        Self {
            minimum: minimum.min(width),
            maximum: maximum.min(width),
        }
    }

    /// Get a [`Measurement`] where the widths are >= width.
    ///
    /// # Arguments
    ///
    /// * `width` - Minimum desired width.
    ///
    /// # Examples
    ///
    /// ```
    /// use wealthy::measure::Measurement;
    ///
    /// let measure = Measurement::new(100, 10);
    ///
    /// assert_eq!(measure.with_minimum(1000), Measurement::new(1000, 1000));
    /// ```
    #[must_use]
    pub fn with_minimum(&self, width: u32) -> Self {
        let (minimum, maximum) = (self.minimum, self.maximum);

        Self {
            minimum: minimum.max(width),
            maximum: maximum.max(width),
        }
    }

    /// Clamp a [`Measurement`] within the specified range.
    ///
    /// # Arguments
    ///
    /// * `min_width` - Minimum desired width
    /// * `max_width` - Maximum desired width
    ///
    /// # Examples
    ///
    /// ```
    /// use wealthy::measure::Measurement;
    ///
    /// let measure = Measurement::new(10, 100);
    ///
    /// assert_eq!(measure.clamp(Some(10), Some(50)), Measurement::new(10, 50));
    /// ```
    #[must_use]
    pub fn clamp(self, min_width: Option<u32>, max_width: Option<u32>) -> Self {
        let mut measurement = self;

        if let Some(min_width) = min_width {
            measurement = measurement.with_minimum(min_width);
        }

        if let Some(max_width) = max_width {
            measurement = measurement.with_maximum(max_width);
        }

        measurement
    }
}

// TODO: Port `test_no_renderable` when `Console` and `Text`, and `get` are
// ported. TODO: Port `test_measure_renderables` when `Console`, and
// `measure_renderables` are ported.
#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Some(10), Some(50), 20, 50)]
    #[case(Some(30), Some(50), 30, 50)]
    #[case(None, Some(50), 20, 50)]
    #[case(Some(30), None, 30, 100)]
    #[case(None, None, 20, 100)]
    fn test_clamp(
        #[case] min_width: Option<u32>,
        #[case] max_width: Option<u32>,
        #[case] minimum: u32,
        #[case] maximum: u32,
    ) {
        let measurement = Measurement::new(20, 100);

        assert_eq!(
            measurement.clamp(min_width, max_width),
            Measurement { minimum, maximum }
        );
    }
}
