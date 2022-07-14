use crate::domain::constant::*;

/// `0` - `we`, `wexe`, `weye`, `weze`: Vibrational Constant - first term (cm^-1)
///
/// ```
/// use emission_spectrum_lib_rs::domain::term::vibrational_constant::VibrationalConstant;
/// let we = VibrationalConstant::new(1.23);
/// assert_eq!(we.get(), 1.23)
/// ```
pub struct VibrationalConstant(f64);

impl VibrationalConstant {
    pub fn new(first: f64) -> Self {
        Self(first)
    }

    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn to_jules(&self) -> f64 {
        let t = 100.0 * self.0; // cm^-1 to m^-1
        t * H * C
    }
}

#[cfg(test)]
mod tests {
    use super::VibrationalConstant;

    #[test]
    fn test() {
        let we = VibrationalConstant::new(1.23);
        assert_eq!(we.get(), 1.23);
    }
}
