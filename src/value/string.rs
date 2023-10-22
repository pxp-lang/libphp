use std::ffi::CString;

use crate::sys::{libphp_zval_create_string, zval};

use super::Value;

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        let mut zval = zval::default();
        let string = value.to_string();
        let cstr = CString::new(string).unwrap();

        unsafe {
            libphp_zval_create_string(&mut zval, cstr.as_ptr());
        }

        Self::new(&zval)
    }
}
