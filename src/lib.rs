extern crate winapi;
extern crate clearscreen;

mod macros;
mod mem;
mod mono;

use std::thread;

use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

fn main_thread() {
    clearscreen::clear().expect("Couldn't Clear"); // Clears console because I use battleyent.

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

    let find_game_object_ptr = mono.compile_method(mono.get_method(game_object, c_str!("Find"), 1));
    println!("GameObject.Find() Addr = {:?}", find_game_object_ptr);

    let find_game_object = make_fn!(find_game_object_ptr, *const usize, *const usize);

    let game_world = find_game_object(mono.new_string(domain, c_str!("GameWorld")));
    println!("GameWorld = {:?}", game_world);

    let world = unsafe { *cast!(game_world as usize + 0x10, usize) as *const usize };
    let temp_0 = unsafe { *cast!(world as usize + 0x30, usize) as *const usize };
    let temp_1 = unsafe { *cast!(temp_0 as usize + 0x18, usize) as *const usize };

    let local_game_world = unsafe { *cast!(temp_1 as usize + 0x28, usize) as *const usize };
    println!("Local Game World = {:?}", local_game_world); // Make a struct for world, player etc.

    let game_world_class = mono.get_class(assembly_csharp, c_str!("EFT"), c_str!("GameWorld"));
    println!("Game World Class = {:?}", game_world_class);

    let registered_players_addr = mono.get_field(game_world_class, "RegisteredPlayers");
    let registered_players = unsafe { *cast!(local_game_world as usize + mono.field_get_offset(registered_players_addr) as usize, usize) as *const usize };
    let player_count = unsafe { *cast!(registered_players as usize + 0x18, i32) }; // Implement lists properly.
    println!("Player Count = {:?}", player_count);
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