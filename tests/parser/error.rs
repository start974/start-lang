use super::utils::test_error;

#[test]
fn definition_ident() {
    test_error("error/definition_ident");
}

#[test]
fn definition_not_eq_def() {
    test_error("error/definition_not_eq_def");
}

#[test]
fn definition_not_eq_def_with_ty_restr() {
    test_error("error/definition_not_eq_def_with_ty_restr");
}

#[test]
fn wrong_def() {
    test_error("error/wrong_def");
}

#[test]
fn definition_many_error() {
    test_error("error/definition_many_error");
}

#[test]
fn type_is_expression() {
    test_error("error/type_is_expression");
}
