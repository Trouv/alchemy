use crate::alchemy::{element::*, AltonWeighable};
use nom::{character::complete, combinator, IResult};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom, fmt, str::FromStr};
use thiserror::Error;

const COMPOUND_WEIGHT: u32 = 7;

#[derive(Error, Debug, PartialEq)]
pub enum CompoundError {
    #[error("invalid alton count in compound: {size}")]
    SizeError { size: u32 },
    #[error("failed to parse compound")]
    ParseError,
}

impl AltonWeighable for HashMap<Element, u32> {
    fn weight(&self) -> u32 {
        self.iter().map(|(e, v)| e.weight() * v).sum()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct Compound {
    element_counts: HashMap<Element, u32>,
}

fn element_count_parser(element: Element) -> impl Fn(&str) -> IResult<&str, (Element, u32)> {
    move |input: &str| {
        let (input, multiplier) = combinator::opt(complete::u32)(input)?;
        let (input, element) = element_parser_maker(element)(input)?;
        Ok((input, (element, multiplier.unwrap_or(1))))
    }
}

fn element_counts_parser(input: &str) -> IResult<&str, HashMap<Element, u32>> {
    let (input, opt_a) = combinator::opt(element_count_parser(Element::A))(input)?;
    let (input, opt_b) = combinator::opt(element_count_parser(Element::B))(input)?;
    let (input, opt_c) = combinator::opt(element_count_parser(Element::C))(input)?;
    let (input, opt_d) = combinator::opt(element_count_parser(Element::D))(input)?;
    let (input, opt_e) = combinator::opt(element_count_parser(Element::E))(input)?;
    Ok((
        input,
        vec![opt_a, opt_b, opt_c, opt_d, opt_e]
            .into_iter()
            .flatten()
            .collect::<HashMap<Element, u32>>(),
    ))
}

impl fmt::Display for Compound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut element_count_pairs = self
            .clone()
            .element_counts
            .into_iter()
            .collect::<Vec<(Element, u32)>>();
        element_count_pairs.sort_by(|a, b| a.0.cmp(&b.0));

        write!(
            f,
            "{}",
            element_count_pairs
                .into_iter()
                .map(|(e, v)| if v > 1 {
                    format!("{}{}", v, e)
                } else {
                    e.to_string()
                })
                .collect::<String>()
        )
    }
}

impl FromStr for Compound {
    type Err = CompoundError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match combinator::all_consuming(element_counts_parser)(value) {
            Ok((_, element_counts)) => Ok(Compound::try_from(element_counts)?),
            _ => Err(CompoundError::ParseError),
        }
    }
}

/// All public constructors of Compound should just call this, since it's directly tied to the
/// internal data structure, and performs the necessary validation.
impl TryFrom<HashMap<Element, u32>> for Compound {
    type Error = CompoundError;

    fn try_from(element_counts: HashMap<Element, u32>) -> Result<Compound, Self::Error> {
        let mut result = Compound { element_counts };

        result.clean();

        if result.validate() {
            Ok(result)
        } else {
            Err(CompoundError::SizeError {
                size: result.weight(),
            })
        }
    }
}

impl AltonWeighable for Compound {
    fn weight(&self) -> u32 {
        self.element_counts.weight()
    }
}

impl Compound {
    pub fn try_from_element_counts(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
    ) -> Result<Compound, CompoundError> {
        let mut element_counts = HashMap::new();
        element_counts.insert(Element::A, a);
        element_counts.insert(Element::B, b);
        element_counts.insert(Element::C, c);
        element_counts.insert(Element::D, d);
        element_counts.insert(Element::E, e);

        Ok(Compound::try_from(element_counts)?)
    }

    fn validate(&self) -> bool {
        self.weight() == COMPOUND_WEIGHT
    }

    /// Remove entries with values equal to 0
    fn clean(&mut self) {
        self.element_counts = self
            .element_counts
            .clone()
            .into_iter()
            .filter(|(_, v)| *v != 0)
            .collect::<HashMap<Element, u32>>();
    }

