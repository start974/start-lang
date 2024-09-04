use super::super::ast::{Ident, Ty};
use std::sync::LazyLock;

pub static N_TYPE_IDENT: LazyLock<Ident> = LazyLock::new(|| Ident::from("N"));

pub static N_TYPE: LazyLock<Ty> = LazyLock::new(|| Ty::make_var(N_TYPE_IDENT.clone()));

pub static N_IDENTS: LazyLock<Vec<&Ident>> = LazyLock::new(|| vec![&N_TYPE_IDENT]);
