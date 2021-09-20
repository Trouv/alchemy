use std::io;
use witchcraft::*;

fn main() -> io::Result<()> {
    let reaction_rules = alchemy::resources::load_reaction_rules()?;

    for alchemy::resources::ReactionRule {
        compound: row_compound,
        stir_method,
        heat,
    } in &reaction_rules
    {
        let reactive_compounds =
            alchemy::systems::get_reactive_compounds(&reaction_rules, *stir_method, *heat);

        let mut row = vec![row_compound.to_string()];
    }
    Ok(())
}
