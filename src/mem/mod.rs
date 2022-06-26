use winapi::shared::minwindef::{FARPROC, HMODULE};

pub fn get_module(module_name: &str) -> HMODULE {
    unsafe { winapi::um::libloaderapi::GetModuleHandleA(module_name.as_ptr() as *const i8) }
}

pub fn get_export(module: HMODULE, export_name: &str) -> FARPROC {
    unsafe { winapi::um::libloaderapi::GetProcAddress(module, export_name.as_ptr() as *const i8) }
}