fn median(a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }
    let mut b = a;
    b.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let a_len = b.len();
    let n = a_len / 2;
    if a_len % 2 == 0 {
        // even case
        return Some((b[n-1] + b[n]) / 2.0);
    } else {
        // odd case
        return Some(b[n]);
    }

}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
