use run_make_support::{rustc, clang, serde_json, rfs};
use std::collections::HashMap;

const C_TYPES: &[(&str, &str)] = &[
    ("c_char", "__CHAR_BIT__"),              // Character width
    ("char_signedness", "__CHAR_UNSIGNED__"),
    ("c_double", "__SIZEOF_DOUBLE__"),       // Double precision floating-point
    ("c_float", "__SIZEOF_FLOAT__"),         // Single precision floating-point
    ("c_int", "__SIZEOF_INT__"),             // Signed integer
    ("c_long", "__SIZEOF_LONG__"),           // Signed long integer
    ("c_longlong", "__SIZEOF_LONG_LONG__"),  // Signed long long integer
    ("c_schar", "__SIZEOF_CHAR__"),          // Signed char
    ("c_short", "__SIZEOF_SHORT__"),         // Signed short integer
    ("c_uchar", "__SIZEOF_CHAR__"),          // Unsigned char
    ("c_uint", "__SIZEOF_INT__"),            // Unsigned integer
    ("c_ulong", "__SIZEOF_LONG__"),          // Unsigned long integer
    ("c_ulonglong", "__SIZEOF_LONG_LONG__"), // Unsigned long long integer
    ("c_ushort", "__SIZEOF_SHORT__"),        // Unsigned short integer
    ("c_size_t", "__SIZEOF_SIZE_T__"),       // Size type
    ("c_ptrdiff_t", "__SIZEOF_PTRDIFF_T__"), // Pointer difference type];
];



// #define __CHAR_BIT__ 8 // c_char
// #define __CHAR_UNSIGNED__ 1 // char signedness

// #define __SIZEOF_DOUBLE__ 8 // c_double
// #define __SIZEOF_FLOAT__ 4 // c_float
// #define __SIZEOF_INT__ 4 // c_int
// #define __SIZEOF_LONG_DOUBLE__ 16 // c_longdouble
// #define __SIZEOF_LONG_LONG__ 8 // c_longlong
// #define __SIZEOF_LONG__ 4 // c_long
// #define __SIZEOF_POINTER__ 4 // *const c_void
// #define __SIZEOF_PTRDIFF_T__ 4 // c_ptrdiff_t
// #define __SIZEOF_SHORT__ 2 // c_short
// #define __SIZEOF_SIZE_T__ 4 // c_size_t


// const ADDITIONAL_CHECKS: &[(&str, &str)] = &[
//     ("bool", "__BOOL_WIDTH__"),
//     ("isize", "__INTPTR_WIDTH__"),
//     ("usize", "__UINTPTR_WIDTH__"),
//     ("c_int_width", "__INT_WIDTH__"),
//     ("c_long_width", "__LONG_WIDTH__"),
//     ("c_longlong_width", "__LLONG_WIDTH__"),
//     ("c_short_width", "__SHRT_WIDTH__"),
//     ("c_size_t_width", "__SIZE_WIDTH__"),
// ];

type ClangDefinitions = HashMap<String, String>;
type TargetDefinitions = HashMap<String, ClangDefinitions>;

fn get_target_list() -> String {

    let completed_process = rustc()
        .arg("--print")
        .arg("target-list")
        .run();

    String::from_utf8(completed_process.stdout())
        .expect("error not a string")
}

//specifically querying the rust target with the llvm target.
fn get_llvm_target(rust_target: &str) -> Option<String> {

    let completed_process = rustc()
        .args(&[
            "-Z", "unstable-options",
            "--print", "target-spec-json",
            "--target", rust_target
        ])
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
    let completed_process = clang()
        .args(&[
            "-E",
            "-dM",
            "-x", "c",
            "/dev/null",
            "-target", llvm_target
        ])
        .run();

    let output = completed_process.stdout_utf8();

    //extract only the relevant defines
    let mut definitions = HashMap::new();
    let relevant_defines: Vec<&str> = C_TYPES.iter().map(|(_, define)| *define).collect();

    for line in output.lines() {
        if let Some((key, value)) = line.strip_prefix("#define ").and_then(|l| {
            let mut parts = l.split_whitespace();
            Some((parts.next()?, parts.next()?))
        }) {
            if relevant_defines.contains(&key) {
                definitions.insert(key.to_string(), value.to_string());
            }
        }
    }

    definitions // clang defines | int vals
}

fn collect_clang_definitions(target: &HashMap<String, String>) -> TargetDefinitions { //rust target | llvm target
    let mut all_definitions = HashMap::new();

    for llvm_target in target.values() {
        if llvm_target != "xtensa-none-elf" {
            let clang_definitions = get_clang_definitions(llvm_target);
            all_definitions.insert(llvm_target.clone(), clang_definitions);
        }
    }

    all_definitions

}

fn generate_rust_test(definitions: HashMap<String, String>) -> String { //clang definition | int
    let mut test_code = String::new();
    test_code.push_str("use std::ffi::c_char;\n");
    test_code.push_str("fn main() {");

    if let Some(is_unsigned) = definitions.get("__CHAR_UNSIGNED__") {
        let char_type = if is_unsigned == "1" { "u8" } else { "i8" };
        test_code.push_str(&format!(
            "    const _CHAR: c_char = 0_{char_type};\n"
        ));
    }

    test_code.push_str("}\n");

    test_code
}

// DEBUGGING PURPOSES
// fn print_definitions(all_definitions: &TargetDefinitions) {
//    let mut file = File::create("debug_output.txt").expect("Unable to create file");
//    for (target, definitions) in all_definitions {
//        writeln!(file, "Target: {}", target).expect("Write failed");
//        for (define, val) in definitions {
//            writeln!(file, "  {} = {}", define, val).expect("Write failed");
//        }
//    }
// }

fn main() {
    let output = get_target_list();

    let targets: Vec<&str> = output
        .lines()
        .collect();

    let mut target_map = HashMap::new();

    for target in targets {
        if let Some(llvm_target) = get_llvm_target(target) {
            //store the Rust target as key and LLVM target as value
            target_map.insert(target.to_string(), llvm_target);
        }
    }

    let all_definitions = collect_clang_definitions(&target_map);

    //print_definitions(&all_definitions);

    //all_definitions Hash<TARGET, HASH<DEFINE, VAL>>

    rfs::create_file("test.rs");
    for defines in all_definitions.values() {
        let test = generate_rust_test(defines.clone());
        rfs::write("test.rs", test);
        rustc()
            .arg("-Z")
            .arg("no-codegen")
            .arg("test.rs")
            .run();
    }
}
