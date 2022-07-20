pub mod domain;
pub mod prelude;
pub mod usecase;

pub use usecase::calc::*;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::prelude::*;

    #[test]
    fn test() {
        let spin_configs: SpinConfigs = vec![
            (
                CouplingKind::B,
                DoubletBranchKind::P1,
                (1..46).map(|v| v as f64 + 0.5).collect(),
            ),
            (
                CouplingKind::B,
                DoubletBranchKind::P2,
                (3..46).map(|v| v as f64 + 0.5).collect(),
            ),
            (
                CouplingKind::B,
                DoubletBranchKind::Q1,
                (0..46).map(|v| v as f64 + 0.5).collect(),
            ),
            (
                CouplingKind::B,
                DoubletBranchKind::Q2,
                (2..46).map(|v| v as f64 + 0.5).collect(),
            ),
            (
                CouplingKind::B,
                DoubletBranchKind::R1,
                (1..46).map(|v| v as f64 + 0.5).collect(),
            ),
            (
                CouplingKind::B,
                DoubletBranchKind::R2,
                (1..46).map(|v| v as f64 + 0.5).collect(),
            ),
        ];

        let dunham_expression_params_u = vec![
            vec![0.0, 1153.3, -19.48, -0.4],
            vec![1.320, -0.022],
            vec![-4.0e-6],
        ];

        let dunham_expression_params_l = vec![
            vec![0.0, 1308.1, -11.10, 0.093],
            vec![1.4172, -0.0184, 0.00011],
            vec![-6.5e-6],
        ];

        let q: Vec<(u8, u8, f64)> = vec![
            (0, 0, 0.6765),
            (0, 1, 0.2821),
            (1, 1, 0.2316),
            (1, 2, 0.4801),
            (2, 2, 0.0297),
            (2, 3, 0.3935),
        ];

        let wave_lengths: Vec<f64> = (0..20000)
            .into_iter()
            .map(|v| (v as f64) * 0.1e-11 + 200e-9)
            .collect();

        let c = Calc {
            dunham_expression_params_u,
            dunham_expression_params_l,
            fwhm: 3.0e-10,
            lu: 2.0,
            ll: 1.0,
            q,
            r: 0.0,
            spin_configs,
            spin_quantum_number_kind: SpinQuantumNumberKind::Doublet,
            t_e_u: Term::new(49399.6),
            t_e_l: Term::new(0.0),
            t_r: Term::new(0.005),
            t_v: Term::new(0.1),
            wave_lengths: wave_lengths.clone(),
        };

        let res: Vec<(f64, f64)> = wave_lengths
            .clone()
            .into_iter()
            .zip(c.clone().exec().into_iter())
            .collect();

        let res = res
            .into_iter()
            .map(|(a, b)| format!("{},{}", a * 1.0e9, if b > 1.0e-10 { b } else { 0.0 }))
            .collect::<Vec<_>>()
            .join("\n");

        let mut file = File::create("./out.csv").unwrap();
        file.write_fmt(format_args!("{}", res)).unwrap();
    }
}
