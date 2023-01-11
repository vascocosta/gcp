#[cfg(test)]
use super::*;

#[test]
fn test_display_float() {
    assert_eq!("2", display_float(2.00));
    assert_eq!("8.83", display_float(8.83));
    assert_eq!("3.57", display_float(3.57));
    assert_eq!("2.33", display_float(2.33));
    assert_eq!("1", display_float(1.00));
    assert_eq!("0", display_float(0.00));
}

#[test]
fn test_mean() {
    assert_eq!("2.00", format!("{:.2}", mean(&vec![1, 2, 3]).unwrap()));
    assert_eq!(
        "8.83",
        format!("{:.2}", mean(&vec![4, 11, 2, 6, 10, 20]).unwrap())
    );
    assert_eq!(
        "3.57",
        format!("{:.2}", mean(&vec![3, 6, 2, 6, 1, 6, 1]).unwrap())
    );
    assert_eq!(
        "2.33",
        format!("{:.2}", mean(&vec![1, 1, 2, 2, 2, 3, 3, 3, 4]).unwrap())
    );
    assert_eq!("1.00", format!("{:.2}", mean(&vec![1]).unwrap()));
    assert_eq!(None, mean(&vec![]));
}

#[test]
fn test_median() {
    assert_eq!(Some(2.00), median(&vec![1, 2, 3]));
    assert_eq!(Some(8.00), median(&vec![4, 11, 2, 6, 10, 20]));
    assert_eq!(Some(3.00), median(&vec![3, 6, 2, 6, 1, 6, 1]));
    assert_eq!(Some(2.00), median(&vec![1, 1, 2, 2, 2, 3, 3, 3, 4]));
    assert_eq!(Some(1.00), median(&vec![1]));
    assert_eq!(None, median(&vec![]));
}

#[test]
fn test_mode() {
    assert_eq!(None, mode(&vec![1, 2, 3]));
    assert_eq!(None, mode(&vec![4, 11, 2, 6, 10, 20]));
    assert_eq!(Some(vec![6]), mode(&vec![3, 6, 2, 6, 1, 6, 1]));
    assert_eq!(Some(vec![2, 3]), mode(&vec![1, 1, 2, 2, 2, 3, 3, 3, 4]));
    assert_eq!(None, mode(&vec![1]));
    assert_eq!(None, mode(&vec![]));
}
