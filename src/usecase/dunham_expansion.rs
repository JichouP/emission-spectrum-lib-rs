use crate::prelude::Term;

type Params = Vec<Vec<f64>>;

/// Dunham Expansion
#[derive(Debug, Clone)]
pub struct DunhamExpansion {
    v: f64,
    j: f64,
    params: Params,
}

impl DunhamExpansion {
    /// Returns the new Dunham Expansion instance
    ///
    /// # Arguments
    ///
    /// * `v` - Vibrational level
    /// * `j` - Rotational level
    /// * `params` - Dunham parameters
    ///
    /// |k\l|0      |1    |2    |3   |4   |
    /// |---|-------|-----|-----|----|----|
    /// |0  |       |`Be` |`-De`|`He`|`Le`|
    /// |1  |`ωe`   |`-αe`|`-βe`|    |    |
    /// |2  |`-ωexe`|`γe` |     |    |    |
    /// |3  |`ωeye` |     |     |    |    |
    /// |4  |`ωeze` |     |     |    |    |
    ///
    pub fn new(v: f64, j: f64, params: Params) -> Self {
        Self { v, j, params }
    }

    /// Returns the result of the evaluation, consuming `self` value
    ///
    /// # Examples
    ///
    /// ```
    /// use emission_spectrum_lib_rs::prelude::DunhamExpansion;
    /// fn test() {
    /// let e = DunhamExpansion::new(1.0, 0.0, vec![vec![0.0, 1.0, -2.0, 3.0, 0.0]]);
    /// assert_eq!(e.eval().unwrap(), 7.125);
    /// }
    /// ```
    pub fn eval(self) -> Term {
        let v = self.v + 0.5; // v+1/2
        let j = self.j * (self.j + 1.0); // J(J+1)
        let res: Params = self
            .params
            .iter()
            .enumerate()
            .map(|(l, vec)| {
                vec.iter()
                    .enumerate()
                    .map(|(k, &y)| y * v.powi(k as i32) * j.powi(l as i32))
                    .collect()
            })
            .collect();
        Term::new(res.concat().iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::DunhamExpansion;

    #[test]
    fn test() {
        let e = DunhamExpansion::new(1.0, 0.0, vec![vec![0.0, 1.0, -2.0, 3.0, 0.0]]);
        assert_eq!(e.eval().unwrap(), 7.125);
    }

    #[test]
    fn it_works_with_cf() {
        let y = DunhamExpansion::new(
            0.0,
            0.0,
            vec![
                vec![0.0, 1308.1, -11.1, 0.093],
                vec![1.4172, -0.0184, 0.00011],
                vec![6.5e-6],
            ],
        );
        assert_eq!(y.eval().unwrap(), 651.286625);
    }
}
