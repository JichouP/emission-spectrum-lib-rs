use crate::{
    domain::constant::{C, E, H},
    prelude::*,
};

pub type SpinConfigs = Vec<(CouplingKind, DoubletBranchKind, Vec<f64>)>;

#[derive(Debug, Clone)]
pub struct Calc {
    pub spin_quantum_number_kind: SpinQuantumNumberKind,
    pub wave_lengths: Vec<f64>,
    pub spin_configs: SpinConfigs,
    /// `Λ'`: Electronic state
    pub lu: f64,
    /// `Λ''`: Electronic state
    pub ll: f64,
    /// `γ`: Rotation-vibration interaction constant
    pub r: f64,
    /// * upper level: u8
    /// * lower level: u8
    /// * `q`: Franck-Condon factor
    pub q: Vec<(u8, u8, f64)>,
    /// |k\l|0      |1    |2    |3   |4   |
    /// |---|-------|-----|-----|----|----|
    /// |0  |       |`Be` |`-De`|`He`|`Le`|
    /// |1  |`ωe`   |`-αe`|`-βe`|    |    |
    /// |2  |`-ωexe`|`γe` |     |    |    |
    /// |3  |`ωeye` |     |     |    |    |
    /// |4  |`ωeze` |     |     |    |    |
    ///
    /// # Example
    ///
    /// ```ignore
    /// vec![
    ///     vec![0.0, we, -wexe, weye, 0.0],
    ///     vec![b, -alpha, r],
    ///     vec![-d, -beta]
    /// ]
    /// ```
    pub dunham_expression_params_u: Vec<Vec<f64>>,
    pub dunham_expression_params_l: Vec<Vec<f64>>,
    /// `Tv`: Vibrational temperature (eV)
    pub t_v: Term,
    /// `Tr`: Rotational temperature (eV)
    pub t_r: Term,
    /// `Te`: Upper minimum electronic energy (eV)
    pub t_e_u: Term,
    /// `Te`: Lower minimum electronic energy (eV)
    pub t_e_l: Term,
    /// * `fwhm` - full width half maximum (m)
    pub fwhm: f64,
}

impl Calc {
    pub fn exec(self) -> Vec<f64> {
        let res: Vec<f64> = match self.spin_quantum_number_kind {
            SpinQuantumNumberKind::Singlet => unimplemented!(),
            SpinQuantumNumberKind::Doublet => {
                let res: Vec<f64> = self
                    .q
                    .into_iter()
                    .map(|(vl_u, vl_l, q)| {
                        let e_v_u = DunhamExpansion::new(
                            vl_u as f64,
                            0.0,
                            vec![self.dunham_expression_params_u[0].clone()],
                        )
                        .eval();
                        let e_v_l = DunhamExpansion::new(
                            vl_l as f64,
                            0.0,
                            vec![self.dunham_expression_params_l[0].clone()],
                        )
                        .eval();

                        let i_v_u =
                            q * ((-1.0 * e_v_u.unwrap() * H * C) / (E * self.t_v.unwrap())).exp();

                        println!("{}", i_v_u);
                        // let i_v_l =
                        //     q * ((-1.0 * e_v_l.unwrap() * H * C) / (E * self.t_v.unwrap())).exp();

                        let res: Vec<f64> = self
                            .spin_configs
                            .clone()
                            .into_iter()
                            .map(|(coupling_kind, branch_kind, j)| {
                                let res: Vec<f64> = j
                                    .into_iter()
                                    .map(|j| {
                                        let mut params_u = self.dunham_expression_params_u.clone();
                                        params_u[0] = vec![];

                                        let mut params_l = self.dunham_expression_params_l.clone();
                                        params_l[0] = vec![];
                                        let e_r_u =
                                            DunhamExpansion::new(vl_u as f64, j, params_u.clone())
                                                .eval();
                                        let e_r_l =
                                            DunhamExpansion::new(vl_l as f64, j, params_l.clone())
                                                .eval();

                                        let s = doublet::HonlLondonFactor {
                                            j,
                                            lu: self.lu,
                                            ll: self.ll,
                                            r: self.r,
                                            coupling_kind: coupling_kind.clone(),
                                            branch_kind: branch_kind.clone(),
                                        };
                                        let s = s.eval();

                                        let e_u_sum = self.t_e_u + e_v_u + e_r_u;
                                        let e_l_sum = self.t_e_l + e_v_l + e_r_l;
                                        let lambda = e_u_sum.to_wave_length(&e_l_sum);
                                        let mu = C / lambda;

                                        let i_r_u = s
                                            * ((-1.0 * e_r_u.unwrap() * H * C)
                                                / (E * self.t_r.unwrap()))
                                            .exp();

                                        let i = i_v_u * i_r_u * mu.powi(4);

                                        let gauss =
                                            |x| i * Gaussian::new(lambda, self.fwhm).calc(x);

                                        let res: Vec<f64> = self
                                            .wave_lengths
                                            .clone()
                                            .into_iter()
                                            .map(gauss)
                                            .collect();

                                        res
                                    })
                                    .reduce(|accum, item| {
                                        accum
                                            .into_iter()
                                            .enumerate()
                                            .map(|(i, v)| v + item[i])
                                            .collect()
                                    })
                                    .unwrap();

                                res
                            })
                            .reduce(|accum, item| {
                                accum
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, v)| v + item[i])
                                    .collect()
                            })
                            .unwrap();

                        res
                    })
                    .reduce(|accum, item| {
                        accum
                            .into_iter()
                            .enumerate()
                            .map(|(i, v)| v + item[i])
                            .collect()
                    })
                    .unwrap();
                res
            }
            SpinQuantumNumberKind::Triplet => unimplemented!(),
        };

        res
    }
}
