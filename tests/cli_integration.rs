#![forbid(unsafe_code)]

use std::process::Command;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

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

fn run_lp_command(project_name: &str, use_force_flag: bool) -> bool {
    let tmp_root = tempdir().unwrap();

    let projects_dir = tmp_root.path().join("projects");
    fs::create_dir_all(&projects_dir).unwrap();
    
    let src_project_dir = Path::new("examples").join("projects").join(project_name);
    let tmp_project_dir = projects_dir.join(project_name);
    copy_dir_all(src_project_dir, &tmp_project_dir).unwrap();
    
    let plugins_dir = tmp_root.path().join("plugins");
    copy_dir_all("examples/plugins", &plugins_dir).unwrap();

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

    let output = cmd.output().expect("Failed to execute cargo run");

    if !output.status.success() {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    output.status.success()
}

#[test]
fn test_python_project_with_force() {
    assert!(run_lp_command("python", true), 
            "Python project with force flag failed");
}

#[test]
fn test_python_and_node_js_project_with_force() {
    assert!(run_lp_command("python_and_node_js", true), 
            "Python and Node.js project with force flag failed");
}

#[test]
fn test_python_project_without_force() {
    assert!(run_lp_command("python", false), 
            "Python project without force flag failed");
}

#[test]
fn test_python_and_node_js_project_without_force() {
    assert!(run_lp_command("python_and_node_js", false), 
            "Python and Node.js project without force flag failed");
}
