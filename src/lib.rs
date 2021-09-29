pub mod alchemy;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Brewing,
}

pub mod utils {
    use std::collections::HashSet;

    pub fn reduce_reverse_pairs<T>(pairs: HashSet<(T, T)>) -> HashSet<(T, T)>
    where
        T: std::hash::Hash + Eq + Clone,
    {
        pairs
            .into_iter()
            .fold(HashSet::new(), |mut collected, (l, r)| {
                if collected.contains(&(r.clone(), l.clone())) {
                    collected
                } else {
                    collected.insert((l, r));
                    collected
                }
            })
    }
}
