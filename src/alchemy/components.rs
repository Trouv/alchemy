use serde::{Deserialize, Serialize};

/// Heat may or may not be present on a Cauldron,
/// If it's not present, no reaction should occur.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum Heat {
    Simmering,
    Boiling,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum StirMethod {
    /// As opposed to Heat, reactions may occur when there's no stirring,
    /// represented by this variant.
    ZeroStir,
    SingleStir,
    DoubleStir,
    QuadrupleStir,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct Cauldron;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct RankDisplayer;
