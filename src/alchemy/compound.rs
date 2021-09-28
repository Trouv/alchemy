use crate::alchemy::{element::*, element_counts::*, AltonWeighable};
use nom::combinator;
use rand::seq::IteratorRandom;
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
pub struct Compound<const W: u32> {
    element_counts: ElementCounts,
}

impl<const W: u32> fmt::Display for Compound<W> {
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

impl<const W: u32> hash::Hash for Compound<W> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl<const W: u32> FromStr for Compound<W> {
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
impl<const W: u32> TryFrom<ElementCounts> for Compound<W> {
    type Error = CompoundError;

    fn try_from(element_counts: ElementCounts) -> Result<Compound<W>, Self::Error> {
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

impl<const W: u32> AltonWeighable for Compound<W> {
    fn weight(&self) -> u32 {
        self.element_counts.weight()
    }
}

impl<const W: u32> Compound<W> {
    pub fn try_from_element_counts(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
    ) -> Result<Compound<W>, CompoundError> {
        let mut element_counts = HashMap::new();
        element_counts.insert(Element::A, a);
        element_counts.insert(Element::B, b);
        element_counts.insert(Element::C, c);
        element_counts.insert(Element::D, d);
        element_counts.insert(Element::E, e);

        Ok(Compound::try_from(element_counts)?)
    }

    fn validate(&self) -> bool {
        self.weight() == W
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

    pub fn react(&mut self, other: &mut Compound<W>) {
        let possible_reactions = self.set_of_possible_reactions(&other);

        let (self_reaction, other_reaction) = possible_reactions
            .into_iter()
            .choose(&mut rand::thread_rng())
            .expect("There should at least be two reactions: the current state and its inverse");

        *self = self_reaction;
        *other = other_reaction;
        self.clean();
        other.clean();
    }

    /// Create a set of all possible redistributions of elements in an ElementCounts into two
    /// `Compounds<W>`
    /// This is meant to be called recursively.
    /// Intended for the reaction logic of compounds
    ///
    /// If the elements can't be redistributed to the desired weight, the resulting set will be empty.
    fn reaction_recursion(
        total_element_counts: &ElementCounts,
        left_element_counts: ElementCounts,
        right_element_counts: ElementCounts,
    ) -> HashSet<(Compound<W>, Compound<W>)> {
        if left_element_counts.weight() > W || right_element_counts.weight() > W {
            // The selected rearrangement is invalid
            HashSet::new()
        } else if total_element_counts.weight() == 0 {
            // The selected rearrangement is valid.
            // We know this because neither element_counts exceed the desired_weight,
            // despite the fact that all elements have been redistributed.
            let mut result = HashSet::new();
            result.insert((
                left_element_counts
                    .try_into()
                    .expect("All possible reactions should be valid"),
                right_element_counts
                    .try_into()
                    .expect("All possible reactions should be valid"),
            ));
            result
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

            // Create the new ElementCounts with the added element
            let mut left_insert = left_element_counts.clone();
            *left_insert.entry(selected_element).or_insert(0) += 1;
            let mut right_insert = right_element_counts.clone();
            *right_insert.entry(selected_element).or_insert(0) += 1;

            // Recurse with both possible redistributions
            let mut possible_reactions = HashSet::new();
            possible_reactions.extend(Self::reaction_recursion(
                &new_total_element_counts,
                left_insert,
                right_element_counts,
            ));
            possible_reactions.extend(Self::reaction_recursion(
                &new_total_element_counts,
                left_element_counts,
                right_insert,
            ));

            possible_reactions
        }
    }

    /// Leverages `alchemy::element::element_rearrangements_of_equal_weight()` to list all possible
    /// reactions between two compounds.
    ///
    /// This is not used in `react()`, which prefers to `Compound::try_from(ElementCounts)` only
    /// once, after a rearrangement is randomly selected.
    pub fn set_of_possible_reactions(
        &self,
        other: &Compound<W>,
    ) -> HashSet<(Compound<W>, Compound<W>)> {
        let total_element_counts = add_element_counts(&self.element_counts, &other.element_counts);

        Self::reaction_recursion(
            &total_element_counts,
            ElementCounts::new(),
            ElementCounts::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compound_equality() -> Result<(), CompoundError> {
        assert_eq!(
            Compound::<7>::try_from_element_counts(0, 1, 0, 0, 1)?,
            Compound::<7>::try_from_element_counts(0, 1, 0, 0, 1)?
        );
        Ok(())
    }

    #[test]
    fn test_compound_appropriate_size_ok() -> Result<(), CompoundError> {
        Compound::<7>::try_from_element_counts(7, 0, 0, 0, 0)?;
        Compound::<7>::try_from_element_counts(2, 0, 0, 0, 1)?;
        Compound::<7>::try_from_element_counts(3, 0, 0, 1, 0)?;
        Compound::<7>::try_from_element_counts(0, 0, 1, 1, 0)?;
        Ok(())
    }

    #[test]
    fn test_compound_inappropriate_size_fails() {
        assert_eq!(
            Compound::<7>::try_from_element_counts(5, 0, 0, 0, 0),
            Err(CompoundError::SizeError { size: 5 })
        );
        assert_eq!(
            Compound::<7>::try_from_element_counts(0, 0, 0, 0, 2),
            Err(CompoundError::SizeError { size: 10 })
        );
        assert_eq!(
            Compound::<7>::try_from_element_counts(0, 1, 1, 1, 0),
            Err(CompoundError::SizeError { size: 9 })
        );
        assert_eq!(
            Compound::<7>::try_from_element_counts(0, 1, 0, 1, 0),
            Err(CompoundError::SizeError { size: 6 })
        );
    }

    #[test]
    fn test_compound_reaction_validity() -> Result<(), CompoundError> {
        let mut compound_a = Compound::<7>::try_from_element_counts(1, 3, 0, 0, 0)?;
        let mut compound_b = Compound::<7>::try_from_element_counts(2, 1, 1, 0, 0)?;
        let mut compound_c = Compound::<7>::try_from_element_counts(0, 1, 0, 0, 1)?;
        let mut compound_d = Compound::<7>::try_from_element_counts(0, 0, 1, 1, 0)?;

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
            Compound::<7>::from_str("2ABC")?,
            Compound::<7>::try_from_element_counts(2, 1, 1, 0, 0)?
        );
        assert_eq!(
            Compound::<7>::from_str("BE")?,
            Compound::<7>::try_from_element_counts(0, 1, 0, 0, 1)?
        );
        assert_eq!(
            Compound::<7>::from_str("3A1D")?,
            Compound::<7>::try_from_element_counts(3, 0, 0, 1, 0)?
        );
        Ok(())
    }

    #[test]
    fn test_compound_parsing_failures() {
        assert_eq!(
            Compound::<7>::from_str("D3A"),
            Err(CompoundError::ParseError)
        );
        assert_eq!(
            Compound::<7>::from_str("FAF"),
            Err(CompoundError::ParseError)
        );
        assert_eq!(
            Compound::<7>::from_str("ABC"),
            Err(CompoundError::SizeError { size: 6 })
        );
        assert_eq!(
            Compound::<7>::from_str("ACD"),
            Err(CompoundError::SizeError { size: 8 })
        );
    }

    #[test]
    fn test_list_possible_reactions() -> Result<(), CompoundError> {
        let left_compound: Compound<7> = "2AE".parse()?;
        let right_compound: Compound<7> = "A3B".parse()?;

        let possible_reactions = left_compound.set_of_possible_reactions(&right_compound);
        println!("{:?}", possible_reactions);

        assert_eq!(
            true,
            possible_reactions.contains(&("2AE".parse()?, "A3B".parse()?))
        );
        assert_eq!(
            true,
            possible_reactions.contains(&("BE".parse()?, "3A2B".parse()?))
        );
        assert_ne!(
            true,
            possible_reactions.contains(&("2AE".parse()?, "3A2B".parse()?))
        );
        assert_ne!(
            true,
            possible_reactions.contains(&("A2C".parse()?, "CD".parse()?))
        );

        Ok(())
    }
}
