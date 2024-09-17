use super::utils::test_error;

#[test]
fn type_error_simple() {
    test_error("error/type_error_simple")
}

#[test]
fn type_error_many() {
    test_error("error/type_error_many")
}

#[test]
fn var_not_exist() {
    test_error("error/var_not_exist")
}
