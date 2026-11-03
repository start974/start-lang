//! `pir` → `sir`, for every file of a project (stdlib, files, REPL inputs):
//!
//! - one node per construct: keyword variants merged, parentheses dropped
//!   (the span of the expression keeps them), comments dropped;
//! - every expression and type definition gets a `DefId`, unique in the
//!   project, and every use is resolved to the definition it refers to: the
//!   last one with that name, before it.

mod error;

use crate::{Checked, CheckedResult, Partial};
use errors::{Errors, Message, ResultErrors as _};
use ir::location::{SourceId, Spanned};
use ir::token::{Token, Trivia};
use ir::{pir, sir};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Errors>;

// ============================================================================
// Lowerer
// ============================================================================

/// lowers the files of a project; it grows with each file: the sources, the
/// definitions and the names visible after it
#[derive(Debug, Default, Clone)]
pub struct Lowerer {
    /// file of each source
    files: HashMap<SourceId, sir::FileId>,
    /// every definition, indexed by `DefId`
    defs: Vec<sir::DefInfo>,
    /// the definition each name refers to at this point: the last one
    scope: HashMap<String, sir::DefId>,
    /// print the `sir`; set again by `Stdlib::debug` once the stdlib is loaded
    pub(crate) debug: bool,
}

/// arguments of a `Lowerer`
#[derive(Debug, Default, Clone, Copy)]
pub struct LowererBuilder {
    debug: bool,
}

impl LowererBuilder {
    /// print the `sir`
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn build(self) -> Lowerer {
        Lowerer {
            debug: self.debug,
            ..Lowerer::default()
        }
    }
}

impl Lowerer {
    pub fn builder() -> LowererBuilder {
        LowererBuilder::default()
    }

    /// the definition `id`
    pub fn def(&self, id: sir::DefId) -> &sir::DefInfo {
        &self.defs[id.0]
    }

    /// lower every command of `file`; a command with an unknown name is left
    /// out, with its errors
    pub fn run(&mut self, file: &Checked<pir::File>) -> CheckedResult<sir::File> {
        let id = self.file(&file.source);
        let first = self.defs.len();
        let commands: Partial<Vec<_>> = file
            .commands
            .iter()
            .map(|cmd| match self.command(cmd, id) {
                Ok(cmd) => Partial::ok(Some(cmd)),
                Err(errs) => Partial::ok(None).with_errors(
                    errs.into_iter()
                        .map(|err| err.with_recovery(Message::text("the command is left out"))),
                ),
            })
            .collect();
        // the definitions made by this file
        let defs = (first..self.defs.len())
            .map(|i| (sir::DefId(i), self.defs[i].clone()))
            .collect();
        commands
            .map(|commands| sir::File {
                file: id,
                commands: commands.into_iter().flatten().collect(),
                defs,
            })
            .check_debug(self.debug)
    }

    /// the `FileId` of `source`, given the first time it is seen
    fn file(&mut self, source: &SourceId) -> sir::FileId {
        if let Some(id) = self.files.get(source) {
            return *id;
        }
        let id = sir::FileId(self.files.len() as u32);
        self.files.insert(source.clone(), id);
        id
    }

    /// a new definition, visible after it
    fn define(&mut self, info: sir::DefInfo) -> sir::DefId {
        let id = sir::DefId(self.defs.len());
        self.scope.insert(info.name.value.clone(), id);
        self.defs.push(info);
        id
    }

    /// the definition `var` refers to, of `kind` (any kind for `None`)
    fn resolve(
        &self,
        var: &pir::Variable,
        kind: Option<sir::DefKind>,
    ) -> Result<sir::Node<sir::Identifier>> {
        match self.scope.get(&var.value) {
            Some(id) if kind.is_none_or(|kind| self.def(*id).kind == kind) => Ok(sir::Node {
                value: sir::Identifier {
                    name: var.value.clone(),
                    id: *id,
                },
                span: var.span(),
            }),
            // one scope for both kinds: a definition of the other kind hides it
            found => {
                let hidden_by = found.map(|id| self.def(*id).kind);
                Err(error::variable_not_found(&var.value, kind, hidden_by, var.span()).into())
            }
        }
    }

    /// lower one command
    fn command(&mut self, cmd: &pir::Command, file: sir::FileId) -> Result<sir::Command> {
        let kind = match &cmd.kind {
            pir::CommandKind::ExpressionDefinition { keyword, def } => {
                let def = self.expression_definition(def, Self::documentation(keyword), file)?;
                sir::CommandKind::ExpressionDefinition(Box::new(def))
            }
            pir::CommandKind::TypeDefinition { keyword, def } => sir::CommandKind::TypeDefinition(
                self.type_definition(def, Self::documentation(keyword), file)?,
            ),
            pir::CommandKind::Eval { expr, .. } => sir::CommandKind::Eval(self.expression(expr)?),
            pir::CommandKind::TypeOf { expr, .. } => {
                sir::CommandKind::TypeOf(self.expression(expr)?)
            }
            pir::CommandKind::Help { var, .. } => sir::CommandKind::Help(self.resolve(var, None)?),
        };
        Ok(sir::Command {
            kind,
            span: cmd.span(),
        })
    }

