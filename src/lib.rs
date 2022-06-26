extern crate winapi;
extern crate clearscreen;
extern crate core;

mod macros;
mod types;
mod mem;
mod mono;

use std::thread;

use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

fn main_thread() {
    clearscreen::clear().expect("Couldn't Clear"); // Clears console because I use battleyent

    println!("hai :3");

    let mono = mono::Lib::new(c_str!("mono-2.0-bdwgc.dll"));

    let assembly_csharp = mono.get_image(c_str!("Assembly-CSharp"));
    println!("Assembly-CSharp = {:?}", assembly_csharp);

    let domain = mono.get_domain();
    println!("Mono Domain = {:?}", domain);

    let thread = mono.get_thread(domain);
    println!("Mono Thread = {:?}", thread);

    let player = mono.get_class(assembly_csharp, c_str!("EFT"), c_str!("Player"));
    println!("Player Class = {:?}", player);

    let physical = mono.get_field(player, c_str!("Physical"));
    println!("Physical = {:?}", physical);

    let physical_offset = mono.field_get_offset(physical);
    println!("Physical Offset = {:?}", physical_offset);
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