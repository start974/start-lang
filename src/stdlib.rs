pub mod number_n;
use super::parser::NameEnv;
use super::typing::TypingEnv;
use std::sync::LazyLock;

pub static NAME_ENV: LazyLock<NameEnv> = LazyLock::new(|| {
    let mut name_env = NameEnv::empty();
    for ident in number_n::N_IDENTS.iter() {
        name_env = name_env.add(ident.clone().clone()).unwrap();
    }
    name_env
});

pub static TYPE_ENV: LazyLock<TypingEnv> = LazyLock::new(|| {
    let mut type_env = TypingEnv::empty();
    for ty in number_n::N_TYPES.iter() {
        type_env = type_env.add_type(ty.clone().clone());
    }
    type_env
});
