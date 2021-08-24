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
    pub element_count: collections::HashMap<Element, u32>,
}

impl From<Compound> for Vec<Element> {
    fn from(compound: Compound) -> Vec<Element> {
        compound
            .element_count
            .iter()
            .map(|(e, v)| (0..*v).map(move |_| *e))
            .flatten()
            .collect::<Vec<Element>>()
    }
}

impl Compound {
    pub fn altons(&self) -> u32 {
        self.element_count.iter().map(|(e, v)| e.altons() * v).sum()
    }

    pub fn react(&mut self, other: &mut Compound) {
        let mut total_elements = Vec::<Element>::from(self.clone());
        total_elements.append(&mut other.clone().into());
        total_elements.sort();
        total_elements.reverse();

        self.element_count.clear();
        other.element_count.clear();

        for element in total_elements {
            let altons = element.altons();
            if self.altons() + altons <= ALTON_COUNT && other.altons() + altons <= ALTON_COUNT {
                if random::<bool>() {
                    *self.element_count.entry(element).or_insert(0) += 1;
                } else {
                    *other.element_count.entry(element).or_insert(0) += 1;
                }
            } else if self.altons() + altons > ALTON_COUNT {
                *other.element_count.entry(element).or_insert(0) += 1;
            } else if other.altons() + altons > ALTON_COUNT {
                *self.element_count.entry(element).or_insert(0) += 1;
            }
        }
    }
}
