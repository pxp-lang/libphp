use std::{ffi::CString, ptr::{null_mut, null}};

use crate::{
    sys::{
        libphp_register_variable, php_embed_init, php_embed_shutdown, php_execute_simple_script,
        zend_eval_string_ex, zend_file_handle, zend_stream_init_filename, zval, libphp_register_constant, zend_fcall_info, zend_call_function, libphp_zval_create_string, zend_fcall_info_cache, zend_function_entry, zend_execute_data, zend_register_functions, zend_arg_info, zend_internal_arg_info, zend_type,
    },
    value::Value,
};

pub type OnInitCallback = fn(&mut Context);
pub type FunctionImplementation = unsafe extern "C" fn(*mut zend_execute_data, *mut zval);

#[derive(Default)]
pub struct Context {
    initd: bool,
    on_init: Option<OnInitCallback>,
    argc: i32,
    argv: Vec<String>,
    bindings: Vec<Value>,
}

impl Context {
    /// Create a new PHP execution context.
    pub fn new() -> Self {
        Self {
            initd: false,
            on_init: None,
            argc: 0,
            argv: Vec::new(),
            bindings: Vec::new(),
        }
    }

    /// Bind a variable to the PHP context.
    /// The variable will be available in the PHP context as a global variable.
    pub fn bind(&mut self, name: &str, value: impl Into<Value>) {
        let mut value = value.into();
        let var_name_cstr = CString::new(name).unwrap();

        unsafe {
            libphp_register_variable(var_name_cstr.as_ptr(), value.as_mut_ptr());
        }

        self.bindings.push(value);
    }

    /// Define a constant in the PHP context.
    /// The constant will be available in the PHP context as a global constant.
    pub fn define(&mut self, name: &str, value: impl Into<Value>) {
        let mut value = value.into();
        let constant_name_cstr = CString::new(name).unwrap();

        unsafe {
            libphp_register_constant(constant_name_cstr.as_ptr(), value.as_mut_ptr());
        }

        self.bindings.push(value);
    }

    /// Define a new function in the PHP context.
    pub fn define_function(&mut self, name: &str, function: FunctionImplementation) {
        let mut function_entry = zend_function_entry::default();   
        let function_name_cstr = CString::new(name).unwrap();

        let mut args: Vec<zend_internal_arg_info> = Vec::new();

        let mut arg = zend_internal_arg_info::default();
        arg.name = null();

        let arg_type = zend_type::default();
        arg.type_ = arg_type;

        args.push(arg);

        function_entry.fname = function_name_cstr.as_ptr();
        function_entry.num_args = 0;
        function_entry.handler = Some(function);
        function_entry.arg_info = Box::into_raw(args.into_boxed_slice()) as *const zend_internal_arg_info;
        
        let mut functions = Vec::new();
        functions.push(function_entry);
        
        unsafe {
            zend_register_functions(null_mut(), functions.as_mut_ptr(), null_mut(), 0);
        }
    }

    /// Specify the number of arguments to pass to the PHP context.
    pub fn argc(&mut self, argc: i32) {
        self.argc = argc;
    }

    /// Specify the arguments to pass to the PHP context.
    pub fn argv(&mut self, argv: Vec<String>) {
        self.argv = argv;
    }

    /// Execute a PHP file.
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

    /// Evaluate a PHP expression and get the result.
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

    /// Call a PHP function with no arguments.
    pub fn call(&mut self, name: &str) -> Value {
        let name_cstring = CString::new(name).unwrap();

        self.init();

        let mut retval_ptr = zval::default();
        
        let mut fcall = zend_fcall_info::default();
        let mut fcall_cache = zend_fcall_info_cache::default();

        unsafe { libphp_zval_create_string(&mut fcall.function_name, name_cstring.as_ptr()); }

        fcall.param_count = 0;
        fcall.object = null_mut();
        fcall.size = std::mem::size_of::<zend_fcall_info>();
        fcall.retval = &mut retval_ptr;

        unsafe {
            zend_call_function(&mut fcall, &mut fcall_cache);
        }

        return Value::new(&retval_ptr);
    }

    /// Call a PHP function with no arguments.
    pub fn call_with(&mut self, name: &str, args: &[impl Into<Value> + Clone]) -> Value {
        let name_cstring = CString::new(name).unwrap();

        self.init();

        // Convert the given arguments into a list of values.
        let mut args = args.iter().map(|arg| arg.clone().into()).collect::<Vec<Value>>();
        let mut retval_ptr = zval::default();
        let mut fcall = zend_fcall_info::default();
        let mut fcall_cache = zend_fcall_info_cache::default();

        unsafe { libphp_zval_create_string(&mut fcall.function_name, name_cstring.as_ptr()); }

        fcall.param_count = args.len() as u32;
        fcall.params = args.first_mut().unwrap().as_mut_ptr();
        fcall.object = null_mut();
        fcall.size = std::mem::size_of::<zend_fcall_info>();
        fcall.retval = &mut retval_ptr;

        unsafe {
            zend_call_function(&mut fcall, &mut fcall_cache);
        }

        return Value::new(&retval_ptr);
    }

    /// Register a callback to be called when the execution context is initialised.
    pub fn on_init(&mut self, callback: OnInitCallback) {
        self.on_init = Some(callback);
    }

    /// Initialise the execution context.
    ///
    /// NOTE: This method does not need to be called manually.
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

        if let Some(callback) = self.on_init {
            callback(self);
        }

        self.initd = true;
    }

    /// Close the execution context.
    ///
    /// NOTE: This method does not need to be called manually. The execution context is automatically closed when Context is dropped.
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
