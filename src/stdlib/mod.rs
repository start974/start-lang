/*pub mod number_n;*/
/*use super::parser::NameEnv;*/
/*use super::typing::TypingEnv;*/
/*use std::sync::LazyLock;*/

/*pub static NAME_ENV: LazyLock<NameEnv> = LazyLock::new(|| {*/
    /*// N idents*/
    /*NameEnv::empty()*/
        /*.add_ident(number_n::N_TYPE_NAME.clone())*/
        /*.unwrap()*/
/*});*/

/*pub static TYPE_ENV: LazyLock<TypingEnv> = LazyLock::new(|| {*/
    /*// N types*/
    /*let (env, res) = TypingEnv::empty()*/
        /*.add_alias(number_n::N_TYPE_NAME.clone(), None)*/
        /*.get_pair();*/
    /*let () = res.map_err(|err| panic!("{}", err)).unwrap();*/
    /*env*/
/*});*/
