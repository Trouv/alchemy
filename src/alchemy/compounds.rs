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

        let result = Compound { element_counts };

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
}
