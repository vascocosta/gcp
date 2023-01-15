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
fn test_operation_execute() {
    let addition = Operation::new(Operator::Addition, 1.0, 1.0);
    let subtraction = Operation::new(Operator::Subtraction, 1.0, 1.0);
    let multiplication = Operation::new(Operator::Multiplication, 1.0, 1.0);
    let division = Operation::new(Operator::Division, 1.0, 1.0);

    assert_eq!(2.0, addition.execute().unwrap());
    assert_eq!(0.0, subtraction.execute().unwrap());
    assert_eq!(1.0, multiplication.execute().unwrap());
    assert_eq!(1.0, division.execute().unwrap());
}