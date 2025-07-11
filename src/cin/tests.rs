use super::*;

#[test]
fn magic_number() {
    let expected = 42u8;
    let got = cin::<u8>("Enter a number: ");
    assert_eq!(got, expected);
}
