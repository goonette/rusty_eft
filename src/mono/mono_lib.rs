use std::mem::transmute;

use winapi::ctypes::{c_char, c_void};
use winapi::shared::minwindef::HMODULE;

use ::{c_str, mem};

type MonoImageLoadedFn = unsafe extern fn(name: *const c_char) -> *const c_void;
type MonoGetRootDomainFn = unsafe extern fn() -> *const c_void;
type MonoThreadAttachFn = unsafe extern fn(domain: *const c_void) -> *const c_void;
type MonoClassFromNameFn = unsafe extern fn(image: *const c_void, namespace: *const c_char, name: *const c_char) -> *const c_void;

pub struct MonoLib {
    mono_module: HMODULE,
}

impl MonoLib {
    pub fn new(module_name: &str) -> MonoLib {
        let mono_module = mem::get_module(module_name);
        MonoLib { mono_module }
    }

    pub fn get_mono_image(&self, name: &str) -> *const c_void  {
        let function_ptr = mem::get_export(self.mono_module,c_str!("mono_image_loaded"));
        let function = unsafe { transmute::<_, MonoImageLoadedFn>(function_ptr) };

        unsafe { function(name.as_ptr() as *const i8) as *const c_void }
    }

    pub fn get_mono_domain(&self) -> *const c_void {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_get_root_domain"));
        let function = unsafe { transmute::<_, MonoGetRootDomainFn>(function_ptr) };

        unsafe { function() as *const c_void }
    }

    pub fn get_mono_thread(&self, domain: *const c_void) -> *const c_void {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_thread_attach"));
        let function = unsafe { transmute::<_, MonoThreadAttachFn>(function_ptr) };

        unsafe { function(domain) as *const c_void }
    }

    pub fn get_mono_class(&self, image: *const c_void, namespace: &str, name: &str) -> *const c_void {
        let function_ptr = mem::get_export(self.mono_module, c_str!("mono_class_from_name"));
        let function = unsafe { transmute::<_, MonoClassFromNameFn>(function_ptr) };

        unsafe { function(image, namespace.as_ptr() as *const i8, name.as_ptr() as *const i8) as *const c_void }
    }
}