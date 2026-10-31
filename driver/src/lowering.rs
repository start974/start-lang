//! `pir` → `sir`, without errors: every valid CST has an AST.
//!
//! - one node per construct: keyword variants merged, parentheses dropped
//!   (the span of the expression keeps them), comments dropped;
//! - every expression and type definition gets a `DefId` in the `DefTable`
//!   of the file, with its documentation and its place;
//! - `Set` / `Unset` options stay names, checked later.

use ir::location::{SourceId, Spanned};
use ir::{pir, sir};

// ============================================================================
// Lowering
// ============================================================================

/// lowers the commands of one source into a `sir::File`; the state is the
/// file being built (its commands and its `DefTable`), kept between calls so
/// the commands can be lowered one at a time
#[derive(Debug, Default)]
pub struct Lowering {
    file: sir::File,
}

impl Lowering {
    /// start lowering the commands of `source`
    pub fn new(source: SourceId) -> Self {
        Self {
            file: sir::File::new(source),
        }
    }

    /// file built so far
    pub fn file(&self) -> &sir::File {
        &self.file
    }

    /// definitions lowered so far
    pub fn defs(&self) -> &sir::DefTable {
        self.file.defs()
    }

    /// stop lowering and get the file
    pub fn finish(self) -> sir::File {
        self.file
    }

    /// lower every command of `file`
    pub fn lower_file(&mut self, file: &pir::File) {
        for cmd in file.commands() {
            self.command(cmd);
        }
    }

    /// lower one command, add it to the file with its definitions, and get it
    pub fn command(&mut self, cmd: &pir::Command) -> &sir::Command {
        let kind = match &cmd.kind {
            pir::CommandKind::ExpressionDefinition { keyword, def } => {
                let def = self.expression_definition(def, documentation(keyword));
                sir::CommandKind::ExpressionDefinition(Box::new(def))
            }
            pir::CommandKind::TypeDefinition { keyword, def } => {
                let def = self.type_definition(def, documentation(keyword));
                sir::CommandKind::TypeDefinition(def)
            }
            pir::CommandKind::Eval { expr, .. } => sir::CommandKind::Eval(expression(expr)),
            pir::CommandKind::TypeOf { expr, .. } => sir::CommandKind::TypeOf(expression(expr)),
            pir::CommandKind::Help { var, .. } => sir::CommandKind::Help(variable(var)),
            pir::CommandKind::Set { var, .. } => sir::CommandKind::Set(variable(var)),
            pir::CommandKind::UnSet { var, .. } => sir::CommandKind::UnSet(variable(var)),
        };
        self.file.add_command(sir::Command::new(kind, cmd.span()));
        self.file
            .commands()
            .last()
            .expect("a command was just added")
    }

    fn expression_definition(
        &mut self,
        def: &pir::ExpressionDefinition,
        doc: Option<sir::Documentation>,
    ) -> sir::ExpressionDefinition {
        let pattern = pattern(&def.pattern);
        let sir::Pattern::Variable(var) = &pattern;
        let id = self
            .file
            .define(var.clone(), sir::DefKind::Expression, def.span(), doc);
        sir::ExpressionDefinition {
            id,
            pattern,
            ty: def.typed_by.as_ref().map(|typed_by| ty(&typed_by.ty)),
            body: expression(&def.body),
        }
    }

    fn type_definition(
        &mut self,
        def: &pir::TypeDefinition,
        doc: Option<sir::Documentation>,
    ) -> sir::TypeDefinition {
        let name = variable(&def.name);
        let id = self
            .file
            .define(name.clone(), sir::DefKind::Type, def.span(), doc);
        sir::TypeDefinition {
            id,
            name,
            ty: ty(&def.ty),
        }
    }
}

/// lower a whole file at once
pub fn file(file: &pir::File, source: SourceId) -> sir::File {
    let mut lowering = Lowering::new(source);
    lowering.lower_file(file);
    lowering.finish()
}

// ============================================================================
// Nodes without state
// ============================================================================

/// documentation comment written just before a keyword
fn documentation<T>(keyword: &pir::Meta<T>) -> Option<sir::Documentation> {
    keyword
        .doc_lines()
        .map(|lines| sir::Documentation::from(lines.to_vec()))
}

/// value and span of a token, without its comments
fn node<T: Clone>(meta: &pir::Meta<T>) -> sir::Node<T> {
    sir::Node::new(meta.value.clone(), meta.span())
}

fn variable(var: &pir::Variable) -> sir::Variable {
    node(var)
}

