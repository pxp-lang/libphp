use libphp::{exec::Context, sys::{zend_execute_data, zval}};

fn main() {
    let mut context = Context::new();

    context.on_init(|ctx| {
        ctx.define_function("hello_world", hello_world);
    });

    context.result_of("hello_world()");
}

unsafe extern "C" fn hello_world(execute_data: *mut zend_execute_data, retval: *mut zval) {
    println!("Hello, world!");
}