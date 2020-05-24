use nu_parser::LiteBlock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Filters {
    filters: HashMap<MatchScheme, Filter>,
}

impl Filters {
    pub fn new<I>(filters: I) -> Self
    where
        I: IntoIterator<Item = Filter>,
    {
        Self {
            filters: filters
                .into_iter()
                .map(|x| (x.matches.clone(), x))
                .collect(),
        }
    }

    pub fn find(&self, matches: MatchScheme) -> Option<&Filter> {
        self.filters.get(&matches)
    }
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub matches: MatchScheme,
    pub output_pipeline: LiteBlock,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum MatchScheme {
    ExactCommand(String),
}
