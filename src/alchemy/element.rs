use crate::alchemy::AltonWeighable;
use nom::{character::complete, IResult};
use serde::{Deserialize, Serialize};
use std::{cmp, fmt};

/// The most basic alchemical object.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub enum Element {
    A,
    B,
    C,
    D,
    E,
}

pub fn element_parser_maker(element: Element) -> impl Fn(&str) -> IResult<&str, Element> {
    move |input: &str| {
        let (input, _) = complete::char(
            element
                .to_string()
                .chars()
                .next()
                .expect("Element::to_string() should contain at least one character"),
        )(input)?;
        Ok((input, element))
    }
}

impl AltonWeighable for Element {
    fn weight(&self) -> u32 {
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
        self.weight().cmp(&other.weight())
    }
}
