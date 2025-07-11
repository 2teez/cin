use cin::cin::cin;

#[test]
fn check_u8() {
    let expected = 127u8;
    let got: u8 = cin("Enter a number: ");
    assert_eq!(got, expected);
}

#[test]
fn check_string_input() {
    let expected = String::from("london");
    let got = cin::<String>("Enter a string: ");
    assert_eq!(got, expected);
}
