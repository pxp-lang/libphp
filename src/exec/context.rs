use std::{ffi::CString, ptr::null_mut};

use crate::{
    sys::{
        libphp_register_variable, php_embed_init, php_embed_shutdown, php_execute_simple_script,
        zend_eval_string_ex, zend_file_handle, zend_stream_init_filename, zval,
    },
    value::Value,
};

#[derive(Default)]
pub struct Context {
    initd: bool,
    argc: i32,
    argv: Vec<String>,
    bindings: Vec<Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            initd: false,
            argc: 0,
            argv: Vec::new(),
            bindings: Vec::new(),
        }
    }

    pub fn bind(&mut self, name: &str, value: impl Into<Value>) {
        self.init();

        let mut value = value.into();
        let var_name_cstr = CString::new(name).unwrap();

        unsafe {
            libphp_register_variable(var_name_cstr.as_ptr(), value.as_mut_ptr());
        }

        self.bindings.push(value);
    }

    pub fn argc(&mut self, argc: i32) {
        self.argc = argc;
    }

    pub fn argv(&mut self, argv: Vec<String>) {
        self.argv = argv;
    }

    pub fn execute_file(&mut self, file: &str) -> Value {
        let mut file_handle = zend_file_handle::default();
        let cstring = CString::new(file).unwrap();

        self.init();

        unsafe {
            zend_stream_init_filename(&mut file_handle, cstring.as_ptr());
        }

        let mut retval_ptr = zval::default();

        unsafe {
            php_execute_simple_script(&mut file_handle, &mut retval_ptr);
        }

        self.bindings.clear();

        Value::new(&retval_ptr)
    }

    pub fn result_of(&mut self, expression: &str) -> Value {
        let code_cstring =
            CString::new(expression).expect("Failed to convert the given code to a C string.");

        let script_name = CString::new("eval'd code").unwrap();

        self.init();

        let mut retval_ptr = zval::default();

        unsafe {
            zend_eval_string_ex(
                code_cstring.as_ptr(),
                &mut retval_ptr as *mut zval,
                script_name.as_ptr(),
                true,
            );
        }

        self.bindings.clear();

        Value::new(&retval_ptr)
    }

    pub fn init(&mut self) {
        if self.initd {
            return;
        }

        unsafe {
            php_embed_init(
                self.argc,
                if self.argv.is_empty() {
                    null_mut()
                } else {
                    self.argv
                        .iter_mut()
                        .map(|arg| arg.as_ptr() as *mut i8)
                        .collect::<Vec<*mut i8>>()
                        .as_mut_ptr()
                },
            );
        }

        self.initd = true;
    }

    pub fn close(&self) {
        if self.initd {
            unsafe { php_embed_shutdown() };
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        self.close();
    }
}
