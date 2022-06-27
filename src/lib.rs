extern crate winapi;
extern crate clearscreen;

mod macros;
mod mem;
mod mono;

use std::thread;

use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::ctypes::c_char;

fn main_thread() {
    clearscreen::clear().expect("Couldn't Clear"); // Clears console because I use battleyent

    let mono = mono::Lib::new(c_str!("mono-2.0-bdwgc.dll"));

    let domain = mono.get_domain();
    println!("Mono Domain = {:?}", domain);

    mono.attach_thread(domain);
    println!("Attached Thread :3");

    let assembly_csharp = mono.get_image(c_str!("Assembly-CSharp"));
    println!("Assembly-CSharp = {:?}", assembly_csharp);

    let core_module = mono.get_image(c_str!("UnityEngine.CoreModule"));
    println!("UnityEngine.CoreModule = {:?}", core_module);

    let game_object = mono.get_class(core_module, c_str!("UnityEngine"), c_str!("GameObject"));
    println!("Game Object Class = {:?}", game_object);

    let find_game_object_ptr = mono.compile_method(mono.get_method(game_object, c_str!("Find()"), 1));
    println!("GameObject.Find() Addr = {:?}", find_game_object_ptr);

    let find_game_object = make_fn!(find_game_object_ptr, *const usize, *const c_char);

    let game_world = find_game_object(c_str!("GameWorld").as_ptr() as *const i8); // Pretty sure I need to recreate unity strings for this.

    println!("GameWorld = {:?}", game_world);
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