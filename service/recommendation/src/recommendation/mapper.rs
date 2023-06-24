use crate::profile::entity::{Filter, Gender};
use crate::recommendation::criteria::Filter as RecommendationFilter;

impl From<Filter> for RecommendationFilter {
    fn from(value: Filter) -> Self {
        Self {
            min_age: value.min_age,
            max_age: value.max_age,
            location: value.location,
            preferences: value.preferences
                .into_iter()
                .map(|gender| gender as i32)
                .collect(),
        }
    }
}

impl From<RecommendationFilter> for Filter {
    fn from(value: RecommendationFilter) -> Self {
        Self {
            min_age: value.min_age,
            max_age: value.max_age,
            location: value.location,
            preferences: value.preferences
                .into_iter()
                .map(|x| match x {
                    1 => Gender::Male,
                    2 => Gender::Female,
                    _ => Gender::NonBinary,
                })
                .collect(),
        }
    }
}

