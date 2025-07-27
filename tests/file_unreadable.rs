use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn exec_startlang<Args>(name: &str, f_args: Args)
where
    Args: Fn(&str) -> Vec<&str>,
{
    let path = std::env::temp_dir().join(name);
    let path_str = path.to_str().unwrap();

    let mut file = File::create(path.clone()).unwrap();
    writeln!(file, "Eval 42.").unwrap();

    let mut perms = fs::metadata(path.clone()).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(path.clone(), perms.clone()).unwrap();

    let output = Command::new("target/debug/startlang")
        .args(f_args(path_str))
        .output();

    perms.set_mode(0o644);
    fs::set_permissions(path.clone(), perms).unwrap();
    fs::remove_file(path.clone()).unwrap();

    let output = output.unwrap();
    assert_eq!(output.status.code().unwrap(), 101);
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!("[101] Error: Cannot read file \"{path_str}\".\n")
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
}

#[test]
fn test_format_file_without_read_permission() {
    exec_startlang("format_file_unreadable.st", |path| vec!["format", path]);
}

#[test]
fn test_run_file_without_read_permission() {
    exec_startlang("run_file_unreadable.st", |path| vec!["run", path]);
}
