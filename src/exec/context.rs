use std::{ptr::null_mut, ffi::CString};

use crate::{sys::{php_embed_init, php_embed_shutdown, zval, zend_eval_string_ex, zend_file_handle, zend_stream_init_filename, php_execute_simple_script}, value::Value};

pub struct Context {
    initd: bool,
    argc: i32,
    argv: Vec<String>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            initd: false,
            argc: 0,
            argv: Vec::new(),
        }
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

            let mut retval_ptr = zval::default();

            php_execute_simple_script(&mut file_handle, &mut retval_ptr);

            Value::new(&retval_ptr)
        }
    }

    pub fn result_of(&mut self, expression: &str) -> Value {
        let code_cstring = CString::new(expression)
            .expect("Failed to convert the given code to a C string.");
        
        let script_name = CString::new("eval'd code").unwrap();
    
        self.init();

        unsafe {
            let mut retval_ptr = zval::default();
            
            zend_eval_string_ex(code_cstring.as_ptr(), &mut retval_ptr as *mut zval, script_name.as_ptr(), true);
            
            Value::new(&mut retval_ptr)
        }
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