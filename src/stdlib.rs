pub mod number_n;
use super::parser::NameEnv;
use std::sync::LazyLock;

pub static NAME_ENV: LazyLock<NameEnv> = LazyLock::new(|| {
    let mut name_env = NameEnv::empty();
    for ident in number_n::N_IDENTS.iter() {
        name_env = name_env.add(ident.clone().clone()).unwrap();
    }
    name_env
});
