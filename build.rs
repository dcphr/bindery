use std::path;
use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::File;

use std::process::Command;

fn write_version_file() {
    write_temp_file("git_commit_hash.txt", &version()[..]);
}

fn write_temp_file(name: &str, contents: &str) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("out_dir: {}", out_dir);
    let dest_path = Path::new(&out_dir).join(name);
    write_file(dest_path, contents);
}

fn write_file(destination: path::PathBuf, contents: &str) {
    let mut f = BufWriter::new(File::create(&destination).unwrap());
    write!(f, "{}", contents).unwrap();
}

fn version() -> String {
    let branch = Command::new("git").args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("failed to execute git branch detection");
    if !branch.status.success() { panic!("{}", String::from_utf8(branch.stderr).unwrap()) }

    let commit = Command::new("git").args(&["log", "-1", "--format=%h"])
        .output()
        .expect("failed to execute git commit detection");
    if !commit.status.success() { panic!("{}", String::from_utf8(commit.stderr).unwrap()) }

    let dirty_raw = Command::new("git").args(&["diff", "--shortstat"])
        .output()
        .expect("failed to execute git dirty detection.");
    if !dirty_raw.status.success() { panic!("{}", String::from_utf8(dirty_raw.stderr).unwrap()) }

    let dirty = match String::from_utf8_lossy(&dirty_raw.stdout).as_ref() {
        "" => "",
        _ => "+",
    };

    format!(
        "{} {}{}",
        String::from_utf8_lossy(&branch.stdout).trim(),
        String::from_utf8_lossy(&commit.stdout).trim(),
        dirty)
}

fn main() {
    write_version_file()
}
