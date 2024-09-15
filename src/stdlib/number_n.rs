use crate::ast::{Ident, Ty};
use std::sync::LazyLock;

pub static N_TYPE_NAME: LazyLock<Ident> = LazyLock::new(|| Ident::from("N"));

pub static N_TYPE: LazyLock<Ty> = LazyLock::new(|| Ty::make_var(N_TYPE_NAME.clone()));
