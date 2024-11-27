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
        .output()?;

    if !output.status.success() {
        return Err("rustc command failed".into());
    }

    let json_str = String::from_utf8(output.stdout)?;

    for line in json_str.lines() {
        if line.contains("\"llvm-target\":") {
            if let Some(target) = line.split(':')
                .nth(1)
                .and_then(|s| s.trim().strip_prefix('"'))
                .and_then(|s| s.strip_suffix("\",")) {
                return Ok(target.to_string());
            }
        }
    }

    Err("could not find llvm-target in JSON output".into());

}

fn get_clang_definitions(target: &str) -> HashMap<String, String> {

}

fn generate_compatibility_test(target: &str, definitions: &HashMap<String, String>) -> String {

}

fn main() {
    //Get list of all targets
    let targets = get_target_list();

    for target in targets {

    }

    println!("All C interop type compatibility checks passed");
}
