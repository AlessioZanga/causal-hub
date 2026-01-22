use approx::assert_relative_eq;
use causal_hub::{
    datasets::{CatEv, CatEvT},
    types::States,
};
use ndarray::prelude::*;

#[test]
fn test_new_sorted() {
    let mut states = States::default();
    states.insert(
        "X".to_string(),
        ["0", "1"].into_iter().map(String::from).collect(),
    );
    states.insert(
        "Y".to_string(),
        ["a", "b"].into_iter().map(String::from).collect(),
    );

    let values = vec![CatEvT::CertainPositive { event: 0, state: 1 }];

    let ev = CatEv::new(states, values);

    assert_eq!(ev.evidences().len(), 2);
    match &ev.evidences()[0] {
        Some(CatEvT::CertainPositive { event, state }) => {
            assert_eq!(*event, 0);
            assert_eq!(*state, 1);
        }
        _ => panic!("Expected CertainPositive"),
    }
    assert!(ev.evidences()[1].is_none());
}

#[test]
fn test_new_unsorted_keys_and_values() {
    // Construct states manually to be unsorted
    let mut states = States::default();
    // Insert "B" before "A" -> Keys unsorted
    // States for B are ["2", "1"] -> Values unsorted
    states.insert(
        "B".to_string(),
        ["2", "1"].into_iter().map(String::from).collect(),
    );
    states.insert(
        "A".to_string(),
        ["y", "x"].into_iter().map(String::from).collect(),
    );

    // Original indices:
    // 0: B (states: 0:"2", 1:"1")
    // 1: A (states: 0:"y", 1:"x")

    // Evidence:
    // 1. CertainPositive for B="1". B is event 0. "1" is state 1.
    // 2. CertainNegative for A not in {"y"}. A is event 1. "y" is state 0.
    let values = vec![
        CatEvT::CertainPositive { event: 0, state: 1 },
        CatEvT::CertainNegative {
            event: 1,
            not_states: [0].into_iter().collect(),
        },
    ];

    let ev = CatEv::new(states, values);

    // Sorted indices:
    // 0: A (states: 0:"x", 1:"y")
    // 1: B (states: 0:"1", 1:"2")

    // Checks:
    // Evidence for B (now event 1):
    // Was B="1". "1" is now state 0.
    // So CertainPositive { event: 1, state: 0 }

    // Evidence for A (now event 0):
    // Was A not "y". "y" is now state 1.
    // So CertainNegative { event: 0, not_states: {1} }

    let evidences = ev.evidences();
    assert_eq!(evidences.len(), 2);

    // Check evidence for A (index 0)
    match &evidences[0] {
        Some(CatEvT::CertainNegative { event, not_states }) => {
            assert_eq!(*event, 0); // A is now at 0
            assert!(not_states.contains(&1)); // "y" is at 1
            assert_eq!(not_states.len(), 1);
        }
        _ => panic!("Expected CertainNegative at index 0"),
    }

    // Check evidence for B (index 1)
    match &evidences[1] {
        Some(CatEvT::CertainPositive { event, state }) => {
            assert_eq!(*event, 1); // B is now at 1
            assert_eq!(*state, 0); // "1" is at 0
        }
        _ => panic!("Expected CertainPositive at index 1"),
    }
}

#[test]
fn test_new_unsorted_uncertain() {
    let mut states = States::default();
    // B: ["2", "1"]
    states.insert(
        "B".to_string(),
        ["2", "1"].into_iter().map(String::from).collect(),
    );
    // A: ["y", "x"]
    states.insert(
        "A".to_string(),
        ["y", "x"].into_iter().map(String::from).collect(),
    );

    // Original: 0->B, 1->A
    // B states: 0->"2", 1->"1"
    // A states: 0->"y", 1->"x"

    // UncertainPositive for B: P("2")=0.2, P("1")=0.8
    // Vector [0.2, 0.8] matches internal order ["2", "1"].

    // UncertainNegative for A: P_not("y")=0.1, P_not("x")=0.9
    // Vector [0.1, 0.9] matches internal order ["y", "x"].

    let values = vec![
        CatEvT::UncertainPositive {
            event: 0,
            p_states: array![0.2, 0.8],
        },
        CatEvT::UncertainNegative {
            event: 1,
            p_not_states: array![0.1, 0.9],
        },
    ];

    let ev = CatEv::new(states, values);

    // Sorted:
    // 0->A: ["x", "y"]
    // 1->B: ["1", "2"]

    let evidences = ev.evidences();

    // Check A (index 0)
    // Was P_not("y")=0.1, P_not("x")=0.9
    // Now "x" is 0, "y" is 1.
    // So vector should be [0.9, 0.1].
    match &evidences[0] {
        Some(CatEvT::UncertainNegative {
            event,
            p_not_states,
        }) => {
            assert_eq!(*event, 0);
            assert_relative_eq!(p_not_states[0], 0.9);
            assert_relative_eq!(p_not_states[1], 0.1);
        }
        _ => panic!("Expected UncertainNegative at index 0"),
    }

    // Check B (index 1)
    // Was P("2")=0.2, P("1")=0.8
    // Now "1" is 0, "2" is 1.
    // So vector should be [0.8, 0.2].
    match &evidences[1] {
        Some(CatEvT::UncertainPositive { event, p_states }) => {
            assert_eq!(*event, 1);
            assert_relative_eq!(p_states[0], 0.8);
            assert_relative_eq!(p_states[1], 0.2);
        }
        _ => panic!("Expected UncertainPositive at index 1"),
    }
}

#[test]
#[should_panic(expected = "Evidence states distributions must have the correct size")]
fn test_invalid_size_uncertain_positive() {
    let mut states = States::default();
    states.insert(
        "X".to_string(),
        ["0", "1"].into_iter().map(String::from).collect(),
    );

    let values = vec![CatEvT::UncertainPositive {
        event: 0,
        p_states: array![0.5], // Wrong size
    }];

    CatEv::new(states, values);
}

#[test]
#[should_panic(expected = "Evidence states distributions must be non-negative")]
fn test_negative_probability() {
    let mut states = States::default();
    states.insert(
        "X".to_string(),
        ["0", "1"].into_iter().map(String::from).collect(),
    );

    let values = vec![CatEvT::UncertainPositive {
        event: 0,
        p_states: array![-0.1, 1.1],
    }];

    CatEv::new(states, values);
}

#[test]
#[should_panic(expected = "Evidence states distributions must sum to 1")]
fn test_sum_probability() {
    let mut states = States::default();
    states.insert(
        "X".to_string(),
        ["0", "1"].into_iter().map(String::from).collect(),
    );

    let values = vec![CatEvT::UncertainPositive {
        event: 0,
        p_states: array![0.5, 0.6],
    }];

    CatEv::new(states, values);
}
