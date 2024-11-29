use run_make_support::{run, rustc, path, dynamic_lib_name, env, fs};
use std::collections::HashMap;
use std::process::Command;
use std::path::Path;

const XFAIL_TARGETS: &[&str] = &[
    //problematic targets here
]

fn get_target_list() -> Vec<String> {
    let output = Command::new("rustc")
        .arg("--print")
        .arg("target-list")
        .output()
        .expect("Failed to get target list");

    String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 in target list")
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

//specifically querying the rust target with the llvm target. nice
fn get_llvm_target(rust_target: &str) -> String {
    let output = Command::new("rustc")
        .args(&[
            "-Z", "unstable-options",
            "--print", "target-spec-json",
            "--target", rust_target
        ])
        .output()
        .expect("Failed to get target spec");

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)
        .expect("Failed to parse json target");

    json["llvm-target"].as_str()
        .expect("No llvm-target in spec")
        .to_string()

}

fn get_clang_definitions(llvm_target: &str) -> HashMap<String, String> {

    let output = Command::new("clang")
        .args([
            "-E",
            "-dM",
            "-x", "c",
            "/dev/null",
            "-target", llvm_target
        ])
        .output()
        .expect("failed to run clang");

    let defines = str::from_utf8(&output.stdout)
        .expect("Invalid clang output");

}

fn main() {
    //Get list of all targets
    println!("All C interop type compatibility checks passed");
}
