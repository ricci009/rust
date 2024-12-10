pub fn create_type_sizes() -> std::collections::HashMap<String, String> {
    let mut sizes = std::collections::HashMap::new();
    sizes.insert("c_char".to_string(), std::mem::size_of::<core::ffi::c_char>().to_string());
    sizes.insert("c_int".to_string(), std::mem::size_of::<core::ffi::c_int>().to_string());
    sizes.insert("c_long".to_string(), std::mem::size_of::<core::ffi::c_long>().to_string());
    sizes.insert(
        "c_ptrdiff_t".to_string(),
        std::mem::size_of::<core::ffi::c_ptrdiff_t>().to_string(),
    );
    sizes.insert("c_size_t".to_string(), std::mem::size_of::<core::ffi::c_size_t>().to_string());
    sizes.insert("c_ssize_t".to_string(), std::mem::size_of::<core::ffi::c_ssize_t>().to_string());
    sizes.insert("c_uint".to_string(), std::mem::size_of::<core::ffi::c_uint>().to_string());
    sizes.insert("c_ulong".to_string(), std::mem::size_of::<core::ffi::c_ulong>().to_string());
    sizes
}
