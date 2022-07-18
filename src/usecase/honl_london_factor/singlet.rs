use super::HonlLondonFactorImpl;
#[allow(unused)]
use crate::prelude::{
    CouplingKind,
    SingletBranchKind::{self, *},
};

#[derive(Debug, Clone)]
pub struct HonlLondonFactor {
    /// `J'`: Quantum number of rotation
    #[allow(unused)]
    j: f64,
    /// `Λ'`: Electronic state
    #[allow(unused)]
    lu: f64,
    /// `Λ''`: Electronic state
    #[allow(unused)]
    ll: f64,
    /// `γ`: Rotation-vibration interaction constant
    #[allow(unused)]
    r: f64,
    #[allow(unused)]
    coupling_kind: CouplingKind,
    #[allow(unused)]
    branch_kind: SingletBranchKind,
}

impl HonlLondonFactorImpl for HonlLondonFactor {
    fn eval(self) -> f64 {
        unimplemented!()
    }
}
