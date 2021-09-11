use crate::alchemy::{components::*, compounds::Compound};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::fs;

#[serde_as]
#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ReactionRule {
    #[serde_as(as = "DisplayFromStr")]
    pub compound: Compound,
    /// Setting to None means this compound reacts under any heat
    pub heat: Option<Heat>,
    /// Setting to None means this compound reacts under any stir method
    pub stir_method: Option<StirMethod>,
}

pub fn load_reaction_rules(mut commands: Commands) {
    let data = fs::read_to_string("assets/design/reaction_rules.json").unwrap();
    let reaction_rules: Vec<ReactionRule> = serde_json::from_str(&data).unwrap();
    commands.insert_resource(reaction_rules)
}
