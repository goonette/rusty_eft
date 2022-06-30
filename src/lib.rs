extern crate winapi;
extern crate clearscreen;

mod macros;
mod mem;
mod sdk;

use std::thread;

use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

fn main_thread() {
    clearscreen::clear().expect("Couldn't Clear"); // Clears console because I use battleyent.

    let mono = sdk::MonoLib::new(c_str!("mono-2.0-bdwgc.dll"));

    let domain = mono.get_domain();
    println!("Mono Domain = {:?}", domain);

    mono.attach_thread(domain);
    println!("Attached Thread :3");

    let game_object = mono.get_class(c_str!("UnityEngine.CoreModule"), c_str!("UnityEngine"), c_str!("GameObject"));
    println!("Game Object Class = {:?}", game_object);

    let find_game_object_ptr = mono.compile_method(mono.get_method(game_object, c_str!("Find"), 1));
    println!("GameObject.Find() Addr = {:?}", find_game_object_ptr);

    let find_game_object = make_fn!(find_game_object_ptr, *const usize, *const usize);

    let game_world = find_game_object(mono.new_string(c_str!("GameWorld")));
    println!("GameWorld = {:?}", game_world);

    let local_game_world = sdk::World::new(game_world, mono);

    let players = local_game_world.get_players();
    println!("Player Count = {:}", players.len());

    for p in players {
        let player = sdk::Player::new(p, mono);
        println!("Player Addr {:?}", player.get_addr());
    }
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