fn constant(constant: &pir::Constant) -> sir::Constant {
    match constant {
        pir::Constant::Builtin(b) => sir::Constant::Builtin(node(b)),
        pir::Constant::Number(n) => sir::Constant::Number(node(n)),
        pir::Constant::Character(c) => sir::Constant::Character(node(c)),
    }
}

fn ty(ty: &pir::Type) -> sir::Type {
    match ty {
        pir::Type::Builtin(b) => sir::Type::Builtin(node(b)),
        pir::Type::Variable(var) => sir::Type::Variable(variable(var)),
    }
}

fn expression(expr: &pir::Expression) -> sir::Expression {
    let kind = match expr {
        pir::Expression::Variable(var) => sir::ExpressionKind::Variable(variable(var)),
        pir::Expression::Constant(c) => sir::ExpressionKind::Constant(constant(c)),
        // the parentheses only widen the span
        pir::Expression::Paren(paren) => expression(paren.inner()).kind,
        pir::Expression::Typed { expr, ty: t, .. } => sir::ExpressionKind::Typed {
            expr: Box::new(expression(expr)),
            ty: ty(t),
        },
    };
    sir::Expression::new(kind, expr.span())
}

fn pattern(pattern: &pir::Pattern) -> sir::Pattern {
    match pattern {
        pir::Pattern::Variable(var) => sir::Pattern::Variable(variable(var)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::location::Span;

    /// parse `src` one command at a time, as the current loop does, and lower
    /// every command into one file
    fn lower(src: &str) -> sir::File {
        let mut lowering = Lowering::new(SourceId::Repl);
        let mut offset = 0;
        while !src[offset..].trim().is_empty() {
            let tokens = crate::lexer::lex(&src[offset..], offset)
                .unwrap_or_else(|_| panic!("lexing error in {src}"));
            let end = tokens.last_offset();
            match crate::parser::parse(tokens).unwrap_or_else(|_| panic!("parsing error in {src}"))
            {
                crate::parser::CommandOrEnd::Command(cmd) => {
                    lowering.command(&cmd);
                }
                crate::parser::CommandOrEnd::End(_) => break,
            }
            offset = end;
        }
        lowering.finish()
    }

    /// the only command of `src`
    fn lower_one(src: &str) -> (sir::File, sir::CommandKind) {
        let file = lower(src);
        assert_eq!(file.commands().len(), 1, "{src}");
        let kind = file.commands()[0].kind.clone();
        (file, kind)
    }

    fn eval(src: &str) -> sir::Expression {
        match lower_one(src).1 {
            sir::CommandKind::Eval(expr) => expr,
            kind => panic!("not an eval: {kind:?}"),
        }
    }

    fn var(name: &str, start: usize) -> sir::Variable {
        sir::Variable::new(name.to_string(), Span::new(start, start + name.len()))
    }

    #[test]
    fn command_span_includes_keyword_and_dot() {
        let file = lower("Eval x.");
        assert_eq!(file.commands()[0].span(), Span::new(0, 7));
    }

    #[test]
    fn variable() {
        let expr = eval("Eval x.");
        assert_eq!(expr.kind, sir::ExpressionKind::Variable(var("x", 5)));
        assert_eq!(expr.span(), Span::new(5, 6));
    }

    #[test]
    fn constants() {
        let number = eval("Eval 42.");
        assert_eq!(
            number.kind,
            sir::ExpressionKind::Constant(sir::Constant::Number(sir::Node::new(
                42u8.into(),
                Span::new(5, 7)
            )))
        );
        let character = eval("Eval 'c'.");
        assert_eq!(
            character.kind,
            sir::ExpressionKind::Constant(sir::Constant::Character(sir::Node::new(
                'c',
                Span::new(5, 8)
            )))
        );
        let builtin = eval("Eval __Constant_true__.");
        assert_eq!(
            builtin.kind,
            sir::ExpressionKind::Constant(sir::Constant::Builtin(sir::Node::new(
                sir::expression::constant::BuiltinT::True,
                Span::new(5, 22)
            )))
        );
    }

    #[test]
    fn parentheses_dropped_span_kept() {
        let expr = eval("Eval (x).");
        assert_eq!(expr.kind, sir::ExpressionKind::Variable(var("x", 6)));
        assert_eq!(expr.span(), Span::new(5, 8));
    }

    #[test]
    fn typed_expression() {
        let expr = eval("Eval x : T.");
        assert_eq!(expr.span(), Span::new(5, 10));
        let sir::ExpressionKind::Typed { expr: inner, ty } = expr.kind else {
            panic!("not typed: {:?}", expr.kind)
        };
        assert_eq!(inner.kind, sir::ExpressionKind::Variable(var("x", 5)));
        assert_eq!(ty, sir::Type::Variable(var("T", 9)));
    }

    #[test]
    fn eval_keywords_merged() {
        // `$` is an alias of `Eval`: same node, the span moves with the source
        assert_eq!(
            eval("$ x.").kind,
            sir::ExpressionKind::Variable(var("x", 2))
        );
    }

    #[test]
    fn typeof_help_set_unset() {
        let (_, kind) = lower_one("TypeOf x.");
        assert!(matches!(kind, sir::CommandKind::TypeOf(_)));
        let (_, kind) = lower_one("Help x.");
        assert_eq!(kind, sir::CommandKind::Help(var("x", 5)));
        let (_, kind) = lower_one("Set DebugParser.");
        assert_eq!(kind, sir::CommandKind::Set(var("DebugParser", 4)));
        let (_, kind) = lower_one("Unset Foo.");
        assert_eq!(kind, sir::CommandKind::UnSet(var("Foo", 6)));
    }

    #[test]
    fn expression_definition() {
        let (file, kind) = lower_one("(** the answer *)\nDef a : ℕ := 42.");
        let sir::CommandKind::ExpressionDefinition(def) = kind else {
            panic!("not a definition: {kind:?}")
        };
        let a = var("a", 22);
        assert_eq!(def.pattern, sir::Pattern::Variable(a.clone()));
        assert!(def.ty.is_some());

        let info = file.defs().get(def.id);
        assert_eq!(info.var, a);
        assert_eq!(info.kind, sir::DefKind::Expression);
        assert_eq!(info.span, def.span());
        assert_eq!(
            file.documentation_of("a").map(sir::Documentation::lines),
            Some(&["the answer".to_string()][..])
        );
    }

    #[test]
    fn definition_keywords_merged() {
        let file = lower("Def a := 1.\nDefinition b := 2.");
        let names: Vec<_> = file.defs().iter().map(|(_, info)| info.name()).collect();
        assert_eq!(names, ["a", "b"]);
    }

    #[test]
    fn type_definition() {
        let (file, kind) = lower_one("Type T := __Type_Nat__.");
        let sir::CommandKind::TypeDefinition(def) = kind else {
            panic!("not a type definition: {kind:?}")
        };
        assert_eq!(def.name, var("T", 5));
        assert_eq!(
            def.ty,
            sir::Type::Builtin(sir::Node::new(sir::ty::BuiltinT::Nat, Span::new(10, 22)))
        );
        assert_eq!(file.defs().get(def.id).kind, sir::DefKind::Type);
        assert_eq!(file.defs().doc(def.id), None);
    }

    #[test]
    fn redefinition_gets_a_new_id() {
        let file = lower("Def a := 1.\nTy T := a.\nDef a := 2.");
        let ids = file.defs().all("a");
        assert_eq!(ids.len(), 2);
        assert_ne!(ids[0], ids[1]);
        assert_eq!(file.commands().len(), 3);
    }

    #[test]
    fn whole_file() {
        let mut pir_file = pir::File::default();
        let tokens = crate::lexer::lex("Def a := 1.", 0).unwrap_or_else(|_| panic!("lexing error"));
        let Ok(crate::parser::CommandOrEnd::Command(cmd)) = crate::parser::parse(tokens) else {
            panic!("no command")
        };
        pir_file.add_command(*cmd);

        let source = SourceId::Repl;
        let file = file(&pir_file, source.clone());
        assert_eq!(file.source(), &source);
        assert_eq!(file.commands().len(), 1);
        assert_eq!(file.defs().len(), 1);
    }

    #[test]
    fn state_kept_between_commands() {
        let mut lowering = Lowering::default();
        for src in ["Def a := 1.", "Eval a.", "Def a := a."] {
            let tokens = crate::lexer::lex(src, 0).unwrap_or_else(|_| panic!("lexing error"));
            let Ok(crate::parser::CommandOrEnd::Command(cmd)) = crate::parser::parse(tokens) else {
                panic!("no command")
            };
            let lowered = lowering.command(&cmd);
            assert_eq!(lowered.span(), cmd.span());
        }
        assert_eq!(lowering.file().commands().len(), 3);
        assert_eq!(lowering.defs().all("a").len(), 2);
        assert_eq!(lowering.finish().source(), &SourceId::Unknown);
    }
}
