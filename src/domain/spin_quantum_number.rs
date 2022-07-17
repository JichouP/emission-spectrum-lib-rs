#[derive(Debug, Clone)]
pub enum SpinQuantumNumberKind {
    Singlet(SingletBranchKind),
    Doublet(DoubletBranchKind),
    Triplet(TripletBranchKind),
}

#[derive(Debug, Clone)]
pub enum SingletBranchKind {
    P1,
    Q1,
    R1,
}

#[derive(Debug, Clone)]
pub enum DoubletBranchKind {
    P1,
    Q1,
    R1,
    P2,
    Q2,
    R2,
}

#[derive(Debug, Clone)]
pub enum TripletBranchKind {
    P1,
    Q1,
    R1,
    P2,
    Q2,
    R2,
    P3,
    Q3,
    R3,
}

#[cfg(test)]
pub mod tests {
    use super::{DoubletBranchKind, SpinQuantumNumberKind};

    #[test]
    fn test() {
        let kind = SpinQuantumNumberKind::Doublet(DoubletBranchKind::Q1);

        match kind {
            SpinQuantumNumberKind::Doublet(kind) => match kind {
                DoubletBranchKind::Q1 => {
                    assert!(true);
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}
