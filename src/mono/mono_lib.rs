use winapi::shared::minwindef::HMODULE;
use winapi::ctypes::{c_char, c_int};

use ::{c_str, make_fn, mem};

pub struct MonoLib {
    mono_module: HMODULE,
}

impl MonoLib {
    pub fn new(module_name: &str) -> MonoLib {
        let mono_module = mem::get_module(module_name);
        MonoLib { mono_module }
    }

    pub fn get_image(&self, name: &str) -> *const usize {
        let function_ptr = mem::get_export(self.mono_module,c_str!("mono_image_loaded"));
        let function = make_fn!(function_ptr, *const usize, *const c_char );

        function(name.as_ptr() as *const i8) as *const usize
    }

    pub fn get_domain(&self) -> *const usize {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_get_root_domain"));
        let function = make_fn!(function_ptr, *const usize);

        function() as *const usize
    }

    pub fn attach_thread(&self, domain: *const usize) {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_thread_attach"));
        let function = make_fn!(function_ptr, (), *const usize);

        function(domain)
    }

    pub fn get_class(&self, image: *const usize, namespace: &str, name: &str) -> *const usize {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_class_from_name"));
        let function = make_fn!(function_ptr, *const usize, *const usize, *const c_char, *const c_char);

        function(image, namespace.as_ptr() as *const i8, name.as_ptr() as *const i8) as *const usize
    }

    pub fn get_field(&self, class: *const usize, name: &str) -> *const usize {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_class_get_field_from_name"));
        let function = make_fn!(function_ptr, *const usize, *const usize, *const c_char);

        function(class, name.as_ptr() as *const i8) as *const usize
    }

    pub fn field_get_offset(&self, field: *const usize) -> *const usize {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_field_get_offset"));
        let function = make_fn!(function_ptr, *const usize, *const usize);

        function(field) as *const usize
    }

    pub fn get_method(&self, class: *const usize, name: &str, args: i32) -> *const usize {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_class_get_method_from_name"));
        let function = make_fn!(function_ptr, *const usize, *const usize, *const c_char, c_int );

        function(class, name.as_ptr() as *const i8, args) as *const usize
    }

    pub fn compile_method(&self, method: *const usize) -> *const usize {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_compile_method"));
        let function = make_fn!(function_ptr, *const usize, *const usize);

        function(method) as *const usize
    }

    pub fn new_string(&self, domain: *const usize, text: &str) -> *const usize {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_string_new"));
        let function = make_fn!(function_ptr, *const usize, *const usize, *const c_char);

        function(domain, text.as_ptr() as *const i8) as *const usize
    }
}