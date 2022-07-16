use crate::domain::term::Term;

/// Returns a term of vibrational energy (`cm^-1`)
///
/// # Arguments
///
/// * `v` - `v`: A vibrational level
/// * `we` - `ωe`: vibrational constant - first term
/// * `wexe` - `ωexe`: vibrational constant - second term
/// * `weye` - `ωeye`: vibrational constant - third term
/// * `weze` - `ωeze`: vibrational constant - fourth term
///
/// # Examples
///
/// ```
/// use emission_spectrum_lib_rs::{domain::term::Term, usecase::calc::vibrational_energy::vibrational_energy};
/// let g = vibrational_energy(
///     1.0,
///     Term::new(1.0),
///     Some(Term::new(2.0)),
///     Some(Term::new(3.0)),
///     None,
/// );
/// assert_eq!(g.unwrap(), 7.125);
/// ```
pub fn vibrational_energy(
    v: impl Into<f64>,
    we: Term,
    wexe: Option<Term>,
    weye: Option<Term>,
    weze: Option<Term>,
) -> Term {
    let v: f64 = v.into() + 0.5;
    let we: f64 = we.into();
    let wexe: f64 = wexe.unwrap_or_else(|| Term::new(0.0)).into();
    let weye: f64 = weye.unwrap_or_else(|| Term::new(0.0)).into();
    let weze: f64 = weze.unwrap_or_else(|| Term::new(0.0)).into();
    let g: f64 = we * v - wexe * v.powi(2) + weye * v.powi(3) + weze * v.powi(4);
    Term::new(g)
}

#[cfg(test)]
mod tests {
    use crate::domain::term::Term;

    use super::vibrational_energy;

    #[test]
    fn test() {
        let g = vibrational_energy(
            1.0,
            Term::new(1.0),
            Some(Term::new(2.0)),
            Some(Term::new(3.0)),
            None,
        );
        assert_eq!(g.unwrap(), 7.125);
    }
}
