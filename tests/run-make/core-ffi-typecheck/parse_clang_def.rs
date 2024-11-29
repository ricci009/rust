use std::collections::HashMap;

/*
struct SystemProperties {
    char_bit: u8,
    pointer_size: u32,
    pointer_width: u32,
    //add other system wide properties as needed
}
//

impl SystemProperties {
    //constructor for system properties
    fn from_defines(output: &str) -> Self {
        let char_bit = output
            .lines()
            .find(|line| line.contains("__CHAR_BIT__"))
            .and_then(|line| line.split_whitespace().nth(2))
            .and_then(|value| value.parse().ok())
            .unwrap_or(8); //assumption can be made c standards require 8 bit char

        let pointer_size = output
            .lines()
            .find(|line| line.contains("__POINTER_SIZE__"))
            .and_then(|line| line.split_whitespace().nth(2))
            .and_then(|value| value.parse().ok());

        let pointer_width = output
            .line()
            .find(|line| line.contains("__POINTER_WIDTH__"))
            .and_then(|line| line.split_whitespace().nth(2))
            .and_then(|value| value.parse().ok());

        if pointer_width.is_none() { //FIXME: should be an assert I believe
            eprintln!("Warning: Could not determine pointer width from Clang output");
        }

        if pointer_size.is_none() {
            eprintln!("Warning: Could not determine pointer size from Clang output");
        }

        SystemProperties {
            char_bit, pointer_size, pointer_width,
        }
    }

    fn is_valid(&self) -> bool {
        self.pointer_size.is_some() && self.pointer_width.is_some()
    }
}
*/

#[derive(Debug)]
struct CTypeInfo {
    size_in_bytes: Option<u32>,
    width_in_bytes: Option<u32>,
    is_unsigned: Option<bool>,
}

impl CTypeInfo {
    fn new() -> self {
        Self {
            size_in_bytes: None,
            width_in_bytes: None,
            is_unsigned: None,
        }
    }
}

fn parse_defines(output: &str) -> (HashMap<String, CTypeInfo>) {
    let mut types = HashMap::new();


    //first pass
    for line in output.lines() {
        if !line.starts_with("#define ") {
           continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let define_name = parts[1];
        let value =  match parts[2].parse::<u32>() {
            Ok(v) => v,
            Err(_) => continue,
        };

        match define_name {
            // Size information
            s if s.starts_with("__SIZEOF_") => {
                let type_name = s.trim_start_matches("__SIZEOF_")
                    .trim_end_matches("__")
                    .to_lowercase();
                let type_info = types.entry(type_name).or_insert_with(CTypeInfo::new);
                type_info.size_in_bytes = Some(value);
            },
            // Width information
            w if w.ends_with("_WIDTH__") => {
                let type_name = w.trim_end_matches("_WIDTH__")
                    .trim_start_matches("__")
                    .to_lowercase();
                let type_info = types.entry(type_name).or_insert_with(CTypeInfo::new);
                type_info.width_in_bits = Some(value);
            },
            // Special case for char signedness
            "__CHAR_UNSIGNED__" => {
                let type_info = types.entry("char".to_string()).or_insert_with(CTypeInfo::new);
                type_info.is_unsigned = Some(true);
            },
            _ => {} //other cases, maybe
        }
        //hash map is now populated.
        //now need to extract all the necessary stuff
    }

    //second pass
    for line in output.lines() {
        if !line.starts_with("#define ") {
           continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let define_name = parts[1];


    } 
}
