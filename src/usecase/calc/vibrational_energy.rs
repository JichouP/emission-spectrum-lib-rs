use crate::domain::term::Term;

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
        assert_eq!(g.get(), 7.125);
    }
}
