use crate::alchemy::compounds::Compound;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum Heat {
    Sitting,
    Simmering,
    Boiling,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum StirMethod {
    ZeroStir,
    SingleStir,
    DoubleStir,
    QuadrupleStir,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ReactionRule {
    compound: Compound,
    /// Setting to None means this compound reacts under any heat
    heat: Option<Heat>,
    /// Setting to None means this compound reacts under any stir method
    stir_method: Option<Heat>,
}
