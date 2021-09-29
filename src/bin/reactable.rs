use csv::Writer;
use std::{env, io};
use witchcraft::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let anarchy = args.contains(&"--anarchy".to_string()) || args.contains(&"-a".to_string());
    let reaction_rules = alchemy::resources::load_reaction_rules()?;
    let mut writer = Writer::from_writer(io::stdout());

    let mut first_row = vec!["".to_string()];
    for alchemy::resources::ReactionRule { compound, .. } in &reaction_rules {
        first_row.push(compound.to_string())
    }

    writer.write_record(first_row)?;

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
            if reactive_compounds.contains(col_compound) || anarchy {
                row.push(
                    utils::reduce_reverse_pairs(
                        row_compound.set_of_possible_reactions(col_compound),
                    )
                    .into_iter()
                    .filter(|(left, right)| {
                        (left, right) != (row_compound, col_compound)
                            && (right, left) != (row_compound, col_compound)
                    })
                    .map(|(left, right)| format!("{}+{}", left, right))
                    .collect::<Vec<String>>()
                    .join(", "),
                )
            } else {
                row.push("".to_string())
            }
        }
        writer.write_record(row)?;
    }

    writer.flush()?;

    Ok(())
}
