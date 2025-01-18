// tests.rs

use super::*; // `super` will include everything from `smallcore` once glued together

cfg_if! {
    if #[cfg(all(target_os = "windows", target_arch = "aarch64"))] {
        const XFAIL_C_LONG_SIZE: usize = 4;
        pub const TEST_C_LONG_SIZE: () = if size_of::<ffi::c_long>() != XFAIL_C_LONG_SIZE {
            panic!("wrong c_long size test windows aarch64");
        };
    }
    else if #[cfg(all(target_arch = "aarch64", target_abi = "ilp32"))] {
        const XFAIL_C_LONG_SIZE: usize = 4;
        pub const TEST_C_LONG_SIZE: () = if size_of::<ffi::c_long>() != XFAIL_C_LONG_SIZE {
            panic!("wrong c_long size test ilp32");
        };
    }
    else if #[cfg(all(target_pointer_width = "32", target_os = "watchos"))] {
        const XFAIL_C_LONG_SIZE: usize = 4;
        pub const TEST_C_LONG_SIZE: () = if size_of::<ffi::c_long>() != XFAIL_C_LONG_SIZE {
            panic!("wrong c_long size test watchos");
        };
    }
    else if #[cfg(any(
        target_arch = "arm",
        target_arch = "csky",
        target_arch = "hexagon",
        target_arch = "x86",
        target_arch = "m68k",
        target_arch = "mips",
        target_arch = "mips32r6",
        target_arch = "powerpc",
        target_arch = "sparc"
    ))] {
        const XFAIL_C_LONG_SIZE: usize = 4;
        pub const TEST_C_LONG_SIZE: () = if size_of::<ffi::c_long>() != XFAIL_C_LONG_SIZE {
            panic!("wrong c_long size test for 32-bit architecture");
        };
    }
    else if #[cfg(any(
        target_arch = "avr",
        target_arch = "msp430"
    ))] {
        const XFAIL_C_LONG_SIZE: usize = 4;
        pub const TEST_C_LONG_SIZE: () = if size_of::<ffi::c_long>() != XFAIL_C_LONG_SIZE {
            panic!("wrong c_long size test for embedded architecture");
        };
    }
    else if #[cfg(all(target_arch = "x86_64", target_abi="x32"))] {
        const XFAIL_C_LONG_SIZE: usize = 4;
        pub const TEST_C_LONG_SIZE: () = if size_of::<ffi::c_long>() != XFAIL_C_LONG_SIZE {
            panic!("wrong c_long size test x86_64 x32 ABI");
        };
    }
    else {
        // Default test
        pub const TEST_C_LONG_SIZE: () = if size_of::<ffi::c_long>() != CLANG_C_LONG_SIZE {
            panic!("wrong c_long size");
        };
    }
}

cfg_if! {
    if #[cfg(target_arch = "csky")] {
        const XFAIL_C_CHAR_SIGNED: bool = false;  // Change to true for darwin
        pub const TEST_C_CHAR_UNSIGNED: () = if ffi::c_char::SIGNED ^ XFAIL_C_CHAR_SIGNED {
            panic!("mismatched c_char signed, target_arch: csky");
        };
    }
    else if #[cfg(target_arch = "msp430")] {
        const XFAIL_C_CHAR_SIGNED: bool = false;  // Change to true for darwin
        pub const TEST_C_CHAR_UNSIGNED: () = if ffi::c_char::SIGNED ^ XFAIL_C_CHAR_SIGNED {
            panic!("mismatched c_char signed, target_arch: msp430");
        };
    }
    else {
        pub const TEST_C_CHAR_UNSIGNED: () = if ffi::c_char::SIGNED ^ CLANG_C_CHAR_SIGNED {
            panic!("mismatched c_char sign");
        };
    }
}

cfg_if! {
    if #[cfg(target_arch = "avr")] {
        const XFAIL_C_INT_SIZE: usize = 2;
         pub const TEST_C_INT_SIZE: () = if size_of::<ffi::c_int>() != XFAIL_C_INT_SIZE {
            panic!("mismatched c_int size, target_arch: avr");
        };
    }
    else if #[cfg(target_arch = "msp430")] {
        const XFAIL_C_INT_SIZE: usize = 2;  // Change to true for darwin
         pub const TEST_C_INT_SIZE: () = if size_of::<ffi::c_int>() != XFAIL_C_INT_SIZE {
            panic!("mismatched c_int size, target_arch: msp430");
        };
    }
    else {
        pub const TEST_C_INT_SIZE: () = if size_of::<ffi::c_int>() != CLANG_C_INT_SIZE {
            panic!("wrong c_int size");
        };
    }
}

cfg_if! {
    if #[cfg(target_arch = "avr")] {
        const XFAIL_C_DOUBLE_SIZE: usize = 4;
         pub const TEST_C_DOUBLE_SIZE: () = if size_of::<ffi::c_double>() != XFAIL_C_DOUBLE_SIZE {
            panic!("wrong c_double size, target_arch: avr");
        };
    }
    else {
        pub const TEST_C_DOUBLE_SIZE: () = if size_of::<ffi::c_double>() != CLANG_C_DOUBLE_SIZE {
            panic!("wrong c_double size");
        };
    }
}

trait Signed {
    const SIGNED: bool;
}

impl Signed for i8 {
    const SIGNED: bool = true;
}

impl Signed for u8 {
    const SIGNED: bool = false;
}

//c_char size
pub const TEST_C_CHAR_SIZE: () = if size_of::<ffi::c_char>() != CLANG_C_CHAR_SIZE {
    panic!("wrong c_char size");
};

//c_short size
pub const TEST_C_SHORT_SIZE: () = if size_of::<ffi::c_short>() != CLANG_C_SHORT_SIZE {
    panic!("wrong c_short size");
};

//c_longlong size
pub const TEST_C_LONGLONG_SIZE: () = if size_of::<ffi::c_longlong>() != CLANG_C_LONGLONG_SIZE {
    panic!("wrong c_longlong size");
};

//c_float size
pub const TEST_C_FLOAT_SIZE: () = if size_of::<ffi::c_float>() != CLANG_C_FLOAT_SIZE {
    panic!("wrong c_float size");
};
