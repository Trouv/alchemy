use rand::random;
use serde::{Deserialize, Serialize};
use std::{cmp, collections, fmt};

/// The most basic alchemical object.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub enum Element {
    A,
    B,
    C,
    D,
    E,
}

impl Element {
    pub fn altons(&self) -> u32 {
        match self {
            Element::A => 1,
            Element::B => 2,
            Element::C => 3,
            Element::D => 4,
            Element::E => 5,
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Element::A => "a",
                Element::B => "b",
                Element::C => "c",
                Element::D => "d",
                Element::E => "e",
            }
        )
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.altons().cmp(&other.altons())
    }
}

const ALTON_COUNT: u32 = 7;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum CompoundError {
    SizeError { size: u32 },
}

impl fmt::Display for CompoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompoundError::SizeError { size: s } => write!(
                f,
                "invalid alton count in Compound: {} (should be {})",
                s, ALTON_COUNT
            ),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct Compound {
    element_counts: collections::HashMap<Element, u32>,
}

impl From<Compound> for Vec<Element> {
    fn from(compound: Compound) -> Vec<Element> {
        compound
            .element_counts
            .iter()
            .map(|(e, v)| (0..*v).map(move |_| *e))
            .flatten()
            .collect::<Vec<Element>>()
    }
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

impl Compound {
    pub fn try_from_element_counts(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
    ) -> Result<Compound, CompoundError> {
        let mut element_counts = collections::HashMap::new();
        element_counts.insert(Element::A, a);
        element_counts.insert(Element::B, b);
        element_counts.insert(Element::C, c);
        element_counts.insert(Element::D, d);
        element_counts.insert(Element::E, e);

        let mut result = Compound { element_counts };

        result.clean();

        if result.validate() {
            Ok(result)
        } else {
            Err(CompoundError::SizeError {
                size: result.altons(),
            })
        }
    }

    fn altons(&self) -> u32 {
        self.element_counts
            .iter()
            .map(|(e, v)| e.altons() * v)
            .sum()
    }

    fn validate(&self) -> bool {
        self.altons() == ALTON_COUNT
    }

    /// Remove entries with values equal to 0
    fn clean(&mut self) {
        self.element_counts = self
            .element_counts
            .clone()
            .into_iter()
            .filter(|(_, v)| *v != 0)
            .collect::<collections::HashMap<Element, u32>>();
    }

    pub fn react(&mut self, other: &mut Compound) {
        let mut total_elements = Vec::<Element>::from(self.clone());
        total_elements.append(&mut other.clone().into());
        total_elements.sort();
        total_elements.reverse();

        self.element_counts.clear();
        other.element_counts.clear();

        for element in total_elements {
            let altons = element.altons();
            if self.altons() + altons <= ALTON_COUNT && other.altons() + altons <= ALTON_COUNT {
                if random::<bool>() {
                    *self.element_counts.entry(element).or_insert(0) += 1;
                } else {
                    *other.element_counts.entry(element).or_insert(0) += 1;
                }
            } else if self.altons() + altons > ALTON_COUNT {
                *other.element_counts.entry(element).or_insert(0) += 1;
            } else if other.altons() + altons > ALTON_COUNT {
                *self.element_counts.entry(element).or_insert(0) += 1;
            }
        }
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
}