    /// the name is defined even when the body has an error: its uses after it
    /// are not reported again
    fn expression_definition(
        &mut self,
        def: &pir::ExpressionDefinition,
        doc: Option<sir::Documentation>,
        file: sir::FileId,
    ) -> Result<sir::ExpressionDefinition> {
        let body = self.expression(&def.body).combine(
            def.typed_by
                .as_ref()
                .map(|typed_by| self.ty(&typed_by.ty))
                .transpose(),
        );
        let pir::Pattern::Variable(var) = &def.pattern;
        let name = Self::node(var);
        let span = name.span;
        let id = self.define(sir::DefInfo {
            name: name.clone(),
            kind: sir::DefKind::Expression,
            file,
            span: def.span(),
            doc,
        });
        let (body, ty) = body?;
        Ok(sir::ExpressionDefinition {
            pattern: sir::Pattern::Variable(sir::Node {
                value: sir::Identifier {
                    name: name.value,
                    id,
                },
                span,
            }),
            ty,
            body,
        })
    }

    fn type_definition(
        &mut self,
        def: &pir::TypeDefinition,
        doc: Option<sir::Documentation>,
        file: sir::FileId,
    ) -> Result<sir::TypeDefinition> {
        let ty = self.ty(&def.ty);
        let name = Self::node(&def.name);
        let id = self.define(sir::DefInfo {
            name: name.clone(),
            kind: sir::DefKind::Type,
            file,
            span: def.span(),
            doc,
        });
        Ok(sir::TypeDefinition { id, name, ty: ty? })
    }

    fn ty(&self, ty: &pir::Type) -> Result<sir::Type> {
        Ok(match ty {
            pir::Type::Builtin(b) => sir::Type::Builtin(Self::node(b)),
            pir::Type::Variable(var) => {
                sir::Type::Variable(self.resolve(var, Some(sir::DefKind::Type))?)
            }
        })
    }

    fn expression(&self, expr: &pir::Expression) -> Result<sir::Expression> {
        let kind = match expr {
            pir::Expression::Variable(var) => {
                sir::ExpressionKind::Variable(self.resolve(var, Some(sir::DefKind::Expression))?)
            }
            pir::Expression::Constant(c) => sir::ExpressionKind::Constant(Self::constant(c)),
            // the parentheses only widen the span
            pir::Expression::Paren(paren) => self.expression(&paren.inner)?.kind,
            pir::Expression::Typed { expr, ty, .. } => {
                let (expr, ty) = self.expression(expr).combine(self.ty(ty))?;
                sir::ExpressionKind::Typed {
                    expr: Box::new(expr),
                    ty,
                }
            }
        };
        Ok(sir::Expression {
            kind,
            span: expr.span(),
        })
    }

    // nodes without names to resolve

    /// documentation comment written just before a keyword
    fn documentation<T>(keyword: &Token<T>) -> Option<sir::Documentation> {
        match keyword.leading.last() {
            Some(Trivia::DocComment(doc_comment)) => {
                Some(sir::Documentation::from(doc_comment.0.clone()))
            }
            _ => None,
        }
    }

    /// value and span of a token, without its comments
    fn node<T: Clone>(token: &Token<T>) -> sir::Node<T> {
        sir::Node {
            value: token.value.clone(),
            span: token.span(),
        }
    }

