#[derive(Debug, Clone)]
pub struct Gaussian {
    center: f64,
    fwhm: f64,
}

impl Gaussian {
    /// Returns the new Gaussian
    ///
    /// # Arguments
    ///
    /// * `center` - center wave length (m)
    /// * `fwhm` - full width half maximum (m)
    ///
    /// # Examples
    ///
    /// ```
    /// use emission_spectrum_lib_rs::prelude::Gaussian;
    /// let gaussian = Gaussian::new(0.0, 1.0);
    /// ```
    pub fn new(center: f64, fwhm: f64) -> Self {
        Self { center, fwhm }
    }

    /// Returns the new Gaussian
    ///
    /// # Arguments
    ///
    /// * `center` - center wave length (m)
    /// * `fwhm` - full width half maximum (m)
    ///
    /// # Examples
    ///
    /// ```
    /// use emission_spectrum_lib_rs::prelude::Gaussian;
    /// let gaussian = Gaussian::new(0.0, 1.0);
    /// assert_eq!(gaussian.calc(0.0), 1.0);
    /// assert_eq!(gaussian.calc(0.5), 0.5);
    /// ```
    pub fn calc(&self, x: f64) -> f64 {
        (-4.0 * 2.0_f64.ln() * (x - self.center).powi(2) / self.fwhm.powi(2)).exp()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Gaussian;

    #[test]
    fn test() {
        let gaussian = Gaussian::new(0.0, 1.0);
        assert_eq!(gaussian.calc(0.0), 1.0);
        assert_eq!(gaussian.calc(0.5), 0.5);
    }
}
