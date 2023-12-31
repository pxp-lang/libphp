use std::ffi::{c_char, c_void};

// Type flags.
pub const IS_NULL: u8 = 1;
pub const IS_FALSE: u8 = 2;
pub const IS_TRUE: u8 = 3;
pub const IS_LONG: u8 = 4;
pub const IS_DOUBLE: u8 = 5;
pub const IS_STRING: u8 = 6;
pub const IS_ARRAY: u8 = 7;

// Hash table flags.
pub const HASH_KEY_IS_STRING: i32 = 1;
pub const HASH_KEY_IS_LONG: i32 = 2;
pub const HASH_KEY_NON_EXISTENT: i32 = 3;

// Misc. constants.
pub const HT_MIN_SIZE: u32 = 8;

#[link(name = "wrapper")]
extern "C" {
    pub fn libphp_zval_get_type(zval: *const zval) -> u8;
    pub fn libphp_zval_get_string(zval: *const zval) -> *const c_char;
    pub fn libphp_var_export(zval: *const zval) -> *const c_char;

    pub fn libphp_zval_create_string(zval: *mut zval, string: *const c_char) -> *const c_void;
    pub fn libphp_zval_create_long(zval: *mut zval, long: i64) -> *const c_void;

    pub fn libphp_zend_string_init(str: *const c_char) -> *mut zend_string;

    pub fn libphp_register_variable(key: *const c_char, value: *mut zval) -> *const c_void;
    pub fn libphp_register_constant(name: *const c_char, value: *mut zval) -> *const c_void;
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
