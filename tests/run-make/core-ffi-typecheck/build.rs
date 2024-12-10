use std::fs::File;
use std::io::Write;
use std::process::Command;

//acquires all c types using grep command
// TODO: add different system support (e.g. Windows)
fn get_c_types() -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("grep 'type c_[^ ]*' mod.rs -o | cut -d' ' -f2 | sort -u")
        .output()
        .expect("Failed, please run on UNIX system");

    String::from_utf8(output.stdout)
        .expect("please use unix system")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    let dest_path = "type_sizes.rs";
    println!("cargo:warning=Writing to: {}", dest_path);

    let mut f = File::create(dest_path).unwrap();

    let c_types = get_c_types();
    println!("cargo:warning=Found C types: {:?}", c_types);

    writeln!(f, "pub fn create_type_sizes() -> std::collections::HashMap<String, String> {{")
        .unwrap();
    writeln!(f, "    let mut sizes = std::collections::HashMap::new();").unwrap();

    for type_name in c_types {
        writeln!(
            f,
            "    sizes.insert(\"{0}\".to_string(), std::mem::size_of::<core::ffi::{0}>().to_string());",
            type_name
        ).unwrap();
    }

    writeln!(f, "    sizes").unwrap();
    writeln!(f, "}}").unwrap();

    println!("cargo:rerun-if-changed=mod.rs");
    println!("cargo:rerun-if-changed=build.rs");
}
