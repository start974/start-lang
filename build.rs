use glob::glob;
use std::{env, fs};

fn build_cli_tests() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = format!("{}/generated_cli_tests.rs", out_dir);

    let mut tests = String::new();

    let patterns = ["tests/cmd/**/*.md", "tests/cmd/**/*.toml"];

    let files = patterns
        .iter()
        .flat_map(|pattern| glob(pattern).unwrap())
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    for path in files {
        let test_name = path
            .strip_prefix("tests")
            .unwrap_or(&path)
            .to_string_lossy()
            .replace("/", "_")
            .replace(".", "_")
            .replace("-", "_");
        let path_str = path.to_string_lossy();

        tests.push_str(&format!(
            r#"
                #[test]
                fn test_{}() {{
                    trycmd::TestCases::new()
                    .case("{}")
                    .run();
                }}
                "#,
            test_name, path_str
        ));
    }

    fs::write(dest_path, tests).unwrap();
}

fn main() {
    build_cli_tests();
}
