pub enum DebugFlag {
    Parser,
    Typer,
    Lexer,
}
pub enum Flag {
    Debug(DebugFlag),
}
