#![forbid(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::{tempdir, TempDir};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();

        if ty.is_dir() {
            copy_dir_all(&path, dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(&path, dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn create_test_files(target_dir: &Path) -> Vec<PathBuf> {
    let test_file_content = "This file should not be affected without force flag";

    let docs_dir = target_dir.join("docs");
    fs::create_dir_all(&docs_dir).unwrap();
    let docs_file = docs_dir.join("this_name_wont_appear_in_build.txt");
    fs::write(&docs_file, test_file_content).unwrap();

    let code_dir = target_dir.join("code");
    fs::create_dir_all(&code_dir).unwrap();
    let code_file = code_dir.join("this_name_wont_appear_in_build.txt");
    fs::write(&code_file, test_file_content).unwrap();

    vec![docs_file, code_file]
}

fn check_files_exist(files: &[PathBuf], expected_content: &str) -> bool {
    for file in files {
        if !file.exists() {
            eprintln!("File does not exist: {:?}", file);
            return false;
        }

        match fs::read_to_string(file) {
            Ok(content) => {
                if content != expected_content {
                    eprintln!("File content mismatch for {:?}", file);
                    return false;
                }
            }
            Err(e) => {
                eprintln!("Failed to read file {:?}: {}", file, e);
                return false;
            }
        }
    }
    true
}

fn check_files_dont_exist(files: &[PathBuf]) -> bool {
    files.iter().all(|file| !file.exists())
}

fn run_lp_command(
    project_name: &str,
    use_force_flag: bool,
    tmp_root: &TempDir,
    test_with_messed_dirs: bool,
    with_plugins: bool,
) -> bool {
    let projects_dir = tmp_root.path().join("projects");
    fs::create_dir_all(&projects_dir).unwrap();

    let src_project_dir = Path::new("examples").join("projects").join(project_name);
    let tmp_project_dir = projects_dir.join(project_name);
    copy_dir_all(src_project_dir, &tmp_project_dir).unwrap();

    let plugins_dir = tmp_root.path().join("plugins");
    if with_plugins {
        copy_dir_all("examples/plugins", &plugins_dir).unwrap();
    }

    let target_dir = tmp_root.path().join("targets");
    fs::create_dir_all(&target_dir).unwrap();

    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("--src-dir")
        .arg(&tmp_project_dir)
        .arg("--plugins-dir")
        .arg(&plugins_dir)
        .arg("--target-dir")
        .arg(&target_dir);

    if use_force_flag {
        cmd.arg("-f");
    }

    let test_files: Vec<PathBuf>;
    let test_file_content = "This file should not be affected without force flag";

    if test_with_messed_dirs {
        test_files = create_test_files(&target_dir);
    } else {
        test_files = Vec::new();
    }

    let output = cmd.output().expect("Failed to execute cargo run");

    if !output.status.success() {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    if use_force_flag {
        if test_with_messed_dirs && !check_files_dont_exist(&test_files) {
            eprintln!("Test files were not deleted with force flag");
            return false;
        }
    } else {
        if test_with_messed_dirs && !check_files_exist(&test_files, test_file_content) {
            eprintln!("Test files were modified without force flag");
            return false;
        }
    }

    output.status.success()
}

#[test]
fn test_python_project_with_force() {
    let tmpdir = tempdir().unwrap();
    assert!(
        run_lp_command("python", true, &tmpdir, false, true),
        "Python project with force flag failed"
    );
}

#[test]
fn test_python_and_node_js_project_with_force() {
    let tmpdir = tempdir().unwrap();
    assert!(
        run_lp_command("python_and_node_js", true, &tmpdir, false, true),
        "Python and Node.js project with force flag failed"
    );
}

#[test]
fn test_python_project_without_force() {
    let tmpdir = tempdir().unwrap();
    assert!(
        run_lp_command("python", false, &tmpdir, false, true),
        "Python project without force flag failed"
    );
}

#[test]
fn test_python_and_node_js_project_without_force() {
    let tmpdir = tempdir().unwrap();
    assert!(
        run_lp_command("python_and_node_js", false, &tmpdir, false, true),
        "Python and Node.js project without force flag failed"
    );
}

#[test]
fn test_python_project_with_force_and_cleaning() {
    let tmpdir = tempdir().unwrap();
    assert!(
        run_lp_command("python", true, &tmpdir, true, true),
        "Python project with force flag failed"
    );
}

#[test]
fn test_python_and_node_js_project_with_force_and_cleaning() {
    let tmpdir = tempdir().unwrap();
    assert!(
        run_lp_command("python_and_node_js", true, &tmpdir, true, true),
        "Python and Node.js project with force flag failed"
    );
}

#[test]
fn test_python_project_without_force_and_with_extra_files() {
    let tmpdir = tempdir().unwrap();
    assert!(
        run_lp_command("python", false, &tmpdir, true, true),
        "Python project without force flag failed"
    );
}

#[test]
fn test_python_and_node_js_project_without_force_and_with_extra_files() {
    let tmpdir = tempdir().unwrap();
    assert!(
        run_lp_command("python_and_node_js", false, &tmpdir, true, true),
        "Python and Node.js project without force flag failed"
    );
}

#[test]
fn test_python_project_without_plugins() {
    let tmpdir = tempdir().unwrap();
    assert!(
        !run_lp_command("python", false, &tmpdir, false, false),
        "Python project without plugins has not failed, but it should have"
    );
}

#[test]
fn test_python_and_node_js_project_without_plugins() {
    let tmpdir = tempdir().unwrap();
    assert!(
        !run_lp_command("python_and_node_js", false, &tmpdir, false, false),
        "Python and Node.js project without plugins has not failed, but it should have"
    );
}
