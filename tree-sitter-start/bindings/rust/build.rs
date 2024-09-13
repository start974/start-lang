fn main() {
    let root_dir = std::path::Path::new(".");
    let common_dir = root_dir.join("common");
    let grammars_dir = root_dir.join("grammars");
    let start_dir = grammars_dir.join("start").join("src");
    let start_repl_dir = grammars_dir.join("start_repl").join("src");

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(&start_dir);

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    println!("cargo:rerun-if-changed={}", common_dir.to_str().unwrap());

    for dir in &[start_dir, start_repl_dir] {
        let parser_path = dir.join("parser.c");
        c_config.file(&parser_path);
        println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
    }

    c_config.compile("tree-sitter-start");
}
