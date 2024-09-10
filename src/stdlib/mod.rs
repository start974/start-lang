pub mod number_n;
use super::parser::NameEnv;
use super::typing::TypingEnv;
use std::sync::LazyLock;

pub static NAME_ENV: LazyLock<NameEnv> = LazyLock::new(|| {
    let mut name_env = NameEnv::empty();
    // N idents
    name_env = name_env.add(number_n::N_TYPE_NAME.clone()).unwrap();
    name_env
});

pub static TYPE_ENV: LazyLock<TypingEnv> = LazyLock::new(|| {
    let mut type_env = TypingEnv::empty();
    // N types
    type_env = type_env.add_type(number_n::N_TYPE.clone());
    type_env
});
