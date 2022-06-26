extern crate winapi;
extern crate clearscreen;

use std::thread;

use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

mod macros;
mod types;
mod mem;
mod mono;

fn main_thread() {
    clearscreen::clear().expect("Couldn't Clear"); // Clears Console because I use battleyent

    println!("hai :3");

    let mono = mono::Lib::new(c_str!("mono-2.0-bdwgc.dll"));

    let image = mono.get_mono_image(c_str!("Assembly-CSharp"));
    println!("Assembly-CSharp = {:?}", image);

    let domain = mono.get_mono_domain();
    println!("Mono Domain = {:?}", domain);

    let thread = mono.get_mono_thread(domain);
    println!("Mono Thread = {:?}", thread);

    let main_application = mono.get_mono_class(image, c_str!("EFT"), c_str!("MainApplication"));
    println!("Main Application = {:?}", main_application);
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(h_module: HINSTANCE, dw_reason: u32, _: LPVOID) -> BOOL {
    if dw_reason == DLL_PROCESS_ATTACH {
        unsafe {
            DisableThreadLibraryCalls(h_module);
        }
        thread::spawn(|| {
            main_thread();
        });
    }
    TRUE
}