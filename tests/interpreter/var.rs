use super::utils::test_ret;

#[test]
fn test_simpl_var() {
    test_ret("var/simple_var");
}

#[test]
fn test_var_end_by_quot() {
    test_ret("var/var_end_by_quot");
}

#[test]
fn test_many_assign() {
    test_ret("var/many_assign");
}
