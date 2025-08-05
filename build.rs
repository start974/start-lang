use glob::glob;
use std::{collections::HashMap, env, fs, path::Path};

fn build_cli_tests() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = format!("{out_dir}/generated_cli_tests.rs");

    let mut modules: HashMap<String, Vec<(String, String)>> = HashMap::new();

    let patterns = ["tests/cmd/**/*.trycmd", "tests/cmd/**/*.toml"];

    // Collecte des fichiers
    let files = patterns
        .iter()
        .flat_map(|pattern| glob(pattern).unwrap())
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    for path in files {
        let module_name = path
            .parent()
            .and_then(|p| p.file_name())
            .unwrap_or_else(|| Path::new("root").as_os_str())
            .to_string_lossy()
            .replace("-", "_");

        // Nom unique du test
        let test_name = path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .replace(".", "_")
            .replace("-", "_");

        let path_str = path.to_string_lossy().into_owned();

        // Ajoute le test dans le bon module
        modules
            .entry(module_name)
            .or_default()
            .push((test_name, path_str));
    }

    // Génération du code Rust
    let mut output = String::new();
    output.push_str("// AUTO-GENERATED FILE - DO NOT EDIT\n");

    for (module, tests) in modules {
        output.push_str(&format!("mod {module} {{\n"));
        for (test_name, path) in tests {
            output.push_str(&format!(
                r#"
    #[test]
    fn {test_name}() {{
        trycmd::TestCases::new()
            .case("{path}")
            .run();
    }}
"#
            ));
        }
        output.push_str("}\n");
    }

    fs::write(dest_path, output).unwrap();
}

fn main() {
    build_cli_tests();
}
