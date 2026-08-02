use std::collections::HashMap;

fn main() {
    let single = vec![42];
    assert_eq!(median(&single), Some(42.0));
    assert_eq!(mode(&single), Some(42));

    let negatives = vec![-10, -5, -10];
    assert_eq!(median(&negatives), Some(-10.0));
    assert_eq!(mode(&negatives), Some(-10));

    let empty: Vec<i32> = vec![];
    assert_eq!(median(&empty), None);
    assert_eq!(mode(&empty), None);

    let unsorted_odd = vec![5, 1, 3];
    assert_eq!(median(&unsorted_odd), Some(3.0));

    let unsorted_even = vec![10, 2, 8, 4];
    assert_eq!(median(&unsorted_even), Some(6.0));

    let late_mode = vec![3, 1, 2, 2];
    assert_eq!(mode(&late_mode), Some(2));

    let interleaved = vec![9, 1, 9, 2, 9, 3];
    assert_eq!(median(&interleaved), Some(6.0));
    assert_eq!(mode(&interleaved), Some(9));

    let identical = vec![7, 7, 7, 7];
    assert_eq!(median(&identical), Some(7.0));
    assert_eq!(mode(&identical), Some(7));
}

fn median(ints: &[i32]) -> Option<f64> {
    if ints.is_empty() {
        return None;
    }
    let mut sorted = ints.to_vec();
    sorted.sort();
    let mid_idx = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        Some((sorted[mid_idx] as f64 + sorted[(mid_idx) - 1] as f64) / 2.0)
    } else {
        Some(sorted[mid_idx] as f64)
    }
}

fn mode(ints: &[i32]) -> Option<i32> {
    let mut counts = HashMap::new();
    for &int in ints {
        *counts.entry(int).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
}
