pub mod doublet;
pub mod singlet;
pub mod triplet;

trait HonlLondonFactorImpl {
    fn eval(self) -> f64;
}
