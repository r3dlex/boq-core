#![allow(missing_docs, clippy::expect_used)]

use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
struct CriteriaFile {
    criteria: Vec<Criterion>,
}

#[derive(Deserialize)]
struct Criterion {
    id: String,
    automated_test: String,
    in_v1: bool,
    blocking: bool,
}

#[test]
fn in_v1_blocking_ava_criteria_have_automated_tests() {
    let text = fs::read_to_string("gaeb/criteria/bvbs_ava_matrix.toml").expect("criteria exists");
    let file: CriteriaFile = toml::from_str(&text).expect("criteria parses");

    for criterion in file
        .criteria
        .iter()
        .filter(|criterion| criterion.in_v1 && criterion.blocking)
    {
        assert!(!criterion.id.is_empty());
        assert!(
            !criterion.automated_test.is_empty(),
            "blocking criterion lacks automated test: {}",
            criterion.id
        );
    }
}
