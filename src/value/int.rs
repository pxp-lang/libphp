use crate::sys::{zval, libphp_zval_create_long};

use super::Value;

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        let mut zval = zval::default();

        unsafe { libphp_zval_create_long(&mut zval, value); }

        Self::new(&zval)
    }
}