use super::*;

/// parse type
/// ```ebfn
/// type_variable := IDENTIFIER
/// ```
pub fn ty_variable() -> impl Parser<'static, TokenStream, cst::ty::Variable, ExtraChumsky> {
    select! {ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} =>
            meta.clone().map(|_| cst::ty::VariableT::from(s.clone()))
    }
    .labelled("type variable")
}

/// parse type builtin
pub fn ty_builtin() -> impl Parser<'static, TokenStream, cst::ty::Builtin, ExtraChumsky> {
    select! {
        ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} if s == "__Type_Nat__" =>
            meta.clone().map(|_| cst::ty::BuiltinT::Nat),

        ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} if s == "__Type_Bool__" =>
            meta.clone().map(|_| cst::ty::BuiltinT::Bool),

        ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} if s == "__Type_Char__" =>
            meta.clone().map(|_| cst::ty::BuiltinT::Char),
    }
    .labelled("builtin variable")
}

/// parse type
/// ```ebfn
/// type :=
/// | type_variable
/// ```
pub fn ty() -> impl Parser<'static, TokenStream, cst::Type, ExtraChumsky> {
    let builtin = ty_builtin().map(cst::Type::Builtin);
    let var = ty_variable().map(cst::Type::Variable);

    choice((builtin, var)).labelled("type")
}

/// parse type definition
/// ```ebfn
/// type_definition := type_variable EQ_DEF type
/// ```
pub fn type_definition() -> impl Parser<'static, TokenStream, cst::TypeDefinition, ExtraChumsky> {
    let eq_def = operator(Operator::EqDef, cst::operator::EqDefT()).labelled(":=");
    ty_variable()
        .then(eq_def)
        .then(ty())
        .map(|((name, eq_def), ty)| cst::TypeDefinition { name, eq_def, ty })
}
