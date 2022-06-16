use std::collections::HashSet;
use std::hash::Hash;

fn unique(mut a: Vec<i32>) -> Vec<i32> {
    // unique_sol1(a)
    // unique_sol2(a)
    // unique_generic_type(a)
    unique_generic_iterator(a.into_iter())
}

fn unique_sol1(a: Vec<i32>) -> Vec<i32> {
    let mut dedup_vec = Vec::new();
    for item in a {
        if !dedup_vec.contains(&item) {
            dedup_vec.push(item);
        }
    }
    dedup_vec.sort();
    dedup_vec
}

fn unique_sol2(mut a: Vec<i32>) -> Vec<i32> {
    // inplace removal
    a.sort();
    a.dedup();
    a
}

// advanced 1: use generic types
fn unique_generic_type<T: Ord>(mut a: Vec<T>) -> Vec<T> {
    a.sort();
    a.dedup();
    a
}

// advanced 2: keep items in order
// fn unique_generic_stable<T>(a: Iterator<T>) -> Vec<T> {
//     todo!();
// }

// advanced 3: use iterators
fn unique_generic_iterator<T: Ord + Hash + Clone, I: Iterator<Item = T>>(a: I) -> Vec<T> {
    let mut visited = HashSet::new();
    let mut dedup_vec: Vec<T> = a.filter(|x: &T| {
        if visited.contains(x) {
            return false;
        } else {
            visited.insert(x.clone());
            return true;
        }
    })
    .collect();
    dedup_vec.sort();
    dedup_vec
}

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}
