use assert_cmd::Command;
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::Output;

fn exec_startlang<Args, Assert>(name: &str, perm: u32, f_args: Args, assert: Assert)
where
    Args: Fn(&str) -> Vec<&str>,
    Assert: Fn(&str, Output),
{
    let path = std::env::temp_dir().join(name);
    let path_str = path.to_str().unwrap();

    let mut file = File::create(&path).unwrap();
    writeln!(file, "Eval 42.").unwrap();

    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(perm);
    fs::set_permissions(&path, perms.clone()).unwrap();

    let output = Command::cargo_bin("startlang")
        .unwrap()
        .args(f_args(path_str))
        .output();

    perms.set_mode(0o644);
    fs::set_permissions(&path, perms).unwrap();
    fs::remove_file(&path).unwrap();

    let output = output.unwrap();
    assert(path_str, output);
}

#[test]
fn test_format_file_without_read_permission() {
    exec_startlang(
        "format_file_bad_read.st",
        0o244,
        |path| vec!["format", path],
        |path, output| {
            assert_eq!(output.status.code().unwrap(), 101);
            assert_eq!(
                String::from_utf8_lossy(&output.stderr),
                format!("[101] Error: Cannot read file \"{path}\".\n")
            );
            assert_eq!(String::from_utf8_lossy(&output.stdout), "");
        },
    );
}
#[test]
fn test_format_file_without_write_permission() {
    exec_startlang(
        "format_file_bad_write.st",
        0o444,
        |path| vec!["format", path],
        |path, output| {
            assert_eq!(output.status.code().unwrap(), 102);
            assert_eq!(
                String::from_utf8_lossy(&output.stderr),
                format!("[102] Error: Cannot write file \"{path}\".\n")
            );
            assert_eq!(String::from_utf8_lossy(&output.stdout), "");
        },
    );
}

#[test]
fn test_run_file_without_read_permission() {
    exec_startlang(
        "run_file_bad_read.st",
        0o244,
        |path| vec!["run", path],
        |path, output| {
            assert_eq!(output.status.code().unwrap(), 101);
            assert_eq!(
                String::from_utf8_lossy(&output.stderr),
                format!("[101] Error: Cannot read file \"{path}\".\n")
            );
            assert_eq!(String::from_utf8_lossy(&output.stdout), "");
        },
    );
}
