//@ needs-force-clang-based-tests
// This test checks that the clang defines for each target allign with the core ffi types defined in
// mod.rs. Therefore each rust target is queried and the clang defines for each target are read and
// compared to the core sizes to verify all types and sizes allign at buildtime.
//
// If this test fails because Rust adds a target that Clang does not support, this target should be
// added to SKIPPED_TARGETS.

use run_make_support::{clang, regex, rfs, rustc, serde_json};
use serde_json::Value;

// It is not possible to run the Rust test-suite on these targets.
const SKIPPED_TARGETS: &[&str] = &[
    "xtensa-esp32-espidf",
    "xtensa-esp32-none-elf",
    "xtensa-esp32s2-espidf",
    "xtensa-esp32s2-none-elf",
    "xtensa-esp32s3-espidf",
    "xtensa-esp32s3-none-elf",
    "csky-unknown-linux-gnuabiv2",
    "csky-unknown-linux-gnuabiv2hf",
];

const MAPPED_TARGETS: &[(&str, &str)] = &[
    ("armv5te-unknown-linux-uclibcgnueabi", "armv5te-unknown-linux"),
    ("mips-unknown-linux-uclibc", "mips-unknown-linux"),
    ("mipsel-unknown-linux-uclibc", "mips-unknown-linux"),
    ("powerpc-unknown-linux-gnuspe", "powerpc-unknown-linux-gnu"),
    ("powerpc-unknown-linux-muslspe", "powerpc-unknown-linux-musl"),
    ("x86_64-unknown-l4re-uclibc", "x86_64-unknown-l4re"),
];

fn main() {
    let minicore_path = run_make_support::source_root().join("tests/auxiliary/minicore.rs");

    preprocess_core_ffi();

    let targets_json =
        rustc().arg("--print").arg("all-target-specs-json").arg("-Z").arg("unstable-options").run();
    let targets_json_str =
        String::from_utf8(targets_json.stdout().to_vec()).expect("error not a string");

    let j: Value = serde_json::from_str(&targets_json_str).unwrap();
    for (target, v) in j.as_object().unwrap() {
        let llvm_target = &v["llvm-target"].as_str().unwrap();

        if SKIPPED_TARGETS.iter().any(|&to_skip_target| target == to_skip_target) {
            continue;
        }

        // Create a new variable to hold either the mapped target or original llvm_target
        let target_to_use = MAPPED_TARGETS
            .iter()
            .find(|&&(from, _)| from == *llvm_target)
            .map(|&(_, to)| to)
            .unwrap_or(llvm_target);

        // Run Clang's preprocessor for the relevant target, printing default macro definitions.
        let clang_output =
            clang().args(&["-E", "-dM", "-x", "c", "/dev/null", "-target", &target_to_use]).run();

        let defines = String::from_utf8(clang_output.stdout()).expect("Invalid UTF-8");

        let minicore_content = rfs::read_to_string(&minicore_path);
        let mut rmake_content = format!(
            r#"
            #![no_std]
            #![no_core]
            #![feature(link_cfg)]
            #![allow(unused)]
            #![crate_type = "rlib"]

            /* begin minicore content */
            {minicore_content}
            /* end minicore content */

            #[path = "processed_mod.rs"]
            mod ffi;
            #[path = "tests.rs"]
            mod tests;
            "#
        );

        rmake_content.push_str(&format!(
            "
            const CLANG_C_CHAR_SIZE: usize = {};
            const CLANG_C_CHAR_SIGNED: bool = {};
            const CLANG_C_SHORT_SIZE: usize = {};
            const CLANG_C_INT_SIZE: usize = {};
            const CLANG_C_LONG_SIZE: usize = {};
            const CLANG_C_LONGLONG_SIZE: usize = {};
            const CLANG_C_FLOAT_SIZE: usize = {};
            const CLANG_C_DOUBLE_SIZE: usize = {};
            const CLANG_C_SIZE_T_SIZE: usize = {};
            const CLANG_C_PTRDIFF_T_SIZE: usize = {};
            ",
            parse_size(&defines, "CHAR"),
            char_is_signed(&defines),
            parse_size(&defines, "SHORT"),
            parse_size(&defines, "INT"),
            parse_size(&defines, "LONG"),
            parse_size(&defines, "LONG_LONG"),
            parse_size(&defines, "FLOAT"),
            parse_size(&defines, "DOUBLE"),
            parse_size(&defines, "SIZE_T"),
            parse_size(&defines, "PTRDIFF_T"),
        ));

        // Generate a target-specific rmake file.
        // If type misalignments occur,
        // generated rmake file name used to identify the failing target.
        let file_name = format!("{}_rmake.rs", target.replace("-", "_").replace(".", "_"));

        // Attempt to build the test file for the relevant target.
        // Tests use constant evaluation, so running is not necessary.
        rfs::write(&file_name, rmake_content);
        let rustc_output = rustc()
            .arg("-Zunstable-options")
            .arg("--emit=metadata")
            .arg("--target")
            .arg(target)
            .arg("-o-")
            .arg(&file_name)
            .run();
        rfs::remove_file(&file_name);
        if !rustc_output.status().success() {
            panic!("Failed for target {}", target);
        }
    }

    // Cleanup
    rfs::remove_file("processed_mod.rs");
}

// Helper to parse size from clang defines
fn parse_size(defines: &str, type_name: &str) -> usize {
    let search_pattern = format!("__SIZEOF_{}__ ", type_name.to_uppercase());
    for line in defines.lines() {
        if line.contains(&search_pattern) {
            if let Some(size_str) = line.split_whitespace().last() {
                return size_str.parse().unwrap_or(0);
            }
        }
    }

    // Only allow CHAR to default to 1
    if type_name.to_uppercase() == "CHAR" {
        return 1;
    }

    panic!("Could not find size definition for type: {}", type_name);
}

fn char_is_signed(defines: &str) -> bool {
    !defines.lines().any(|line| line.contains("__CHAR_UNSIGNED__"))
}

/// Parse core/ffi/mod.rs to retrieve only necessary macros and type defines
fn preprocess_core_ffi() {
    let mod_path = run_make_support::source_root().join("library/core/src/ffi/mod.rs");
    let mut content = rfs::read_to_string(&mod_path);

    //remove stability features #![unstable]
    let mut re = regex::Regex::new(r"#!?\[(un)?stable[^]]*?\]").unwrap();
    content = re.replace_all(&content, "").to_string();

    //remove doc features #[doc...]
    re = regex::Regex::new(r"#\[doc[^]]*?\]").unwrap();
    content = re.replace_all(&content, "").to_string();

    //remove lang feature #[lang...]
    re = regex::Regex::new(r"#\[lang[^]]*?\]").unwrap();
    content = re.replace_all(&content, "").to_string();

    //remove non inline modules
    re = regex::Regex::new(r".*mod.*;").unwrap();
    content = re.replace_all(&content, "").to_string();

    //remove use
    re = regex::Regex::new(r".*use.*;").unwrap();
    content = re.replace_all(&content, "").to_string();

    //remove fn fmt {...}
    re = regex::Regex::new(r"(?s)fn fmt.*?\{.*?\}").unwrap();
    content = re.replace_all(&content, "").to_string();

    //rmv impl fmt {...}
    re = regex::Regex::new(r"(?s)impl fmt::Debug for.*?\{.*?\}").unwrap();
    content = re.replace_all(&content, "").to_string();

    let file_name = "processed_mod.rs";

    rfs::write(&file_name, content);
}