    pub fn react(&mut self, other: &mut Compound) {
        let mut total_element_counts = self.element_counts.clone();

        // Need a total_element_counts for the recursive algorithm
        // So, we add the values of other's elment_counts to self's
        other
            .element_counts
            .clone()
            .into_iter()
            .map(|(e, v)| *total_element_counts.entry(e).or_insert(0) += v)
            .for_each(drop);

        fn enumerate_possible_reactions(
            total_element_counts: &HashMap<Element, u32>,
            left_element_counts: HashMap<Element, u32>,
            right_element_counts: HashMap<Element, u32>,
        ) -> Vec<(HashMap<Element, u32>, HashMap<Element, u32>)> {
            if left_element_counts.weight() > COMPOUND_WEIGHT
                || right_element_counts.weight() > COMPOUND_WEIGHT
            {
                // The selected reaction is invalid
                Vec::new()
            } else if total_element_counts.weight() == 0 {
                // The selected reaction is valid and complete
                // This assumes that self's and other's weight are COMPOUND_WEIGHT,
                // which they should be, since the public constructers ensure it.
                vec![(left_element_counts, right_element_counts)]
            } else {
                // We need to pick an element to subtract from the total_element_counts
                // and add to one of the new compounds for the next step of recursion.
                // We just pick the first (nonzero) in .into_iter() since order shouldn't matter
                let (selected_element, selected_element_count) = total_element_counts
                    .clone()
                    .into_iter()
                    .filter(|(_, v)| *v > 0)
                    .next()
                    .expect("We've already checked for an empty total_element_counts");

                // Cloning to do this subtraction immutably,
                // not sure this is totally necessary.
                let mut new_total_element_counts = total_element_counts.clone();
                new_total_element_counts.insert(selected_element, selected_element_count - 1);

                // Create the new Compounds with the added element
                let mut left_insert = left_element_counts.clone();
                *left_insert.entry(selected_element).or_insert(0) += 1;
                let mut right_insert = right_element_counts.clone();
                *right_insert.entry(selected_element).or_insert(0) += 1;

                // Recurse with both possible additions
                let mut possible_reactions = Vec::new();
                possible_reactions.append(&mut enumerate_possible_reactions(
                    &new_total_element_counts,
                    left_insert,
                    right_element_counts,
                ));
                possible_reactions.append(&mut enumerate_possible_reactions(
                    &new_total_element_counts,
                    left_element_counts,
                    right_insert,
                ));

                possible_reactions
            }
        }

        let possible_reactions =
            enumerate_possible_reactions(&total_element_counts, HashMap::new(), HashMap::new());

        let (self_reaction, other_reaction) = possible_reactions
            .choose(&mut rand::thread_rng())
            .expect("There should at least be two reactions: the current state and its inverse");

        self.element_counts = self_reaction.clone();
        other.element_counts = other_reaction.clone();
        self.clean();
        other.clean();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compound_equality() -> Result<(), CompoundError> {
        assert_eq!(
            Compound::try_from_element_counts(0, 1, 0, 0, 1)?,
            Compound::try_from_element_counts(0, 1, 0, 0, 1)?
        );
        Ok(())
    }

    #[test]
    fn test_compound_appropriate_size_ok() -> Result<(), CompoundError> {
        Compound::try_from_element_counts(7, 0, 0, 0, 0)?;
        Compound::try_from_element_counts(2, 0, 0, 0, 1)?;
        Compound::try_from_element_counts(3, 0, 0, 1, 0)?;
        Compound::try_from_element_counts(0, 0, 1, 1, 0)?;
        Ok(())
    }

    #[test]
    fn test_compound_inappropriate_size_fails() {
        assert_eq!(
            Compound::try_from_element_counts(5, 0, 0, 0, 0),
            Err(CompoundError::SizeError { size: 5 })
        );
        assert_eq!(
            Compound::try_from_element_counts(0, 0, 0, 0, 2),
            Err(CompoundError::SizeError { size: 10 })
        );
        assert_eq!(
            Compound::try_from_element_counts(0, 1, 1, 1, 0),
            Err(CompoundError::SizeError { size: 9 })
        );
        assert_eq!(
            Compound::try_from_element_counts(0, 1, 0, 1, 0),
            Err(CompoundError::SizeError { size: 6 })
        );
    }

    #[test]
    fn test_compound_reaction_validity() -> Result<(), CompoundError> {
        let mut compound_a = Compound::try_from_element_counts(1, 3, 0, 0, 0)?;
        let mut compound_b = Compound::try_from_element_counts(2, 1, 1, 0, 0)?;
        let mut compound_c = Compound::try_from_element_counts(0, 1, 0, 0, 1)?;
        let mut compound_d = Compound::try_from_element_counts(0, 0, 1, 1, 0)?;

        println!(
            "{} {} {} {}",
            compound_a, compound_b, compound_c, compound_d
        );

        compound_a.react(&mut compound_b);
        compound_c.react(&mut compound_d);
        assert_eq!(compound_a.validate(), true);
        assert_eq!(compound_b.validate(), true);
        assert_eq!(compound_c.validate(), true);
        assert_eq!(compound_d.validate(), true);

        println!(
            "{} {} {} {}",
            compound_a, compound_b, compound_c, compound_d
        );

        compound_a.react(&mut compound_c);
        compound_b.react(&mut compound_d);
        assert_eq!(compound_a.validate(), true);
        assert_eq!(compound_b.validate(), true);
        assert_eq!(compound_c.validate(), true);
        assert_eq!(compound_d.validate(), true);
        println!(
            "{} {} {} {}",
            compound_a, compound_b, compound_c, compound_d
        );

        compound_a.react(&mut compound_d);
        compound_b.react(&mut compound_c);
        assert_eq!(compound_a.validate(), true);
        assert_eq!(compound_b.validate(), true);
        assert_eq!(compound_c.validate(), true);
        assert_eq!(compound_d.validate(), true);
        println!(
            "{} {} {} {}",
            compound_a, compound_b, compound_c, compound_d
        );

        Ok(())
    }

    #[test]
    fn test_compound_parsing() -> Result<(), CompoundError> {
        assert_eq!(
            Compound::from_str("2abc")?,
            Compound::try_from_element_counts(2, 1, 1, 0, 0)?
        );
        assert_eq!(
            Compound::from_str("be")?,
            Compound::try_from_element_counts(0, 1, 0, 0, 1)?
        );
        assert_eq!(
            Compound::from_str("3a1d")?,
            Compound::try_from_element_counts(3, 0, 0, 1, 0)?
        );
        Ok(())
    }

    #[test]
    fn test_compound_parsing_failures() {
        assert_eq!(Compound::from_str("d3a"), Err(CompoundError::ParseError));
        assert_eq!(Compound::from_str("faf"), Err(CompoundError::ParseError));
        assert_eq!(
            Compound::from_str("abc"),
            Err(CompoundError::SizeError { size: 6 })
        );
        assert_eq!(
            Compound::from_str("acd"),
            Err(CompoundError::SizeError { size: 8 })
        );
    }
}
