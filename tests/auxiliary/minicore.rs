//! Auxiliary `minicore` prelude which stubs out `core` items for `no_core` tests that need to work
//! in cross-compilation scenarios where no `core` is available (that don't want nor need to
//! `-Zbuild-std`).
//!
//! # Important notes
//!
//! - `minicore` is **only** intended for `core` items, and the stubs should match the actual `core`
//!   items.
//! - Be careful of adding new features and things that are only available for a subset of targets.
//!
//! # References
//!
//! This is partially adapted from `rustc_codegen_cranelift`:
//! <https://github.com/rust-lang/rust/blob/c0b5cc9003f6464c11ae1c0662c6a7e06f6f5cab/compiler/rustc_codegen_cranelift/example/mini_core.rs>.
// ignore-tidy-linelength

#![feature(no_core, lang_items, rustc_attrs, decl_macro, naked_functions, f16, f128)]
#![allow(unused, improper_ctypes_definitions, internal_features)]
#![feature(asm_experimental_arch)]
#![feature(intrinsics)]
#![no_std]
#![no_core]

// `core` has some exotic `marker_impls!` macro for handling the with-generics cases, but for our
// purposes, just use a simple macro_rules macro.
macro_rules! impl_marker_trait {
    ($Trait:ident => [$( $ty:ident ),* $(,)?] ) => {
        $( impl $Trait for $ty {} )*
    }
}

#[lang = "sized"]
pub trait Sized {}

#[lang = "legacy_receiver"]
pub trait LegacyReceiver {}
impl<T: ?Sized> LegacyReceiver for &T {}
impl<T: ?Sized> LegacyReceiver for &mut T {}

#[lang = "copy"]
pub trait Copy: Sized {}

impl_marker_trait!(
    Copy => [
        bool, char,
        isize, i8, i16, i32, i64, i128,
        usize, u8, u16, u32, u64, u128,
        f16, f32, f64, f128,
    ]
);
impl<'a, T: ?Sized> Copy for &'a T {}
impl<T: ?Sized> Copy for *const T {}
impl<T: ?Sized> Copy for *mut T {}
impl<T: Copy, const N: usize> Copy for [T; N] {}

#[lang = "phantom_data"]
pub struct PhantomData<T: ?Sized>;
impl<T: ?Sized> Copy for PhantomData<T> {}

pub enum Option<T> {
    None,
    Some(T),
}
impl<T: Copy> Copy for Option<T> {}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
impl<T: Copy, E: Copy> Copy for Result<T, E> {}

#[lang = "manually_drop"]
#[repr(transparent)]
pub struct ManuallyDrop<T: ?Sized> {
    value: T,
}
impl<T: Copy + ?Sized> Copy for ManuallyDrop<T> {}

#[lang = "unsafe_cell"]
#[repr(transparent)]
pub struct UnsafeCell<T: ?Sized> {
    value: T,
}

#[rustc_builtin_macro]
pub macro asm("assembly template", $(operands,)* $(options($(option),*))?) {
    /* compiler built-in */
}
#[rustc_builtin_macro]
pub macro naked_asm("assembly template", $(operands,)* $(options($(option),*))?) {
    /* compiler built-in */
}
#[rustc_builtin_macro]
pub macro global_asm("assembly template", $(operands,)* $(options($(option),*))?) {
    /* compiler built-in */
}

#[rustc_builtin_macro]
#[macro_export]
macro_rules! concat {
    ($($e:expr),* $(,)?) => {
        /* compiler built-in */
    };
}

#[rustc_builtin_macro]
#[macro_export]
macro_rules! stringify {
    ($($t:tt)*) => {
        /* compiler built-in */
    };
}

#[macro_export]
macro_rules! partialEq {
    ($($ty:ty),* $(,)?) => {
        $(
            impl PartialEq for $ty {
                #[inline]
                fn eq(&self, other: &$ty) -> bool {
                    (*self) == (*other)
                }
                #[inline]
                fn ne(&self, other: &$ty) -> bool {
                    (*self) != (*other)
                }
            }
        )*
    }
}

partialEq!(
    bool, char, isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128, f16, f32, f64, f128
);

#[lang = "panic"]
//#[rustc_const_panic_str]
//#[inline(never)]
//#[cold]
//#[track_caller]
//#[rustc_nounwind]
const fn panic(expr: &'static str) -> ! {
    abort()
}

#[lang = "panic_fmt"]
const fn panic_fmt(fmt: &str) -> ! {
    abort()
}

#[macro_export]
macro_rules! panic {
    ($msg:expr) => {
        $crate::panic($msg)
    };
}

#[rustc_intrinsic]
#[rustc_intrinsic_const_stable_indirect]
#[rustc_intrinsic_must_be_overridden]
pub const fn size_of<T>() -> usize {
    loop {}
}

#[rustc_intrinsic]
#[rustc_intrinsic_must_be_overridden]
pub const fn abort() -> ! {
    loop {}
}

#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

//impl PartialEq for usize {
//    fn eq(&self, other: &usize) -> bool {
//        (*self) == (*other)
//    }
//}
//
//impl PartialEq for bool {
//    fn eq(&self, other: &bool) -> bool {
//        (*self) == (*other)
//    }
//}

#[lang = "not"]
pub trait Not {
    type Output;
    fn not(self) -> Self::Output;
}

impl Not for bool {
    type Output = bool;
    fn not(self) -> Self {
        !self
    }
}
