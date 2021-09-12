use crate::alchemy::AltonWeighable;
use nom::{character::complete, combinator, IResult};
use serde::{Deserialize, Serialize};
use std::{cmp, collections::HashMap, fmt};

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

pub type ElementCounts = HashMap<Element, u32>;

impl AltonWeighable for ElementCounts {
    fn weight(&self) -> u32 {
        self.iter().map(|(e, v)| e.weight() * v).sum()
    }
}

pub fn element_count_parser(element: Element) -> impl Fn(&str) -> IResult<&str, (Element, u32)> {
    move |input: &str| {
        let (input, multiplier) = combinator::opt(complete::u32)(input)?;
        let (input, element) = element_parser_maker(element)(input)?;
        Ok((input, (element, multiplier.unwrap_or(1))))
    }
}

pub fn element_counts_parser(input: &str) -> IResult<&str, ElementCounts> {
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
            .collect(),
    ))
}

fn add_element_counts(
    left_element_counts: &ElementCounts,
    right_element_counts: &ElementCounts,
) -> ElementCounts {
    let mut total_element_counts = left_element_counts.clone();
    right_element_counts
        .clone()
        .into_iter()
        .map(|(e, v)| *total_element_counts.entry(e).or_insert(0) += v)
        .for_each(drop);

    total_element_counts
}

/// Create a list of all possible rearrangements of elements in two `ElementCounts`, so the
/// resulting `ElementCounts` have equal weight.
/// This is meant to be called recursively.
/// Intended for the reaction logic of compounds
pub fn element_rearrangements_of_equal_weight(
    left_element_counts: &ElementCounts,
    right_element_counts: &ElementCounts,
) -> Vec<(ElementCounts, ElementCounts)> {
    fn recurse(
        total_element_counts: &ElementCounts,
        left_element_counts: ElementCounts,
        right_element_counts: ElementCounts,
        desired_weight: u32,
    ) -> Vec<(ElementCounts, ElementCounts)> {
        if left_element_counts.weight() > desired_weight
            || right_element_counts.weight() > desired_weight
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
            possible_reactions.append(&mut recurse(
                &new_total_element_counts,
                left_insert,
                right_element_counts,
                desired_weight,
            ));
            possible_reactions.append(&mut recurse(
                &new_total_element_counts,
                left_element_counts,
                right_insert,
                desired_weight,
            ));

            possible_reactions
        }
    }

    let total_element_counts = add_element_counts(left_element_counts, right_element_counts);

    recurse(
        &total_element_counts,
        HashMap::new(),
        HashMap::new(),
        total_element_counts.weight() / 2,
    )
}
