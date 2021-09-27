use crate::alchemy::{element::*, AltonWeighable};
use nom::{character::complete, combinator, IResult};
use std::collections::HashMap;

pub struct ElementCounts<const W: u32> {
    map: HashMap<Element, u32>,
}

impl<const W: u32> AltonWeighable for ElementCounts<W> {
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

pub fn element_counts_parser<const W: u32>(input: &str) -> IResult<&str, ElementCounts<W>> {
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

fn add_element_counts<const W: u32, const X: u32>(
    left_element_counts: &ElementCounts<W>,
    right_element_counts: &ElementCounts<X>,
) -> HashMap<Element, u32> {
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
///
/// If the elements can't be redistributed to the desired weight, the resulting list will be empty.
pub fn element_rearrangements_of_equal_weight<const W: u32>(
    left_element_counts: &ElementCounts<W>,
    right_element_counts: &ElementCounts<W>,
) -> Vec<(ElementCounts<W>, ElementCounts<W>)> {
    fn recurse<const W: u32>(
        total_element_counts: &HashMap<Element, u32>,
        left_element_counts: HashMap<Element, u32>,
        right_element_counts: HashMap<Element, u32>,
        desired_weight: u32,
    ) -> Vec<(ElementCounts<W>, ElementCounts<W>)> {
        if left_element_counts.weight() > desired_weight
            || right_element_counts.weight() > desired_weight
        {
            // The selected rearrangement is invalid
            Vec::new()
        } else if total_element_counts.weight() == 0 {
            // The selected rearrangement is valid.
            // We know this because neither element_counts exceed the desired_weight,
            // despite the fact that all elements have been redistributed.
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

            // Create the new ElementCounts with the added element
            let mut left_insert = left_element_counts.clone();
            *left_insert.entry(selected_element).or_insert(0) += 1;
            let mut right_insert = right_element_counts.clone();
            *right_insert.entry(selected_element).or_insert(0) += 1;

            // Recurse with both possible redistributions
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impossible_element_rearrangements_give_empty_list() {
        let mut left_element_counts: ElementCounts = HashMap::new();
        let mut right_element_counts: ElementCounts = HashMap::new();

        left_element_counts.insert(Element::C, 5);
        right_element_counts.insert(Element::B, 2);
        assert_eq!(
            Vec::<(ElementCounts, ElementCounts)>::new(),
            element_rearrangements_of_equal_weight(&left_element_counts, &right_element_counts)
        );

        left_element_counts.clear();
        right_element_counts.clear();
        left_element_counts.insert(Element::A, 1);
        assert_eq!(
            Vec::<(ElementCounts, ElementCounts)>::new(),
            element_rearrangements_of_equal_weight(&left_element_counts, &right_element_counts)
        );

        // The total weight doesn't have to be odd to be impossible
        left_element_counts.clear();
        right_element_counts.clear();
        left_element_counts.insert(Element::C, 5);
        right_element_counts.insert(Element::E, 1);
        assert_eq!(
            Vec::<(ElementCounts, ElementCounts)>::new(),
            element_rearrangements_of_equal_weight(&left_element_counts, &right_element_counts)
        );
    }
}
