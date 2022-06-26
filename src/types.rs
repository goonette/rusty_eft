use winapi::ctypes::{c_char, c_int, c_void};

pub type MonoImageLoadedFn = unsafe extern fn(name: *const c_char) -> *const c_void;
pub type MonoGetRootDomainFn = unsafe extern fn() -> *const c_void;
pub type MonoThreadAttachFn = unsafe extern fn(domain: *const c_void) -> *const c_void;
pub type MonoClassFromNameFn = unsafe extern fn(image: *const c_void, namespace: *const c_char, name: *const c_char) -> *const c_void;
pub type MonoClassGetFieldFromNameFn = unsafe extern fn(class: *const c_void, name: *const c_char) -> *const c_void;
pub type MonoFieldGetOffsetFn = unsafe extern fn(field: *const c_void) -> *const c_void;
pub type MonoClassGetMethodFromNameFn = unsafe extern fn(class: *const c_void, name: *const c_char, args: c_int) -> *const c_void;
pub type MonoCompileMethodFn = unsafe extern fn(method: *const c_void) -> *const c_void;