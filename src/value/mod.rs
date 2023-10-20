use std::{ffi::CStr, fmt::{Display, Debug}};

use crate::sys::{zval, IS_LONG, libphp_zval_get_type, IS_DOUBLE, IS_NULL, IS_STRING, libphp_zval_get_string, IS_FALSE, IS_TRUE, zval_ptr_dtor, libphp_var_export};

#[derive(Clone)]
pub struct Value {
    ptr: Box<zval>,
}

impl Value {
    pub fn new(zval: &zval) -> Self {
        Self { ptr: Box::new(*zval) }
    }

    pub fn get_type(&self) -> u8 {
        unsafe { libphp_zval_get_type(self.ptr.as_ref()) }
    }

    pub fn is_int(&self) -> bool {
        self.get_type() == IS_LONG
    }

    pub fn is_float(&self) -> bool {
        self.get_type() == IS_DOUBLE
    }

    pub fn is_null(&self) -> bool {
        self.get_type() == IS_NULL
    }

    pub fn is_string(&self) -> bool {
        self.get_type() == IS_STRING
    }

    pub fn is_true(&self) -> bool {
        self.get_type() == IS_TRUE
    }

    pub fn is_false(&self) -> bool {
        self.get_type() == IS_FALSE
    }

    pub fn is_bool(&self) -> bool {
        self.is_true() || self.is_false()
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let cstr = CStr::from_ptr(libphp_zval_get_string(self.ptr.as_ref()));
            cstr.to_str().unwrap()
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let cstr = CStr::from_ptr(libphp_zval_get_string(self.ptr.as_ref()));
            cstr.to_bytes()
        }
    }

    pub fn as_cstr(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(libphp_zval_get_string(self.ptr.as_ref()))
        }
    }

    pub fn to_int(&self) -> i64 {
        unsafe { self.ptr.value.lval }
    }

    pub fn to_float(&self) -> f64 {
        unsafe { self.ptr.value.dval }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    pub fn to_null(&self) -> () {
        ()
    }

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

        write!(f, "{}", unsafe { CStr::from_ptr(var_exported).to_string_lossy() })
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            zval_ptr_dtor(self.ptr.as_mut());
        }
    }
}