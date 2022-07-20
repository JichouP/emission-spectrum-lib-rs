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
