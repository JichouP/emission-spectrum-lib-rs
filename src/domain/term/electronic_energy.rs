use crate::domain::constant::*;

/// * `0` - `Te`: Electronic Energy (cm^-1)
///
/// ```
/// use emission_spectrum_lib_rs::domain::term::electronic_energy::ElectronicEnergy;
/// let te = ElectronicEnergy::new(1.23);
/// assert_eq!(te.get(), 1.23)
/// ```
pub struct ElectronicEnergy(f64);

impl ElectronicEnergy {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn get(self) -> f64 {
        self.0
    }

    pub fn to_jules(self) -> f64 {
        let t = 100.0 * self.0; // cm^-1 to m^-1
        t * H * C
    }
}

#[cfg(test)]
mod tests {
    use super::ElectronicEnergy;

    #[test]
    fn test() {
        let te = ElectronicEnergy::new(1.23);
        assert_eq!(te.get(), 1.23);
    }
}