    fn constant(constant: &pir::Constant) -> sir::Constant {
        match constant {
            pir::Constant::Builtin(b) => sir::Constant::Builtin(Self::node(b)),
            pir::Constant::Number(n) => sir::Constant::Number(Self::node(n)),
            pir::Constant::Character(c) => sir::Constant::Character(Self::node(c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use ir::location::Span;
    use std::path::PathBuf;

    /// lex, parse and lower `src`, a file of `source`, with `lowerer`
    fn lower_in(lowerer: &mut Lowerer, source: SourceId, src: &str) -> Partial<sir::File> {
        let tokens = Lexer::builder()
            .build()
            .run(src)
            .unwrap_or_else(|_| panic!("lexing error in {src}"));
        let pir = Parser::builder()
            .source(source)
            .build()
            .run(&tokens)
            .unwrap_or_else(|_| panic!("parsing error in {src}"));
        lowerer.run(&pir).into()
    }

    /// `src` lowered alone, without error
    fn lower(src: &str) -> sir::File {
        let lowered = lower_in(&mut Lowerer::default(), SourceId::Repl, src);
        assert!(lowered.is_ok(), "{} errors in {src}", lowered.errors.len());
        lowered.value
    }

    /// the last command of `src`
    fn last(src: &str) -> sir::CommandKind {
        lower(src).commands.pop().expect("a command").kind
    }

    fn eval(src: &str) -> sir::Expression {
        match last(src) {
            sir::CommandKind::Eval(expr) => expr,
            kind => panic!("not an eval: {kind:?}"),
        }
    }

    /// a use of `name` at `start`, resolved to the definition `id`
    fn reference(name: &str, id: usize, start: usize) -> sir::Node<sir::Identifier> {
        sir::Node {
            value: sir::Identifier {
                name: name.to_string(),
                id: sir::DefId(id),
            },
            span: Span::new(start, start + name.len()),
        }
    }

    fn name(name: &str, start: usize) -> sir::Node<String> {
        sir::Node {
            value: name.to_string(),
            span: Span::new(start, start + name.len()),
        }
    }

    /// error codes of `src` lowered alone
    fn codes(src: &str) -> Vec<i32> {
        let lowered = lower_in(&mut Lowerer::default(), SourceId::Repl, src);
        (&lowered.errors)
            .into_iter()
            .map(|err| err.code())
            .collect()
    }

    #[test]
    fn command_span_includes_keyword_and_dot() {
        let file = lower("Def x := 1.\nEval x.");
        assert_eq!(file.commands[0].span(), Span::new(0, 11));
        assert_eq!(file.commands[1].span(), Span::new(12, 19));
    }

    #[test]
    fn use_resolved_to_its_definition() {
        let expr = eval("Def x := 1.\nEval x.");
        assert_eq!(
            expr.kind,
            sir::ExpressionKind::Variable(reference("x", 0, 17))
        );
        assert_eq!(expr.span(), Span::new(17, 18));
    }

    #[test]
    fn constants() {
        let number = eval("Eval 42.");
        assert_eq!(
            number.kind,
            sir::ExpressionKind::Constant(sir::Constant::Number(sir::Node {
                value: 42u8.into(),
                span: Span::new(5, 7)
            }))
        );
        let builtin = eval("Eval __Constant_true__.");
        assert_eq!(
            builtin.kind,
            sir::ExpressionKind::Constant(sir::Constant::Builtin(sir::Node {
                value: sir::expression::constant::BuiltinConstant::True,
                span: Span::new(5, 22)
            }))
        );
        let character = eval("Eval 'c'.");
        assert_eq!(
            character.kind,
            sir::ExpressionKind::Constant(sir::Constant::Character(sir::Node {
                value: 'c',
                span: Span::new(5, 8)
            }))
        );
    }

    #[test]
    fn parentheses_dropped_span_kept() {
        let expr = eval("Def x := 1.\nEval (x).");
        assert_eq!(
            expr.kind,
            sir::ExpressionKind::Variable(reference("x", 0, 18))
        );
        assert_eq!(expr.span(), Span::new(17, 20));
    }

    #[test]
    fn typed_expression() {
        let expr = eval("Type T := __Type_Nat__.\nDef x := 1.\nEval x : T.");
        assert_eq!(expr.span(), Span::new(41, 46));
        let sir::ExpressionKind::Typed { expr: inner, ty } = expr.kind else {
            panic!("not typed: {:?}", expr.kind)
        };
        assert_eq!(
            inner.kind,
            sir::ExpressionKind::Variable(reference("x", 1, 41))
        );
        assert_eq!(ty, sir::Type::Variable(reference("T", 0, 45)));
    }

    #[test]
    fn keywords_merged() {
        assert!(matches!(last("$ 1."), sir::CommandKind::Eval(_)));
        assert!(matches!(last("?: 1."), sir::CommandKind::TypeOf(_)));
        assert!(matches!(
            last("Def x := 1.\n? x."),
            sir::CommandKind::Help(_)
        ));
        assert!(matches!(
            last("Ty T := __Type_Nat__."),
            sir::CommandKind::TypeDefinition(_)
        ));
        let file = lower("Def a := 1.\nDefinition b := 2.");
        assert_eq!(file.commands.len(), 2);
    }

    #[test]
    fn comments_dropped() {
        let commented = eval("Eval (* one *) 1 (* still one *).");
        assert_eq!(
            commented.kind,
            sir::ExpressionKind::Constant(sir::Constant::Number(sir::Node {
                value: 1u8.into(),
                span: Span::new(15, 16)
            }))
        );
        // a comment is not documentation
        let file = lower("(* not a doc *)\nDef a := 1.");
        assert_eq!(file.defs[&sir::DefId(0)].doc, None);
    }

    #[test]
    fn typeof_and_help() {
        assert!(matches!(last("TypeOf 1."), sir::CommandKind::TypeOf(_)));
        assert_eq!(
            last("Def x := 1.\nHelp x."),
            sir::CommandKind::Help(reference("x", 0, 17))
        );
    }

    #[test]
    fn expression_definition() {
        let file = lower("(** the answer *)\nDef a : __Type_Nat__ := 42.");
        let sir::CommandKind::ExpressionDefinition(def) = &file.commands[0].kind else {
            panic!("not a definition")
        };
        let id = sir::DefId(0);
        assert_eq!(def.pattern, sir::Pattern::Variable(reference("a", 0, 22)));
        assert!(def.ty.is_some());
        // the tables of the file
        let info = &file.defs[&id];
        assert_eq!(info.name, name("a", 22));
        assert_eq!(info.kind, sir::DefKind::Expression);
        assert_eq!(info.span, def.span());
        assert_eq!(
            info.doc,
            Some(sir::Documentation::from(vec!["the answer".to_string()]))
        );
    }

    #[test]
    fn type_definition() {
        let sir::CommandKind::TypeDefinition(def) = last("Type T := __Type_Nat__.") else {
            panic!("not a type definition")
        };
        assert_eq!(def.name, name("T", 5));
        assert_eq!(
            def.ty,
            sir::Type::Builtin(sir::Node {
                value: sir::ty::BuiltinType::Nat,
                span: Span::new(10, 22)
            })
        );
        assert_eq!(lower("Type T := __Type_Nat__.").defs[&def.id].doc, None);
    }

    #[test]
    fn redefinition_gets_a_new_id() {
        // the body of the second `a` refers to the first one
        let file = lower("Def a := 1.\nDef a := a.\nEval a.");
        let sir::CommandKind::ExpressionDefinition(second) = &file.commands[1].kind else {
            panic!("not a definition")
        };
        assert_eq!(
            second.pattern,
            sir::Pattern::Variable(reference("a", 1, 16))
        );
        assert_eq!(
            second.body.kind,
            sir::ExpressionKind::Variable(reference("a", 0, 21))
        );
        let sir::CommandKind::Eval(expr) = &file.commands[2].kind else {
            panic!("not an eval")
        };
        assert_eq!(
            expr.kind,
            sir::ExpressionKind::Variable(reference("a", 1, 29))
        );
    }

    #[test]
    fn unknown_names() {
        assert_eq!(codes("Eval x."), [301]);
        assert_eq!(codes("Help y."), [301]);
        assert_eq!(codes("Def a : U := 1 : V."), [301, 301]);
        // a use before its definition
        assert_eq!(codes("Eval a.\nDef a := 1."), [301]);
    }

    #[test]
    fn names_of_the_wrong_kind() {
        assert_eq!(codes("Type T := __Type_Nat__.\nEval T."), [301]);
        assert_eq!(codes("Def a := 1.\nDef b : a := 1."), [301]);
        // `Help` takes both
        assert_eq!(codes("Type T := __Type_Nat__.\nHelp T."), []);
    }

    #[test]
    fn command_with_an_error_left_out_without_cascade() {
        let lowered = lower_in(
            &mut Lowerer::default(),
            SourceId::Repl,
            "Def a := x.\nEval a.",
        );
        assert_eq!(lowered.errors.len(), 1);
        assert_eq!(lowered.value.commands.len(), 1);
    }

    #[test]
    fn name_hidden_by_the_other_kind() {
        let lowered = lower_in(
            &mut Lowerer::builder().build(),
            SourceId::Repl,
            "Type N := __Type_Nat__.\nDef N := 1.\nDef x : N := 2.",
        );
        let text = (&lowered.errors)
            .into_iter()
            .map(|err| format!("{err:?}"))
            .collect::<String>();
        assert!(text.contains("hidden by an expression"), "{text}");
    }

    #[test]
    fn the_project_grows_with_each_file() {
        let mut lowerer = Lowerer::builder().build();
        let stdlib = SourceId::File(PathBuf::from("stdlib.st"));
        let first = lower_in(&mut lowerer, stdlib.clone(), "Def a := 1.");
        let second = lower_in(&mut lowerer, SourceId::Repl, "Def b := a.");
        let third = lower_in(&mut lowerer, SourceId::Repl, "Eval b.");

        assert!(first.is_ok() && second.is_ok() && third.is_ok());
        assert_ne!(first.value.file, second.value.file);
        assert_eq!(second.value.file, third.value.file);
        assert_eq!(lowerer.def(sir::DefId(1)).name.value, "b");
        assert_eq!(lowerer.def(sir::DefId(1)).file, second.value.file);
        // each file only has its own definitions
        assert_eq!(
            first.value.defs.keys().copied().collect::<Vec<_>>(),
            [sir::DefId(0)]
        );
        assert_eq!(
            second.value.defs.keys().copied().collect::<Vec<_>>(),
            [sir::DefId(1)]
        );
        assert!(third.value.defs.is_empty());
    }
}
