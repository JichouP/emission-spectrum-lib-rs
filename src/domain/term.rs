use crate::domain::constant::*;

/// Term value
/// * `0` - `T`: Term (cm^-1)
///
/// ```
/// use emission_spectrum_lib_rs::domain::term::Term;
/// let t = Term::new(1.23);
/// assert_eq!(t.get(), 1.23)
/// ```
pub struct Term(f64);

impl Term {
    pub fn new(value: f64) -> Self {
        Self(value)
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
    use super::Term;

    #[test]
    fn test() {
        let t = Term::new(1.23);
        assert_eq!(t.get(), 1.23);
    }
}
