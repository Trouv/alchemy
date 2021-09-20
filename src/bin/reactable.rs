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

        for alchemy::resources::ReactionRule {
            compound: col_compound,
            ..
        } in &reaction_rules
        {
            if reactive_compounds.contains(col_compound) {
                row.push(
                    row_compound
                        .list_possible_reactions(col_compound)
                        .into_iter()
                        .map(|(left, right)| format!("{}+{}", left, right))
                        .collect::<Vec<String>>()
                        .join(", "),
                )
            } else {
                row.push("".to_string())
            }
        }
    }
    Ok(())
}
