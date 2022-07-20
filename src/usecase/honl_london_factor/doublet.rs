use super::HonlLondonFactorImpl;
use crate::prelude::{
    CouplingKind,
    DoubletBranchKind::{self, *},
};

#[derive(Debug, Clone)]
pub struct HonlLondonFactor {
    /// `J'`: Quantum number of rotation
    j: f64,
    /// `Λ'`: Electronic state
    lu: f64,
    /// `Λ''`: Electronic state
    ll: f64,
    /// `γ`: Rotation-vibration interaction constant
    r: f64,
    coupling_kind: CouplingKind,
    branch_kind: DoubletBranchKind,
}

impl HonlLondonFactorImpl for HonlLondonFactor {
    fn eval(self) -> f64 {
        #[allow(unused)]
        let Self {
            j,
            lu,
            ll,
            r,
            coupling_kind,
            branch_kind,
        } = self;

        let dl = lu - ll;

        let up = match coupling_kind {
            CouplingKind::A => unimplemented!(),
            CouplingKind::B => |j: f64, l: f64| 2.0 * (j - l + 0.5),
        };
        let um = match coupling_kind {
            CouplingKind::A => unimplemented!(),
            CouplingKind::B => |j: f64, l: f64| 2.0 * (j + l + 0.5),
        };
        let cp = match coupling_kind {
            CouplingKind::A => unimplemented!(),
            CouplingKind::B => |j: f64, l: f64| 4.0 * (j + 0.5) * (j - l + 0.5),
        };
        let cm = match coupling_kind {
            CouplingKind::A => unimplemented!(),
            CouplingKind::B => |j: f64, l: f64| 4.0 * (j + 0.5) * (j + l + 0.5),
        };

        if dl == 1.0 {
            match branch_kind {
                P1 => {
                    return (j - ll - 1.5)
                        * (j - ll - 0.5)
                        * (um(j - 1.0, lu) * um(j, ll) + 4.0 * (j - ll + 0.5) * (j + ll + 0.5))
                            .powi(2)
                        / (8.0 * j * cm(j - 1.0, lu) * cm(j, ll))
                }
                Q1 => {
                    return (j + 0.5)
                        * (j - ll - 0.5)
                        * (j + ll + 1.5)
                        * (up(j - 1.0, lu) * up(j, ll) + 4.0 * (j - ll + 0.5) * (j + ll + 0.5))
                            .powi(2)
                        / (4.0 * j * (j + 1.0) * cm(j, lu) * cm(j, ll))
                }
                R1 => {
                    return (j + ll + 1.5)
                        * (j + ll + 2.5)
                        * (um(j + 1.0, lu) * um(j, ll) + 4.0 * (j - ll + 0.5) * (j + ll + 0.5))
                            .powi(2)
                        / (8.0 * (j + 1.0) * cm(j + 1.0, lu) * cm(j, ll))
                }
                P2 => {
                    return (j - ll - 1.5)
                        * (j - ll - 0.5)
                        * (up(j - 1.0, lu) * up(j, ll) + 4.0 * (j - ll + 0.5) * (j + ll + 0.5))
                            .powi(2)
                        / (8.0 * j * cp(j - 1.0, lu) * cp(j, ll))
                }
                Q2 => {
                    return (j + 0.5)
                        * (j - ll - 0.5)
                        * (j + ll + 1.5)
                        * (up(j - 1.0, lu) * up(j, ll) + 4.0 * (j - ll + 0.5) * (j + ll + 0.5))
                            .powi(2)
                        / (4.0 * j * (j + 1.0) * cp(j, lu) * cp(j, ll))
                }
                R2 => {
                    return (j + ll + 1.5)
                        * (j + ll + 2.5)
                        * (up(j + 1.0, lu) * up(j, ll) + 4.0 * (j - ll + 0.5) * (j + ll + 0.5))
                            .powi(2)
                        / (8.0 * (j + 1.0) * cp(j + 1.0, lu) * cp(j, ll))
                }
            }
        }

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::{INFINITY, NAN};

    use crate::{
        prelude::DoubletBranchKind::{self, *},
        usecase::honl_london_factor::HonlLondonFactorImpl,
    };

    use super::HonlLondonFactor;

    fn init(branch_kind: DoubletBranchKind) -> Vec<f64> {
        let v: Vec<i64> = (0..35).collect();
        let v: Vec<f64> = v.into_iter().map(|v| v as f64 + 0.5).collect();
        let res: Vec<f64> = v
            .iter()
            .map(|&j| {
                let s = HonlLondonFactor {
                    j,
                    lu: 2.0,
                    ll: 1.0,
                    r: 0.0,
                    coupling_kind: crate::prelude::CouplingKind::B,
                    branch_kind: branch_kind.clone(),
                };
                s.eval()
            })
            .collect();
        res
    }

    fn assert_vec_includes_nan(v1: Vec<f64>, v2: Vec<f64>) -> bool {
        if v1.len() != v2.len() {
            panic!("left: {:?}\nright: {:?}", v1, v2);
        };

        let res = v1
            .iter()
            .zip(v2.clone())
            .all(|(x1, x2)| if x1.is_nan() { x2.is_nan() } else { x1.eq(&x2) });

        if !res {
            panic!("left: {:?}\nright: {:?}", v1, v2);
        }

        res
    }

    #[test]
    fn test_p1() {
        let p1 = init(P1);

        assert!(assert_vec_includes_nan(
            p1,
            vec![
                INFINITY,
                -0.0,
                0.0,
                0.38095238095238093,
                0.8333333333333334,
                1.309090909090909,
                1.794871794871795,
                2.2857142857142856,
                2.7794117647058822,
                3.2748538011695905,
                3.7714285714285714,
                4.2687747035573125,
                4.766666666666667,
                5.264957264957265,
                5.763546798029557,
                6.26236559139785,
                6.761363636363637,
                7.260504201680672,
                7.75975975975976,
                8.259109311740891,
                8.758536585365853,
                9.258028792912514,
                9.757575757575758,
                10.257169287696577,
                10.756802721088436,
                11.256470588235294,
                11.756168359941945,
                12.255892255892256,
                12.75563909774436,
                13.255406195207481,
                13.755191256830601,
                14.254992319508448,
                14.754807692307692,
                15.254635911352329,
                15.754475703324808
            ]
        ));
    }

    #[test]
    fn test_p2() {
        let p2 = init(P2);

        assert!(assert_vec_includes_nan(
            p2,
            vec![
                NAN,
                0.0,
                NAN,
                0.6428571428571429,
                1.0666666666666667,
                1.5151515151515151,
                1.978021978021978,
                2.45,
                2.9281045751633985,
                3.4105263157894736,
                3.896103896103896,
                4.384057971014493,
                4.873846153846154,
                5.365079365079365,
                5.857471264367816,
                6.350806451612903,
                6.8449197860962565,
                7.339682539682539,
                7.834992887624467,
                8.330769230769231,
                8.826945412311266,
                9.323467230443974,
                9.820289855072463,
                10.317375886524824,
                10.81469387755102,
                11.312217194570136,
                11.809923130677848,
                12.307792207792208,
                12.805807622504537,
                13.303954802259886,
                13.802221047065045,
                14.300595238095237,
                14.7990675990676,
                15.297629499561019,
                15.796273291925466
            ]
        ));
    }

    #[test]
    fn test_q1() {
        let q1 = init(Q1);

        assert!(assert_vec_includes_nan(
            q1,
            vec![
                -0.0,
                0.0,
                0.1523809523809524,
                0.5142857142857142,
                1.0343434343434343,
                1.665001665001665,
                2.3736263736263736,
                3.138562091503268,
                3.945235638114895,
                4.7835953520164045,
                5.64652738565782,
                6.528874024526198,
                7.426813186813187,
                8.337456668491152,
                9.258583611420097,
                10.18845954804209,
                11.125710890416773,
                12.06923615344668,
                13.018142028668345,
                13.97169659608684,
                14.929294566276983,
                15.890431105800165,
                16.854681878918697,
                17.821687653784917,
                18.791142303075077,
                19.762783358787797,
                20.736384509969415,
                21.711749593419285,
                22.688707742471315,
                23.66710944198956,
                24.64682329833044,
                25.627733377733378,
                26.609736999903813,
                27.59274289858708,
                28.57666967991524
            ]
        ));
    }

    #[test]
    fn test_q2() {
        let q2 = init(Q2);

        assert!(assert_vec_includes_nan(
            q2,
            vec![
                NAN,
                NAN,
                1.5238095238095237,
                2.5714285714285716,
                3.6202020202020204,
                4.662004662004662,
                5.696703296703297,
                6.7254901960784315,
                7.749570003439972,
                8.769924812030075,
                9.787314135140223,
                10.80231884057971,
                11.815384615384616,
                12.82685641306331,
                13.837004078605858,
                14.846041055718475,
                15.854138018843901,
                16.86143286143286,
                17.868038078564393,
                18.87404627892433,
                19.87953434351619,
                20.884566596194503,
                21.889197245348956,
                22.893472282530034,
                23.897430972388957,
                24.901107032072627,
                25.904529572454102,
                26.907723855092275,
                27.910711905421064,
                28.913513012873945,
                29.91614414142177,
                30.91862026862027,
                31.920954667223324,
                32.92315914035958,
                33.92524421893681
            ]
        ));
    }

    #[test]
    fn test_r1() {
        let r1 = init(R1);

        assert!(assert_vec_includes_nan(
            r1,
            vec![
                4.0,
                3.6,
                3.8095238095238093,
                4.166666666666667,
                4.581818181818182,
                5.0256410256410255,
                5.485714285714286,
                5.955882352941177,
                6.432748538011696,
                6.914285714285715,
                7.399209486166008,
                7.886666666666667,
                8.376068376068377,
                8.866995073891626,
                9.359139784946237,
                9.852272727272727,
                10.346218487394959,
                10.84084084084084,
                11.336032388663968,
                11.831707317073171,
                12.327796234772979,
                12.824242424242424,
                13.32099907493062,
                13.818027210884354,
                14.315294117647058,
                14.81277213352685,
                15.31043771043771,
                15.808270676691729,
                16.3062536528346,
                16.804371584699453,
                17.302611367127497,
                17.80096153846154,
                18.299412030755313,
                18.797953964194374,
                19.29657947686117
            ]
        ));
    }

    #[test]
    fn test_r2() {
        let r2 = init(R2);

        assert!(assert_vec_includes_nan(
            r2,
            vec![
                NAN,
                2.6666666666666665,
                3.2142857142857144,
                3.7333333333333334,
                4.242424242424242,
                4.747252747252747,
                5.25,
                5.751633986928105,
                6.252631578947368,
                6.753246753246753,
                7.253623188405797,
                7.753846153846154,
                8.253968253968255,
                8.754022988505747,
                9.254032258064516,
                9.754010695187166,
                10.253968253968255,
                10.753911806543385,
                11.253846153846155,
                11.753774680603948,
                12.253699788583509,
                12.753623188405797,
                13.25354609929078,
                13.753469387755102,
                14.253393665158372,
                14.753319357092941,
                15.253246753246753,
                15.753176043557168,
                16.253107344632767,
                16.753040719196193,
                17.25297619047619,
                17.752913752913752,
                18.252853380158033,
                18.7527950310559,
                19.252738654147105
            ]
        ));
    }
}
