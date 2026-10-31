//! Diagnostics of a document: every pass on its text; errors become *error*
//! diagnostics, the outputs of `Eval`, `TypeOf` and `Help` *information*
//! diagnostics at their place.

use super::{backend::Backend, position_memo::PositionMemo};
use crate::compile::{Output, compile};
use driver::stdlib::Stdlib;
use errors::Error;
use ir::location::{SourceId, Span, Spanned};
use pp::{MessageTheme, Pretty, Theme};
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString, Url,
};

/// diagnostics of the document `url` whose text is `content`
pub fn diagnostics(url: &Url, content: String) -> Vec<Diagnostic> {
    let mut position_memo = PositionMemo::new(content.clone());
    let source = SourceId::Url(url.to_string());
    match compile(&mut Stdlib::load(), source, &content, 0) {
        Ok(compiled) => compiled
            .outputs()
            .filter_map(|output| match output {
                Output::Defined(_) => None,
                Output::Value(value, span) => Some(info(&mut position_memo, span, &value)),
                Output::Type(ty, span) => Some(info(&mut position_memo, span, ty)),
                Output::Help(help) => Some(info(&mut position_memo, help.span(), help)),
            })
            .collect(),
        Err(errs) => errs
            .into_iter()
            .map(|err| error(&mut position_memo, url, &err))
            .collect(),
    }
}

/// information diagnostic: `doc` at `span`
fn info(position_memo: &mut PositionMemo, span: Span, doc: &impl Pretty) -> Diagnostic {
    Diagnostic {
        range: position_memo.range(&span),
        message: doc.make_string(&Theme::default()),
        severity: Some(DiagnosticSeverity::INFORMATION),
        source: Some(Backend::name().to_string()),
        ..Diagnostic::default()
    }
}

fn error(position_memo: &mut PositionMemo, url: &Url, err: &Error) -> Diagnostic {
    let theme = MessageTheme::default();
    let range = position_memo.range(&err.span());
    let message = err
        .text()
        .unwrap_or_else(|| err.header())
        .make_string(&theme);
    let related_information = err.note().map(|note| {
        vec![DiagnosticRelatedInformation {
            location: Location {
                uri: url.clone(),
                range,
            },
            message: note.make_string(&theme),
        }]
    });
    Diagnostic {
        range,
        message,
        related_information,
        severity: Some(DiagnosticSeverity::ERROR),
        code: Some(NumberOrString::Number(err.code())),
        source: Some(Backend::name().to_string()),
        ..Diagnostic::default()
    }
}
