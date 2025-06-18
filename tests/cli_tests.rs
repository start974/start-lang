#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cmd/parser/*.md")
        .case("tests/cmd/typer/*.md")
        .case("tests/cmd/eval/*.md")
        .run();
}
