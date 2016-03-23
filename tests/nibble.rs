#[macro_use]
extern crate bounded_integer;

use bounded_integer::BoundedInteger;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
#[allow(dead_code)]
#[repr(u8)]
enum SNibble {
    N8 = 248, N7, N6, N5, N4, N3, N2, N1, U0 = 0, P1, P2, P3, P4, P5, P6, P7
}
bounded_integer_impls!(SNibble, i8, SNibble::N8, SNibble::P7);
bounded_integer_partial_ord_impl!(SNibble);

#[test]
fn from_repr() {
    assert_eq!(Some(SNibble::U0), SNibble::from_repr(0i8));
    assert_eq!(Some(SNibble::N8), SNibble::from_repr(-8i8));
    assert_eq!(Some(SNibble::P7), SNibble::from_repr(7i8));
}

#[test]
fn to_repr() {
    assert_eq!(0i8, SNibble::U0.to_repr());
    assert_eq!(-8i8, SNibble::N8.to_repr());
    assert_eq!(7i8, SNibble::P7.to_repr());
}

#[test]
fn min_value() {
    assert_eq!(SNibble::N8, SNibble::min_value());
}

#[test]
fn max_value() {
    assert_eq!(SNibble::P7, SNibble::max_value());
}

#[test]
fn checked_add() {
    assert_eq!(Some(SNibble::P3), SNibble::P1.checked_add(SNibble::P2));
    assert_eq!(Some(SNibble::N3), SNibble::N1.checked_add(SNibble::N2));
    assert_eq!(Some(SNibble::N1), SNibble::P1.checked_add(SNibble::N2));
    assert_eq!(None, SNibble::P7.checked_add(SNibble::P1));
    assert_eq!(None, SNibble::N8.checked_add(SNibble::N1));
}

#[test]
fn checked_sub() {
    assert_eq!(Some(SNibble::P1), SNibble::P3.checked_sub(SNibble::P2));
    assert_eq!(Some(SNibble::N1), SNibble::N3.checked_sub(SNibble::N2));
    assert_eq!(Some(SNibble::P1), SNibble::N1.checked_sub(SNibble::N2));
    assert_eq!(None, SNibble::N8.checked_sub(SNibble::P1));
    assert_eq!(None, SNibble::P7.checked_sub(SNibble::N1));
}

#[test]
fn checked_mul() {
    assert_eq!(Some(SNibble::P6), SNibble::P2.checked_mul(SNibble::P3));
    assert_eq!(Some(SNibble::P6), SNibble::N2.checked_mul(SNibble::N3));
    assert_eq!(Some(SNibble::N6), SNibble::N2.checked_mul(SNibble::P3));
    assert_eq!(None, SNibble::P2.checked_mul(SNibble::P4));
    assert_eq!(None, SNibble::N2.checked_mul(SNibble::N4));
    assert_eq!(None, SNibble::N3.checked_mul(SNibble::P3));
}
