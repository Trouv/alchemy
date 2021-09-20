use crate::alchemy::{element::*, element_counts::*, AltonWeighable};
use nom::combinator;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    convert::{TryFrom, TryInto},
    fmt, hash,
    str::FromStr,
};
use thiserror::Error;

const COMPOUND_WEIGHT: u32 = 7;

#[derive(Error, Debug, PartialEq)]
pub enum CompoundError {
    #[error("invalid alton count in compound: {size}")]
    SizeError { size: u32 },
    #[error("failed to parse compound")]
    ParseError,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct Compound {
    element_counts: ElementCounts,
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

impl hash::Hash for Compound {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
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
impl TryFrom<ElementCounts> for Compound {
    type Error = CompoundError;

    fn try_from(element_counts: ElementCounts) -> Result<Compound, Self::Error> {
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
            .collect::<ElementCounts>();
    }

    pub fn react(&mut self, other: &mut Compound) {
        let possible_reactions =
            element_rearrangements_of_equal_weight(&self.element_counts, &other.element_counts);

        let (self_reaction, other_reaction) = possible_reactions
            .choose(&mut rand::thread_rng())
            .expect("There should at least be two reactions: the current state and its inverse");

        self.element_counts = self_reaction.clone();
        other.element_counts = other_reaction.clone();
        self.clean();
        other.clean();
    }

    /// Leverages `alchemy::element::element_rearrangements_of_equal_weight()` to list all possible
    /// reactions between two compounds.
    ///
    /// This is not used in `react()`, which prefers to `Compound::try_from(ElementCounts)` only
    /// once, after a rearrangement is randomly selected.
    pub fn set_of_possible_reactions(&self, other: &Compound) -> HashSet<(Compound, Compound)> {
        let set_with_inverses: HashSet<(Compound, Compound)> =
            element_rearrangements_of_equal_weight(&self.element_counts, &other.element_counts)
                .into_iter()
                .map(|(left_ec, right_ec)| {
                    (
                        left_ec
                            .try_into()
                            .expect("All possible reactions should be valid"),
                        right_ec
                            .try_into()
                            .expect("All possible reactions should be valid"),
                    )
                })
                .collect();
        let mut result = HashSet::new();
        for (left, right) in set_with_inverses.into_iter() {
            if !result.contains(&(right.clone(), left.clone()))
                && !(left == *self && right == *other)
                && !(right == *self && left == *other)
            {
                result.insert((left, right));
            }
        }
        result
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

    #[test]
    fn test_list_possible_reactions() -> Result<(), CompoundError> {
        let left_compound: Compound = "2ae".parse()?;
        let right_compound: Compound = "a3b".parse()?;

        let possible_reactions = left_compound.set_of_possible_reactions(&right_compound);

        assert_eq!(
            true,
            possible_reactions.contains(&("2ae".parse()?, "a3b".parse()?))
        );
        assert_eq!(
            true,
            possible_reactions.contains(&("be".parse()?, "3a2b".parse()?))
        );
        assert_ne!(
            true,
            possible_reactions.contains(&("2ae".parse()?, "3a2b".parse()?))
        );
        assert_ne!(
            true,
            possible_reactions.contains(&("a2c".parse()?, "cd".parse()?))
        );

        Ok(())
    }
}
