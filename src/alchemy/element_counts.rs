use crate::alchemy::{element::*, AltonWeighable};
use nom::{character::complete, combinator, IResult};
use std::collections::HashMap;

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

pub fn add_element_counts(
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
