pub mod doublet;
pub mod singlet;
pub mod triplet;

pub trait HonlLondonFactorImpl {
    fn eval(self) -> f64;
}
