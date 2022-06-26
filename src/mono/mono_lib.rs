use std::mem::transmute;

use winapi::shared::minwindef::HMODULE;
use winapi::ctypes::{c_int, c_void};

use ::{c_str, mem};
use super::super::types::*;

pub struct MonoLib {
    mono_module: HMODULE,
}

impl MonoLib {
    pub fn new(module_name: &str) -> MonoLib {
        let mono_module = mem::get_module(module_name);
        MonoLib { mono_module }
    }

    pub fn get_image(&self, name: &str) -> *const c_void  {
        let function_ptr = mem::get_export(self.mono_module,c_str!("mono_image_loaded"));
        let function = unsafe { transmute::<_, MonoImageLoadedFn>(function_ptr) };

        unsafe { function(name.as_ptr() as *const i8) as *const c_void }
    }

    pub fn get_domain(&self) -> *const c_void {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_get_root_domain"));
        let function = unsafe { transmute::<_, MonoGetRootDomainFn>(function_ptr) };

        unsafe { function() as *const c_void }
    }

    pub fn get_thread(&self, domain: *const c_void) -> *const c_void {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_thread_attach"));
        let function = unsafe { transmute::<_, MonoThreadAttachFn>(function_ptr) };

        unsafe { function(domain) as *const c_void }
    }

    pub fn get_class(&self, image: *const c_void, namespace: &str, name: &str) -> *const c_void {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_class_from_name"));
        let function = unsafe { transmute::<_, MonoClassFromNameFn>(function_ptr) };

        unsafe { function(image, namespace.as_ptr() as *const i8, name.as_ptr() as *const i8) as *const c_void }
    }

    pub fn get_field(&self, class: *const c_void, name: &str) -> *const c_void {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_class_get_field_from_name"));
        let function = unsafe { transmute::<_, MonoClassGetFieldFromNameFn>(function_ptr) };

        unsafe { function(class, name.as_ptr() as *const i8) as *const c_void }
    }

    pub fn field_get_offset(&self, field: *const c_void) -> *const c_void {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_field_get_offset"));
        let function = unsafe { transmute::<_, MonoFieldGetOffsetFn>(function_ptr) };

        unsafe { function(field) as *const c_void }
    }

    pub fn get_method(&self, class: *const c_void, name: &str, args: c_int) -> *const c_void {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_class_get_method_from_name"));
        let function = unsafe { transmute::<_, MonoClassGetMethodFromNameFn>(function_ptr) };

        unsafe { function(class, name.as_ptr() as *const i8, args) as *const c_void }
    }

    pub fn compile_method(&self, method: *const c_void) -> *const c_void {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_compile_method"));
        let function = unsafe { transmute::<_, MonoCompileMethodFn>(function_ptr) };

        unsafe { function(method) as *const c_void }
    }
}