use std::ops::{Add, Div, Mul, Sub};

use crate::domain::constant::*;

/// Term value
/// * `0` - `T`: Term (`cm^-1`)
///
/// # Examples
///
/// ```
/// use emission_spectrum_lib_rs::prelude::Term;
/// let t1 = Term::new(1.0);
/// assert_eq!(t1.unwrap(), 1.0);
/// let t2 = Term::new(2.0);
/// assert_eq!(t2.unwrap(), 2.0);
/// let t3 = t1 + t2;
/// assert_eq!(t3.unwrap(), 3.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Term(f64);

impl Term {
    /// Returns the new term
    ///
    /// # Arguments
    ///
    /// * `value` - `f64`
    ///
    /// # Examples
    ///
    /// ```
    /// use emission_spectrum_lib_rs::prelude::Term;
    /// let t = Term::new(1.0);
    /// assert_eq!(t.unwrap(), 1.0);
    /// ```
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the value
    ///
    /// # Examples
    ///
    /// ```
    /// use emission_spectrum_lib_rs::prelude::Term;
    /// let t = Term::new(1.0);
    /// assert_eq!(t.unwrap(), 1.0);
    /// ```
    pub fn unwrap(self) -> f64 {
        self.0
    }

    /// Return the value converted to joules
    ///
    /// # Examples
    ///
    /// ```
    /// use emission_spectrum_lib_rs::prelude::Term;
    /// let t = Term::new(1.0);
    /// assert_eq!(t.to_jules(), 1.9864458571489285e-23);
    /// ```
    pub fn to_jules(self) -> f64 {
        let t = 100.0 * self.0; // cm^-1 to m^-1
        t * H * C
    }

    /// Return the value converted to eV
    ///
    /// # Examples
    ///
    /// ```
    /// use emission_spectrum_lib_rs::prelude::Term;
    /// let t = Term::new(1.0);
    /// assert_eq!(t.to_ev(), 0.00012398419874273966);
    /// ```
    pub fn to_ev(self) -> f64 {
        let t = 100.0 * self.0; // cm^-1 to m^-1
        t * H * C / E
    }
}

impl From<Term> for f64 {
    fn from(item: Term) -> f64 {
        item.0
    }
}

impl Add for Term {
    type Output = Term;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl Sub for Term {
    type Output = Term;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl Mul for Term {
    type Output = Term;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl Div for Term {
    type Output = Term;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.0 / rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Term;

    #[test]
    fn test() {
        let t = Term::new(1.0);
        assert_eq!(t.unwrap(), 1.0);
        assert_eq!(t.to_jules(), 1.9864458571489285e-23);
        assert_eq!(t.to_ev(), 0.00012398419874273966);
    }

    #[test]
    fn add() {
        let t1 = Term::new(1.0);
        let t2 = Term::new(2.0);
        assert_eq!((t1 + t2).unwrap(), 3.0);
    }

    #[test]
    fn sub() {
        let t1 = Term::new(3.0);
        let t2 = Term::new(2.0);
        assert_eq!((t1 - t2).unwrap(), 1.0);
    }

    #[test]
    fn mul() {
        let t1 = Term::new(2.0);
        let t2 = Term::new(3.0);
        assert_eq!((t1 * t2).unwrap(), 6.0);
    }

    #[test]
    fn div() {
        let t1 = Term::new(6.0);
        let t2 = Term::new(3.0);
        assert_eq!((t1 / t2).unwrap(), 2.0);
    }
}
