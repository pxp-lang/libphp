use std::{
    ffi::CStr,
    fmt::{Debug, Display},
};

use crate::sys::{
    libphp_var_export, libphp_zval_get_string, libphp_zval_get_type, zval, zval_ptr_dtor,
    IS_DOUBLE, IS_FALSE, IS_LONG, IS_NULL, IS_STRING, IS_TRUE,
};

mod string;

#[derive(Clone)]
pub struct Value {
    ptr: Box<zval>,
}

impl Value {
    /// Create a new Value from an existing zval.
    pub fn new(zval: &zval) -> Self {
        Self {
            ptr: Box::new(*zval),
        }
    }

    /// Get the type byte that represents the type of the value.
    pub fn get_type(&self) -> u8 {
        unsafe { libphp_zval_get_type(self.ptr.as_ref()) }
    }

    /// Check if the value is an integer (long).
    pub fn is_int(&self) -> bool {
        self.get_type() == IS_LONG
    }

    /// Check if the value is a float (double).
    pub fn is_float(&self) -> bool {
        self.get_type() == IS_DOUBLE
    }

    /// Check if the value is null.
    pub fn is_null(&self) -> bool {
        self.get_type() == IS_NULL
    }

    /// Check if the value is a string.
    pub fn is_string(&self) -> bool {
        self.get_type() == IS_STRING
    }

    /// Check if the value is true.
    pub fn is_true(&self) -> bool {
        self.get_type() == IS_TRUE
    }

    /// Check if the value is false.
    pub fn is_false(&self) -> bool {
        self.get_type() == IS_FALSE
    }

    /// Check if the value is a boolean.
    pub fn is_bool(&self) -> bool {
        self.is_true() || self.is_false()
    }

    /// Check a raw pointer to the underlying zval.
    pub fn as_ptr(&self) -> *const zval {
        self.ptr.as_ref()
    }

    /// Check a mutable raw pointer to the underlying zval.
    pub fn as_mut_ptr(&mut self) -> *mut zval {
        self.ptr.as_mut()
    }

    /// Convert the value to a string.
    ///
    /// WARNING: This method will panic if the PHP string is not valid UTF-8.
    pub fn as_str(&self) -> &str {
        unsafe {
            let cstr = CStr::from_ptr(libphp_zval_get_string(self.ptr.as_ref()));
            cstr.to_str().unwrap()
        }
    }

    /// Convert the value to a slice of bytes.
    ///
    /// WARNING: This method will panic if the PHP string is not valid UTF-8.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let cstr = CStr::from_ptr(libphp_zval_get_string(self.ptr.as_ref()));
            cstr.to_bytes()
        }
    }

    /// Convert the value to a C string (const char*).
    pub fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(libphp_zval_get_string(self.ptr.as_ref())) }
    }

    /// Convert the value to a 64-bit integer.
    pub fn to_int(&self) -> i64 {
        unsafe { self.ptr.value.lval }
    }

    /// Convert the value to a 64-bit floating point number.
    pub fn to_float(&self) -> f64 {
        unsafe { self.ptr.value.dval }
    }

    /// Convert the value to null (unit type).
    ///
    /// NOTE: This method only exists for consistency, there's no reason to use it.
    pub fn to_null(&self) {}

    /// Get a pretty name for the type of the value.
    pub fn get_type_name(&self) -> &'static str {
        match self.get_type() {
            IS_LONG => "int",
            IS_DOUBLE => "float",
            IS_NULL => "null",
            IS_STRING => "string",
            _ => "unknown",
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let var_exported = unsafe { libphp_var_export(self.ptr.as_ref()) };

        write!(f, "{}", unsafe {
            CStr::from_ptr(var_exported).to_string_lossy()
        })
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            zval_ptr_dtor(self.ptr.as_mut());
        }
    }
}
