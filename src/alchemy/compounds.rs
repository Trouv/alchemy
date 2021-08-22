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

#[derive(Clone, Eq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Compound {
    pub elements: Vec<Element>,
}

impl PartialEq for Compound {
    fn eq(&self, other: &Self) -> bool {
        self.element_count() == other.element_count()
    }
}

impl Compound {
    pub fn element_count(&self) -> collections::HashMap<Element, u32> {
        let mut result = collections::HashMap::new();
        for element in self.elements.clone() {
            *result.entry(element).or_insert(0) += 1;
        }
        result
    }

    pub fn alton_count(&self) -> u32 {
        self.elements.iter().map(|e| e.altons()).sum()
    }

    pub fn react(&mut self, other: &mut Compound) {
        let mut total_elements = self.elements.clone();
        total_elements.extend(other.elements.clone());
        self.elements.clear();
        other.elements.clear();

        total_elements.sort();
        total_elements.reverse();

        for element in total_elements {
            let altons = element.altons();
            if self.alton_count() + altons <= ALTON_COUNT
                && other.alton_count() + altons <= ALTON_COUNT
            {
                if random::<bool>() {
                    self.elements.push(element);
                } else {
                    other.elements.push(element);
                }
            } else if self.alton_count() + altons > 7 {
                other.elements.push(element);
            } else if other.alton_count() + altons > 7 {
                self.elements.push(element);
            }
        }
    }
}
