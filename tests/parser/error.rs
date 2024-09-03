use super::utils::test_parse_error;

#[test]
fn definition_ident() {
    test_parse_error("error/definition_ident");
}

#[test]
fn definition_not_eq_def() {
    test_parse_error("error/definition_not_eq_def");
}

#[test]
fn definition_not_eq_def_with_ty_restr() {
    test_parse_error("error/definition_not_eq_def_with_ty_restr");
}

#[test]
fn definition_no_def() {
    test_parse_error("error/definition_no_def");
}

#[test]
fn expr_wrong() {
    test_parse_error("error/expr_wrong");
}

#[test]
fn definition_many_error() {
    test_parse_error("error/definition_many_error");
    // TODO : fix parser
}
