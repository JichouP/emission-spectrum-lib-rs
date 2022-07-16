use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CalcConfig {
    spin_quantum: SpinQuantumNumber,
}

pub fn calc(config: CalcConfig) -> Vec<(f64, f64)> {
    let spin = config.spin_quantum;
    println!("{:?}", spin);
    unimplemented!()
}
