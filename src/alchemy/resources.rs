use crate::alchemy::{components::*, compound::Compound};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{fs, io};

#[serde_as]
#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ReactionRule {
    #[serde_as(as = "DisplayFromStr")]
    pub compound: Compound<7>,
    /// Setting to None means this compound reacts under any heat
    pub heat: Option<Heat>,
    /// Setting to None means this compound reacts under any stir method
    pub stir_method: Option<StirMethod>,
}

pub fn load_reaction_rules() -> io::Result<Vec<ReactionRule>> {
    let data = fs::read_to_string("assets/design/reaction_rules.json")?;
    Ok(serde_json::from_str(&data)?)
}

pub fn insert_reaction_rules(mut commands: Commands) {
    commands.insert_resource(load_reaction_rules().expect("Failed to load reaction rules"))
}
