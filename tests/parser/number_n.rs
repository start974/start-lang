use super::utils::test_parse_ok;

#[test]
fn number_simple() {
    test_parse_ok("number_n/number_simple");
}

#[test]
fn number_typed() {
    test_parse_ok("number_n/number_typed");
}
#[test]
fn number_many() {
    test_parse_ok("number_n/number_many");
}
