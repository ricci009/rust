use std::collections::HashMap;

include!("type_sizes.rs");

use run_make_support::{cargo, clang, rustc, serde_json};

type ClangDefinitions = HashMap<String, String>;
type TargetDefinitions = HashMap<String, ClangDefinitions>;

fn get_target_list() -> String {
    let completed_process = rustc().arg("--print").arg("target-list").run();

    String::from_utf8(completed_process.stdout()).expect("error not a string")
}

//specifically querying the rust target with the llvm target.
fn get_llvm_target(rust_target: &str) -> Option<String> {
    let completed_process = rustc()
        .args(&["-Z", "unstable-options", "--print", "target-spec-json", "--target", rust_target])
        .run();

    let json_output = match String::from_utf8(completed_process.stdout()) {
        Ok(output) => output,
        Err(_) => return None,
    };

    let vals: serde_json::Value = match serde_json::from_str(&json_output) {
        Ok(v) => v,
        Err(_) => return None,
    };

    vals["llvm-target"].as_str().map(|s| s.to_string())
}

fn get_clang_definitions(llvm_target: &str) -> HashMap<String, String> {
    let completed_process =
        clang().args(&["-E", "-dM", "-x", "c", "/dev/null", "-target", llvm_target]).run();

    let output = completed_process.stdout_utf8();

    //extract only the relevant defines
    let mut definitions = HashMap::new();

    for line in output.lines() {
        if let Some((key, value)) = line.strip_prefix("#define ").and_then(|l| {
            let mut parts = l.split_whitespace();
            Some((parts.next()?, parts.next()?))
        }) {
            definitions.insert(key.to_string(), value.to_string());
        }
    }

    definitions // clang defines | int vals
}

fn collect_clang_definitions(target: &HashMap<String, String>) -> TargetDefinitions {
    //rust target | llvm target
    let mut all_definitions = HashMap::new();

    for llvm_target in target.values() {
        if llvm_target != "xtensa-none-elf" {
            let clang_definitions = get_clang_definitions(llvm_target);
            all_definitions.insert(llvm_target.clone(), clang_definitions);
        }
    }

    all_definitions
}

//i need to rebuild every single time to get the new types.
fn get_core_ffi_types() -> HashMap<String, String> {
    create_type_sizes()
}
//c_char -> size
//c_int -> size
// c_long -> size
// c_ptrdiff_t
// c_size_t
// c_ssize_t
// c_uint
// c_ulong

// __CHAR_BIT__ = 8
// __CHAR_UNSIGNED__ = 1
// __SIZEOF_DOUBLE__ = 8
// __SIZEOF_INT__ = 4
// __SIZEOF_LONG__ = 8
// __SIZEOF_PTRDIFF_T__ = 8
// __SIZEOF_SIZE_T__ = 8
// __SIZEOF_FLOAT__ = 4
// __SIZEOF_LONG_LONG__ = 8
// __SIZEOF_SHORT__ = 2

//fn check_type_allignment(core_types: &HashMap<String, String>, llvm_target: String, llvm_target_defines: &HashMap<String, String>) {
//
//}

fn main() {
    let output = get_target_list();

    let targets: Vec<&str> = output.lines().collect();

    let mut target_map = HashMap::new();

    for target in targets {
        if let Some(llvm_target) = get_llvm_target(target) {
            //store the Rust target as key and LLVM target as value
            target_map.insert(target.to_string(), llvm_target);
        }
    }

    let core_ffi_types = get_core_ffi_types();
    println!("Core FFI types: {:?}", core_ffi_types);

    let all_definitions = collect_clang_definitions(&target_map);

    //now need to compare the types to the definition sizes.

    //print_definitions(&all_definitions);

    //all_definitions Hash<TARGET, HASH<DEFINE, VAL>>

    //do not think this is right.
    // rfs::create_file("test.rs");
    // for defines in all_definitions.values() {
    //     let test = generate_rust_test(defines.clone(), core_ffi_types);
    //     rfs::write("test.rs", test);
    //     rustc().arg("-Z").arg("no-codegen").arg("test.rs").run();
    // }
}

// //check char_bit
// // this can be checked, specifically c_char
// //llvm_target_defines.find(__CHAR_BIT__) != llvm_target_defines.end()
//     // then can compare to c_char and make sure sizes allign
//     // if sizes dont allign print("error sizes do not allign for {llvm_target})
// if let (Some(char_bit), Some(c_char)) = (
//     llvm_target_defines.get("__CHAR_BIT__").and_then(|v| v.parse::<usize>().ok()),
//     core_types.get("c_char").and_then(|v| v.parse::<usize>().ok())
// ) {
//     if char_bit != c_char {
//         println!("error: sizes do not align for {}: CHAR_BIT is {} but c_char is {} bits",
//                  llvm_target, char_bit, c_char);
//     }
// }
// //this needs to be worked on, cannot really do this.

//check unsigned
// im not too sure if this needs to be checked for core::ffi will look into this.
// check unsigned with system
/*
if let Some(is_unsigned) = definitions.get("__CHAR_UNSIGNED__") {
    let char_type = if is_unsigned == "1" { "u8" } else { "i8" };
    test_code.push_str(&format!("    const _CHAR: c_char = 0_{char_type};\n"));
}
*/
